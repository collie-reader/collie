use core::fmt;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use chrono::{DateTime, FixedOffset, Utc};
use rusqlite::{Connection, Row};
use sea_query::{Expr, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

use super::database::Feeds;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum FeedStatus {
    Subscribed,
    Unsubscribed,
}

impl Display for FeedStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            FeedStatus::Subscribed => write!(f, "subscribed"),
            FeedStatus::Unsubscribed => write!(f, "unsubscribed"),
        }
    }
}

impl FromStr for FeedStatus {
    type Err = Error;

    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        match x {
            "subscribed" => Ok(Self::Subscribed),
            "unsubscribed" => Ok(Self::Unsubscribed),
            _ => Err(Error::InvalidEnumKey(
                x.to_string(),
                "FeedStatus".to_string(),
            )),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Feed {
    pub id: i32,
    pub title: String,
    pub link: String,
    pub status: FeedStatus,
    pub checked_at: DateTime<FixedOffset>,
}

impl From<&Row<'_>> for Feed {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get_unwrap("id"),
            title: row.get_unwrap("title"),
            link: row.get_unwrap("link"),
            status: FeedStatus::from_str(&row.get_unwrap::<&str, String>("status")).unwrap(),
            checked_at: row.get_unwrap("checked_at"),
        }
    }
}

#[derive(Deserialize)]
pub struct FeedToCreate {
    pub title: String,
    pub link: String,
}

#[derive(Deserialize)]
pub struct FeedToUpdate {
    pub id: i32,
    pub title: Option<String>,
    pub link: Option<String>,
    pub status: Option<FeedStatus>,
    pub checked_at: Option<DateTime<FixedOffset>>,
}

pub fn create(db: &Connection, arg: &FeedToCreate) -> Result<usize> {
    let (sql, values) = Query::insert()
        .into_table(Feeds::Table)
        .columns([Feeds::Title, Feeds::Link, Feeds::CheckedAt])
        .values_panic([(*arg.title).into(), (*arg.link).into(), Utc::now().into()])
        .build_rusqlite(SqliteQueryBuilder);

    Ok(db.execute(sql.as_str(), &*values.as_params())?)
}

pub fn read_all(db: &Connection) -> Result<Vec<Feed>> {
    let (sql, values) = Query::select()
        .columns([
            Feeds::Id,
            Feeds::Title,
            Feeds::Link,
            Feeds::Status,
            Feeds::CheckedAt,
        ])
        .from(Feeds::Table)
        .build_rusqlite(SqliteQueryBuilder);

    let mut stmt = db.prepare(sql.as_str())?;
    let rows = stmt.query_map(&*values.as_params(), |x| Ok(Feed::from(x)))?;

    Ok(rows.map(std::result::Result::unwrap).collect::<Vec<Feed>>())
}

pub fn read(db: &Connection, id: i32) -> Result<Option<Feed>> {
    let (sql, values) = Query::select()
        .columns([
            Feeds::Id,
            Feeds::Title,
            Feeds::Link,
            Feeds::Status,
            Feeds::CheckedAt,
        ])
        .from(Feeds::Table)
        .and_where(Expr::col(Feeds::Id).eq(id))
        .limit(1)
        .build_rusqlite(SqliteQueryBuilder);

    let mut stmt = db.prepare(sql.as_str())?;
    let mut rows = stmt.query(&*values.as_params())?;

    Ok(rows.next()?.map(Feed::from))
}

pub fn update(db: &Connection, arg: &FeedToUpdate) -> Result<usize> {
    let mut vals = vec![];

    if let Some(title) = &arg.title {
        vals.push((Feeds::Title, title.into()));
    }

    if let Some(link) = &arg.link {
        vals.push((Feeds::Link, link.into()));
    }

    if let Some(status) = &arg.status {
        vals.push((Feeds::Status, status.to_string().into()));
    }

    if let Some(checked_at) = arg.checked_at {
        vals.push((Feeds::CheckedAt, checked_at.into()));
    }

    let (sql, values) = Query::update()
        .table(Feeds::Table)
        .values(vals)
        .and_where(Expr::col(Feeds::Id).eq(arg.id))
        .build_rusqlite(SqliteQueryBuilder);

    Ok(db.execute(sql.as_str(), &*values.as_params())?)
}

pub fn delete(db: &Connection, id: i32) -> Result<usize> {
    let (sql, values) = Query::delete()
        .from_table(Feeds::Table)
        .and_where(Expr::col(Feeds::Id).eq(id))
        .build_rusqlite(SqliteQueryBuilder);

    Ok(db.execute(sql.as_str(), &*values.as_params())?)
}
