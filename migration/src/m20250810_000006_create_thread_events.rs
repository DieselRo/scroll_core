use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum ThreadEvents {
    Table,
    Id,
    ThreadId,
    EventType,
    Actor,
    Reason,
    CreatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ThreadEvents::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ThreadEvents::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ThreadEvents::ThreadId).string().not_null())
                    .col(ColumnDef::new(ThreadEvents::EventType).string().not_null())
                    .col(ColumnDef::new(ThreadEvents::Actor).string().not_null())
                    .col(ColumnDef::new(ThreadEvents::Reason).string())
                    .col(
                        ColumnDef::new(ThreadEvents::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_thread_events_thread")
                    .table(ThreadEvents::Table)
                    .col(ThreadEvents::ThreadId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ThreadEvents::Table).to_owned())
            .await
    }
}
