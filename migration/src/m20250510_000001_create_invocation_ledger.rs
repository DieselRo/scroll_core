use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum InvocationLedger {
    Table,
    Id,
    Phrase,
    Invoker,
    Invoked,
    Tier,
    Mode,
    ResonanceRequired,
    Timestamp,
    CostSystemPressure,
    CostTokenPressure,
    Decision,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(InvocationLedger::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(InvocationLedger::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(InvocationLedger::Phrase).text().not_null())
                    .col(
                        ColumnDef::new(InvocationLedger::Invoker)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InvocationLedger::Invoked)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(InvocationLedger::Tier).string().not_null())
                    .col(ColumnDef::new(InvocationLedger::Mode).string().not_null())
                    .col(
                        ColumnDef::new(InvocationLedger::ResonanceRequired)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InvocationLedger::Timestamp)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InvocationLedger::CostSystemPressure)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InvocationLedger::CostTokenPressure)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InvocationLedger::Decision)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(InvocationLedger::Table).to_owned())
            .await
    }
}
