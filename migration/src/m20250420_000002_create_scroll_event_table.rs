// migration/src/m20250420_000002_create_scroll_event_table.rs

use sea_orm_migration::prelude::*;

pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("scroll_events"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("session_id")).string().not_null())
                    .col(ColumnDef::new(Alias::new("author")).string().not_null())
                    .col(
                        ColumnDef::new(Alias::new("timestamp"))
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alias::new("content_json")).text().not_null())
                    .col(ColumnDef::new(Alias::new("actions_json")).text().not_null())
                    .col(ColumnDef::new(Alias::new("partial")).boolean().not_null())
                    .col(
                        ColumnDef::new(Alias::new("turn_complete"))
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Alias::new("interrupted"))
                            .boolean()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alias::new("branch")).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("scroll_events")).to_owned())
            .await
    }
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250420_000002_create_scroll_event_table"
    }
}
