use core::fmt::{self, Display, Formatter};
use std::str::FromStr;

use chrono::{DateTime, FixedOffset};
use rusqlite::{Result, Row};
use sea_query::{Expr, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use serde::{Deserialize, Serialize};
use sha1_smol::Sha1;

use super::database::{open_connection, Items};

#[derive(Serialize, Deserialize, Debug)]
pub enum ItemStatus {
    Unread,
    Read,
    Saved,
}

impl Display for ItemStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ItemStatus::Unread => write!(f, "unread"),
            ItemStatus::Read => write!(f, "read"),
            ItemStatus::Saved => write!(f, "saved"),
        }
    }
}

impl FromStr for ItemStatus {
    type Err = ();

    fn from_str(x: &str) -> Result<ItemStatus, Self::Err> {
        match x {
            "unread" => Ok(ItemStatus::Unread),
            "read" => Ok(ItemStatus::Read),
            "saved" => Ok(ItemStatus::Saved),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Item {
    id: i32,
    fingerprint: String,
    author: Option<String>,
    title: String,
    description: String,
    link: String,
    status: ItemStatus,
    published_at: DateTime<FixedOffset>,
    feed: i32,
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
            published_at: row.get_unwrap("published_at"),
            feed: row.get_unwrap("feed"),
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
        Sha1::from(format!(
            "{}:{}:{}",
            &self.title, &self.description, &self.link
        ))
        .hexdigest()
    }
}

#[derive(Deserialize)]
pub struct ItemToUpdate {
    id: i32,
    status: Option<ItemStatus>,
}

#[derive(Deserialize)]
pub struct ItemReadOption {
    pub feed: Option<i32>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

pub fn create(arg: &ItemToCreate) -> Result<usize> {
    let db = open_connection()?;

    let cols = [
        Items::Fingerprint,
        Items::Author,
        Items::Title,
        Items::Description,
        Items::Link,
        Items::Status,
        Items::PublishedAt,
        Items::Feed,
    ];

    let vals = [
        arg.fingerprint().into(),
        arg.author.clone().into(),
        arg.title.clone().into(),
        arg.description.clone().into(),
        arg.link.clone().into(),
        arg.status.to_string().into(),
        arg.published_at.into(),
        arg.feed.into(),
    ];

    let (sql, values) = Query::insert()
        .into_table(Items::Table)
        .columns(cols)
        .values_panic(vals)
        .build_rusqlite(SqliteQueryBuilder);

    db.execute(sql.as_str(), &*values.as_params())
}

pub fn read_all(opt: ItemReadOption) -> Result<Vec<Item>> {
    let db = open_connection()?;

    let cols = [
        Items::Fingerprint,
        Items::Author,
        Items::Title,
        Items::Description,
        Items::Link,
        Items::Status,
        Items::PublishedAt,
        Items::Feed,
    ];

    let mut query = Query::select().columns(cols).from(Items::Table).to_owned();

    if let Some(feed) = opt.feed {
        query.and_where(Expr::col(Items::Feed).eq(feed));
    }

    if let Some(limit) = opt.limit {
        query.limit(limit);
    }

    if let Some(offset) = opt.offset {
        query.offset(offset);
    }

    let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
    let mut stmt = db.prepare(sql.as_str())?;
    let rows = stmt.query_map(&*values.as_params(), |x| Ok(Item::from(x)))?;

    Ok(rows.map(|x| x.unwrap()).collect::<Vec<Item>>())
}

pub fn update(arg: ItemToUpdate) -> Result<usize> {
    let db = open_connection()?;

    let mut vals = vec![];
    if let Some(status) = arg.status {
        vals.push((Items::Status, status.to_string().into()));
    }

    let (sql, values) = Query::update()
        .table(Items::Table)
        .values(vals)
        .and_where(Expr::col(Items::Id).eq(arg.id))
        .build_rusqlite(SqliteQueryBuilder);

    db.execute(sql.as_str(), &*values.as_params())
}
