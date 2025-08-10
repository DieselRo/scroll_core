use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum OpenThreads {
    Table,
    Assignee,
    Priority,
    Tags,
    DueAt,
    Source,
    ReopenedCount,
    DedupeKey,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add new columns if they don't exist
        // SQLite lacks robust IF NOT EXISTS for columns, but migration runner ensures ordering.
        // SQLite doesn't support multiple alter options in one statement; do one at a time
        manager
            .alter_table(
                Table::alter()
                    .table(OpenThreads::Table)
                    .add_column_if_not_exists(ColumnDef::new(OpenThreads::Assignee).string())
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(OpenThreads::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(OpenThreads::Priority)
                            .string()
                            .not_null()
                            .default("MEDIUM"),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(OpenThreads::Table)
                    .add_column_if_not_exists(ColumnDef::new(OpenThreads::Tags).string())
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(OpenThreads::Table)
                    .add_column_if_not_exists(ColumnDef::new(OpenThreads::DueAt).timestamp())
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(OpenThreads::Table)
                    .add_column_if_not_exists(ColumnDef::new(OpenThreads::Source).string())
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(OpenThreads::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(OpenThreads::ReopenedCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(OpenThreads::Table)
                    .add_column_if_not_exists(ColumnDef::new(OpenThreads::DedupeKey).string())
                    .to_owned(),
            )
            .await?;

        // Index to speed dedupe lookup
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_open_threads_dedupe")
                    .table(OpenThreads::Table)
                    .col(OpenThreads::DedupeKey)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Down migration drops columns is non-trivial on SQLite; leave as no-op.
        let _ = manager; // silence unused
        Ok(())
    }
}
