use core::fmt::{self, Display, Formatter};
use std::str::FromStr;

use chrono::{DateTime, FixedOffset};
use rusqlite::{Connection, Row};
use sea_query::{Alias, Expr, Func, Order, Query, SqliteQueryBuilder, Values};
use sea_query_rusqlite::RusqliteBinder;
use serde::{Deserialize, Serialize};
use sha1_smol::Sha1;

use crate::error::{Error, Result};

use super::database::{Feeds, Items};

#[derive(Serialize, Deserialize, Debug)]
pub enum ItemStatus {
    Unread,
    Read,
}

impl Display for ItemStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Unread => write!(f, "unread"),
            Self::Read => write!(f, "read"),
        }
    }
}

impl FromStr for ItemStatus {
    type Err = Error;

    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        match x {
            "unread" => Ok(Self::Unread),
            "read" => Ok(Self::Read),
            _ => Err(Error::InvalidEnumKey(
                x.to_string(),
                "ItemStatus".to_string(),
            )),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ItemFeed {
    pub id: i32,
    pub title: String,
    pub link: String,
}

#[derive(Serialize, Debug)]
pub struct Item {
    pub id: i32,
    pub fingerprint: String,
    pub author: Option<String>,
    pub title: String,
    pub description: String,
    pub link: String,
    pub status: ItemStatus,
    pub is_saved: bool,
    pub published_at: DateTime<FixedOffset>,
    pub feed: ItemFeed,
}

impl From<&Row<'_>> for Item {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get_unwrap("id"),
            fingerprint: row.get_unwrap("fingerprint"),
            author: row.get_unwrap("author"),
            title: row.get_unwrap("title"),
            description: row.get_unwrap("description"),
            link: row.get_unwrap("link"),
            status: ItemStatus::from_str(&row.get_unwrap::<&str, String>("status")).unwrap(),
            is_saved: row.get_unwrap("is_saved"),
            published_at: row.get_unwrap("published_at"),
            feed: ItemFeed {
                id: row.get_unwrap("feed_id"),
                title: row.get_unwrap("feed_title"),
                link: row.get_unwrap("feed_link"),
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ItemToCreate {
    pub author: Option<String>,
    pub title: String,
    pub description: String,
    pub link: String,
    pub status: ItemStatus,
    pub published_at: DateTime<FixedOffset>,
    pub feed: i32,
}

impl ItemToCreate {
    pub fn fingerprint(&self) -> String {
        Sha1::from(format!("{}:{}", &self.title, &self.link)).hexdigest()
    }
}

#[derive(Deserialize)]
pub struct ItemToUpdate {
    id: i32,
    status: Option<ItemStatus>,
    is_saved: Option<bool>,
}

#[derive(Deserialize)]
pub struct ItemToUpdateAll {
    status: Option<ItemStatus>,
    is_saved: Option<bool>,
    opt: Option<ItemReadOption>,
}

#[derive(Deserialize)]
pub enum ItemOrder {
    ReceivedDateDesc,
    PublishedDateDesc,
    UnreadFirst,
}

#[derive(Deserialize)]
pub struct ItemReadOption {
    pub ids: Option<Vec<i32>>,
    pub feed: Option<i32>,
    pub status: Option<ItemStatus>,
    pub is_saved: Option<bool>,
    pub order_by: Option<ItemOrder>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

pub fn create(db: &Connection, arg: &ItemToCreate) -> Result<usize> {
    let (sql, values) = Query::insert()
        .into_table(Items::Table)
        .columns([
            Items::Fingerprint,
            Items::Author,
            Items::Title,
            Items::Description,
            Items::Link,
            Items::Status,
            Items::PublishedAt,
            Items::Feed,
        ])
        .values_panic([
            arg.fingerprint().into(),
            arg.author.clone().into(),
            arg.title.clone().into(),
            arg.description.clone().into(),
            arg.link.clone().into(),
            arg.status.to_string().into(),
            arg.published_at.into(),
            arg.feed.into(),
        ])
        .build_rusqlite(SqliteQueryBuilder);

    Ok(db.execute(sql.as_str(), &*values.as_params())?)
}

pub fn read_all(db: &Connection, opt: &ItemReadOption) -> Result<Vec<Item>> {
    let mut query = Query::select()
        .columns([
            (Items::Table, Items::Id),
            (Items::Table, Items::Fingerprint),
            (Items::Table, Items::Author),
            (Items::Table, Items::Title),
            (Items::Table, Items::Description),
            (Items::Table, Items::Link),
            (Items::Table, Items::Status),
            (Items::Table, Items::IsSaved),
            (Items::Table, Items::PublishedAt),
        ])
        .expr_as(Expr::col((Feeds::Table, Feeds::Id)), Alias::new("feed_id"))
        .expr_as(
            Expr::col((Feeds::Table, Feeds::Title)),
            Alias::new("feed_title"),
        )
        .expr_as(
            Expr::col((Feeds::Table, Feeds::Link)),
            Alias::new("feed_link"),
        )
        .from(Items::Table)
        .inner_join(
            Feeds::Table,
            Expr::col((Items::Table, Items::Feed)).equals((Feeds::Table, Feeds::Id)),
        )
        .clone();

    if let Some(ids) = &opt.ids {
        query.and_where(Expr::col(Items::Id).is_in(ids.clone()));
    }

    if let Some(feed) = &opt.feed {
        query.and_where(Expr::col(Items::Feed).eq(*feed));
    }

    if let Some(status) = &opt.status {
        query.and_where(Expr::col((Items::Table, Items::Status)).eq(status.to_string()));
    }

    if let Some(is_saved) = &opt.is_saved {
        query.and_where(Expr::col(Items::IsSaved).eq(*is_saved));
    }

    if let Some(order_by) = &opt.order_by {
        match order_by {
            ItemOrder::ReceivedDateDesc => {
                query
                    .order_by((Items::Table, Items::Id), Order::Desc)
                    .order_by(Items::PublishedAt, Order::Desc);
            }
            ItemOrder::PublishedDateDesc => {
                query.order_by(Items::PublishedAt, Order::Desc);
            }
            ItemOrder::UnreadFirst => {
                query
                    .order_by(
                        (Items::Table, Items::Status),
                        Order::Field(Values(vec![ItemStatus::Unread.to_string().into()])),
                    )
                    .order_by(Items::PublishedAt, Order::Desc);
            }
        }
    }

    if let Some(limit) = &opt.limit {
        query.limit(*limit);
    }

    if let Some(offset) = &opt.offset {
        query.offset(if let Some(limit) = &opt.limit {
            offset * limit
        } else {
            *offset
        });
    }

    let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
    let mut stmt = db.prepare(sql.as_str())?;
    let rows = stmt.query_map(&*values.as_params(), |x| Ok(Item::from(x)))?;

    Ok(rows.map(std::result::Result::unwrap).collect::<Vec<Item>>())
}

pub fn count_all(db: &Connection, opt: &ItemReadOption) -> Result<i64> {
    let mut query = Query::select()
        .from(Items::Table)
        .expr(Func::count(Expr::col(Items::Id)))
        .clone();

    if let Some(feed) = &opt.feed {
        query.and_where(Expr::col(Items::Feed).eq(*feed));
    }

    if let Some(status) = &opt.status {
        query.and_where(Expr::col((Items::Table, Items::Status)).eq(status.to_string()));
    }

    if let Some(is_saved) = &opt.is_saved {
        query.and_where(Expr::col(Items::IsSaved).eq(*is_saved));
    }

    let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
    let mut stmt = db.prepare(sql.as_str())?;
    let mut rows = stmt.query(&*values.as_params())?;

    Ok(if let Some(row) = rows.next()? {
        row.get_unwrap(0)
    } else {
        0
    })
}

pub fn update(db: &Connection, arg: &ItemToUpdate) -> Result<usize> {
    let mut vals = vec![];

    if let Some(status) = &arg.status {
        vals.push((Items::Status, status.to_string().into()));
    }

    if let Some(is_saved) = &arg.is_saved {
        vals.push((Items::IsSaved, (*is_saved).into()));
    }

    let (sql, values) = Query::update()
        .table(Items::Table)
        .values(vals)
        .and_where(Expr::col(Items::Id).eq(arg.id))
        .build_rusqlite(SqliteQueryBuilder);

    Ok(db.execute(sql.as_str(), &*values.as_params())?)
}

pub fn update_all(db: &Connection, arg: &ItemToUpdateAll) -> Result<usize> {
    let mut vals = vec![];

    if let Some(status) = &arg.status {
        vals.push((Items::Status, status.to_string().into()));
    }

    if let Some(is_saved) = &arg.is_saved {
        vals.push((Items::IsSaved, (*is_saved).into()));
    }

    let mut query = Query::update().table(Items::Table).values(vals).clone();

    if let Some(opt) = &arg.opt {
        if let Some(ids) = &opt.ids {
            query.and_where(Expr::col(Items::Id).is_in(ids.clone()));
        }

        if let Some(feed) = &opt.feed {
            query.and_where(Expr::col(Items::Feed).eq(*feed));
        }

        if let Some(status) = &opt.status {
            query.and_where(Expr::col((Items::Table, Items::Status)).eq(status.to_string()));
        }

        if let Some(is_saved) = &opt.is_saved {
            query.and_where(Expr::col(Items::IsSaved).eq(*is_saved));
        }
    }

    let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
    Ok(db.execute(sql.as_str(), &*values.as_params())?)
}
