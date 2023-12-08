use core::fmt;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use std::ops::Deref;

use rusqlite::{Connection, Row};
use sea_query::{Expr, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

use super::database::Settings;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum SettingKey {
    PollingFrequency,
    // seconds
    Notification,
    DbSchemeVersion,
    Theme,
    ItemsOrder,
    Proxy,
}

impl Display for SettingKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::PollingFrequency => write!(f, "polling_frequency"),
            Self::Notification => write!(f, "notification"),
            Self::DbSchemeVersion => write!(f, "db_scheme_version"),
            Self::Theme => write!(f, "theme"),
            Self::ItemsOrder => write!(f, "items_order"),
            Self::Proxy => write!(f, "proxy"),
        }
    }
}

impl FromStr for SettingKey {
    type Err = Error;

    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        match x {
            "polling_frequency" => Ok(Self::PollingFrequency),
            "notification" => Ok(Self::Notification),
            "db_scheme_version" => Ok(Self::DbSchemeVersion),
            "theme" => Ok(Self::Theme),
            "items_order" => Ok(Self::ItemsOrder),
            "proxy" => Ok(Self::Proxy),
            _ => Err(Error::InvalidEnumKey(
                x.to_string(),
                "SettingKey".to_string(),
            )),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Setting {
    pub key: SettingKey,
    pub value: String,
}

impl From<&Row<'_>> for Setting {
    fn from(row: &Row) -> Self {
        Self {
            key: SettingKey::from_str(&row.get_unwrap::<&str, String>("key")).unwrap(),
            value: row.get_unwrap("value"),
        }
    }
}

#[derive(Deserialize)]
pub struct SettingToUpdate {
    pub key: SettingKey,
    pub value: String,
}

pub fn read_all(db: &Connection) -> Result<Vec<Setting>> {
    let (sql, values) = Query::select()
        .columns([Settings::Key, Settings::Value])
        .from(Settings::Table)
        .build_rusqlite(SqliteQueryBuilder);

    let mut stmt = db.prepare(sql.as_str())?;
    let rows = stmt.query_map(&*values.as_params(), |x| Ok(Setting::from(x)))?;

    Ok(rows
        .map(std::result::Result::unwrap)
        .collect::<Vec<Setting>>())
}

pub fn read(db: &Connection, key: &SettingKey) -> Result<Setting> {
    let (sql, values) = Query::select()
        .columns([Settings::Key, Settings::Value])
        .from(Settings::Table)
        .and_where(Expr::col(Settings::Key).eq(key.to_string()))
        .limit(1)
        .build_rusqlite(SqliteQueryBuilder);

    let mut stmt = db.prepare(sql.as_str())?;
    let mut rows = stmt.query(&*values.as_params())?;
    match rows.next()? {
        None => {
            Err(Error::Unknown)
        }
        row => {
            Ok(row.map(Setting::from).unwrap())
        }
    }
    // Ok(rows.next()?.map(Setting::from).unwrap())
}

pub fn update(db: &Connection, arg: &SettingToUpdate) -> Result<usize> {
    match arg.key {
        SettingKey::PollingFrequency => {
            if arg.value.parse::<i32>().map(|x| x < 30).unwrap_or(false) {
                return Err(Error::Unknown);
            }
        }
        SettingKey::Notification => {
            if arg.value.parse::<bool>().unwrap_or(false) {
                return Err(Error::Unknown);
            }
        }
        SettingKey::Proxy => {
            if arg.value.parse::<String>().map(|x| reqwest::Proxy::http(x.deref())).map(|_| false).unwrap_or(true) {
                return Err(Error::Unknown);
            }
        }
        SettingKey::DbSchemeVersion => return Err(Error::Forbidden),
        _ => {}
    }

    let (sql, values) = Query::update()
        .table(Settings::Table)
        .values([(Settings::Value, arg.value.clone().into())])
        .and_where(Expr::col(Settings::Key).eq(arg.key.to_string()))
        .build_rusqlite(SqliteQueryBuilder);

    Ok(db.execute(sql.as_str(), &*values.as_params())?)
}
