use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum OpenThreads {
    Table,
    Id,
    ScrollPath,
    Title,
    Status,
    CreatedAt,
    UpdatedAt,
    LastEventId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OpenThreads::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OpenThreads::Id)
                            .string() // store UUID as TEXT for SQLite
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OpenThreads::ScrollPath).string().not_null())
                    .col(ColumnDef::new(OpenThreads::Title).string().not_null())
                    .col(ColumnDef::new(OpenThreads::Status).string().not_null())
                    .col(
                        ColumnDef::new(OpenThreads::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OpenThreads::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(OpenThreads::LastEventId).string())
                    .to_owned(),
            )
            .await?;

        // Index on status
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_open_threads_status")
                    .table(OpenThreads::Table)
                    .col(OpenThreads::Status)
                    .to_owned(),
            )
            .await?;

        // Index on scroll_path
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_open_threads_scroll_path")
                    .table(OpenThreads::Table)
                    .col(OpenThreads::ScrollPath)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OpenThreads::Table).to_owned())
            .await
    }
}

