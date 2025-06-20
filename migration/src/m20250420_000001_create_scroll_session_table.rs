// migration/src/m20250420_000001_create_scroll_session_table.rs

use sea_orm_migration::prelude::*;

pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Add your table creation logic here
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Add your table drop logic here
        Ok(())
    }
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250420_000001_create_scroll_session_table" // use actual filename
    }
}
