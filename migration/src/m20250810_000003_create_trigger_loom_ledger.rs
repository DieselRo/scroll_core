use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum TriggerTicks {
    Table,
    Id,
    TickNo,
    StartedAt,
    EmotionsJson,
    BudgetIn,
    BudgetOut,
}

#[derive(DeriveIden)]
enum TriggerDecisions {
    Table,
    Id,
    TickId,
    Construct,
    DecisionKind,
    EstCostTokens,
    BudgetRemaining,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TriggerTicks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TriggerTicks::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TriggerTicks::TickNo)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TriggerTicks::StartedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TriggerTicks::EmotionsJson).json())
                    .col(ColumnDef::new(TriggerTicks::BudgetIn).integer().not_null())
                    .col(ColumnDef::new(TriggerTicks::BudgetOut).integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TriggerDecisions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TriggerDecisions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TriggerDecisions::TickId).uuid().not_null())
                    .col(
                        ColumnDef::new(TriggerDecisions::Construct)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TriggerDecisions::DecisionKind)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TriggerDecisions::EstCostTokens)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TriggerDecisions::BudgetRemaining)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_trigger_decisions_tick")
                            .from(TriggerDecisions::Table, TriggerDecisions::TickId)
                            .to(TriggerTicks::Table, TriggerTicks::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TriggerDecisions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(TriggerTicks::Table).to_owned())
            .await
    }
}
