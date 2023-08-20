use chrono::{NaiveDateTime, Utc};
use rusqlite::{Result, Row};
use sea_query::{Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use serde::Deserialize;

use super::database::{open_connection, Feeds};

pub struct Feed {
    id: i32,
    title: String,
    link: String,
    checked_at: NaiveDateTime,
}

impl From<&Row<'_>> for Feed {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get_unwrap("id"),
            title: row.get_unwrap("title"),
            link: row.get_unwrap("link"),
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
}

pub fn create(args: FeedToCreate) -> Result<usize> {
    let db = open_connection()?;

    let (sql, values) = Query::insert()
        .into_table(Feeds::Table)
        .columns([Feeds::Title, Feeds::Link, Feeds::CheckedAt])
        .values_panic([args.title.into(), args.link.into(), Utc::now().into()])
        .build_rusqlite(SqliteQueryBuilder);

    db.execute(sql.as_str(), &*values.as_params())
}

pub fn read_all() {}

pub fn update() {}

pub fn delete() {}
