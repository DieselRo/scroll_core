use sea_orm_migration::prelude::*;
use serde_json::json;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("scroll_sessions"))
                    .add_column(
                        ColumnDef::new(Alias::new("state"))
                            .json()
                            .not_null()
                            .default(Expr::value(json!({}))),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("scroll_sessions"))
                    .drop_column(Alias::new("state"))
                    .to_owned(),
            )
            .await
    }
}
