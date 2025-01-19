use rusqlite::Connection;
use sea_query::{ColumnDef, Iden, Query, SqliteQueryBuilder, Table, TableStatement};
use sea_query_rusqlite::RusqliteBinder;

use crate::error::Result;

#[derive(Iden)]
pub enum Settings {
    Table,
    Key,
    Value,
}

pub fn settings_table() -> Vec<TableStatement> {
    let create_stmt = Table::create()
        .table(Settings::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(Settings::Key)
                .text()
                .not_null()
                .primary_key(),
        )
        .col(ColumnDef::new(Settings::Value).text().not_null())
        .to_owned();

    vec![TableStatement::Create(create_stmt)]
}

pub fn insert_default_settings(db: &Connection) -> Result<()> {
    let _ = insert_settings(db, "db_scheme_version", "1");
    let _ = insert_settings(db, "polling_frequency", "300");
    let _ = insert_settings(db, "notification", "1");
    let _ = insert_settings(db, "theme", "system");
    let _ = insert_settings(db, "items_order", "ReceivedDateDesc");
    let _ = insert_settings(db, "proxy", "");
    let _ = insert_settings(db, "fetch_old_items", "1");
    let _ = insert_settings(db, "upstream_url", "");

    Ok(())
}

fn insert_settings(db: &Connection, key: &str, value: &str) -> Result<usize> {
    let (insert_settings_sql, insert_settings_values) = Query::insert()
        .into_table(Settings::Table)
        .columns([Settings::Key, Settings::Value])
        .values([key.into(), value.into()])?
        .build_rusqlite(SqliteQueryBuilder);

    Ok(db.execute(
        insert_settings_sql.as_str(),
        &*insert_settings_values.as_params(),
    )?)
}
