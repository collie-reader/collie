use std::path::Path;

use rusqlite::{Connection, Result};
use sea_query::{
    ColumnDef, Expr, ForeignKey, ForeignKeyAction, Iden, Index, Query, SqliteQueryBuilder, Table,
};
use sea_query_rusqlite::RusqliteBinder;

#[derive(Iden)]
pub enum Feeds {
    Table,
    Id,
    Title,
    Link,
    Status,
    CheckedAt,
}

#[derive(Iden)]
pub enum Items {
    Table,
    Id,
    Fingerprint,
    Author,
    Title,
    Description,
    Link,
    Status,
    IsSaved,
    PublishedAt,
    Feed,
}

#[derive(Iden)]
pub enum Settings {
    Table,
    Key,
    Value,
}

pub fn open_connection(path: &Path) -> Result<Connection> {
    Connection::open(path.join("collie.db"))
}

pub fn migrate(db: &Connection) -> Result<()> {
    let create_table_feeds = Table::create()
        .table(Feeds::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(Feeds::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Feeds::Title).text().not_null())
        .col(ColumnDef::new(Feeds::Link).text().not_null())
        .col(
            ColumnDef::new(Feeds::Status)
                .text()
                .check(Expr::col(Feeds::Status).is_in(["subscribed", "unsubscribed"]))
                .not_null()
                .default("subscribed"),
        )
        .col(ColumnDef::new(Feeds::CheckedAt).date_time().not_null())
        .index(
            Index::create()
                .unique()
                .name("uk_feeds_title_link")
                .col(Feeds::Title)
                .col(Feeds::Link),
        )
        .build(SqliteQueryBuilder);

    let create_table_items = Table::create()
        .table(Items::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(Items::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(Items::Fingerprint)
                .text()
                .not_null()
                .unique_key(),
        )
        .col(ColumnDef::new(Items::Author).text())
        .col(ColumnDef::new(Items::Title).text().not_null())
        .col(ColumnDef::new(Items::Description).text().not_null())
        .col(ColumnDef::new(Items::Link).text().not_null())
        .col(
            ColumnDef::new(Items::Status)
                .text()
                .check(Expr::col(Items::Status).is_in(["unread", "read"]))
                .not_null()
                .default("unread"),
        )
        .col(
            ColumnDef::new(Items::IsSaved)
                .integer()
                .check(Expr::col(Items::IsSaved).is_in([0, 1]))
                .not_null()
                .default(0),
        )
        .col(ColumnDef::new(Items::PublishedAt).date_time().not_null())
        .col(ColumnDef::new(Items::Feed).integer().not_null())
        .foreign_key(
            ForeignKey::create()
                .name("fk_items_feeds")
                .from(Items::Table, Items::Feed)
                .to(Feeds::Table, Feeds::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .build(SqliteQueryBuilder);

    let create_table_settings = Table::create()
        .table(Settings::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(Settings::Key)
                .text()
                .check(Expr::col(Settings::Key).is_in(["polling_frequency"]))
                .not_null()
                .primary_key(),
        )
        .col(ColumnDef::new(Settings::Value).text().not_null())
        .build(SqliteQueryBuilder);

    db.execute_batch(
        &[
            create_table_feeds,
            create_table_items,
            create_table_settings,
        ]
        .join(";"),
    )?;

    let (insert_settings_sql, insert_settings_values) = Query::insert()
        .into_table(Settings::Table)
        .columns([Settings::Key, Settings::Value])
        .values_panic(["polling_frequency".into(), "120".into()])
        .build_rusqlite(SqliteQueryBuilder);

    db.execute(
        insert_settings_sql.as_str(),
        &*insert_settings_values.as_params(),
    )?;

    Ok(())
}
