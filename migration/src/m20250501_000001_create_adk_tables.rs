use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

/// Sessions table columns
#[derive(Iden)]
enum Sessions {
    Table,
    AppName,
    UserId,
    Id,
    State,
    CreateTime,
    UpdateTime,
    Status,
}

/// Events table columns
#[derive(Iden)]
enum Events {
    Table,
    Id,
    AppName,
    UserId,
    SessionId,
    InvocationId,
    Author,
    Branch,
    Timestamp,
    Content,
    Actions,
    Partial,
    TurnComplete,
    Interrupted,
    ErrorCode,
    ErrorMessage,
}

/// App state table columns
#[derive(Iden)]
enum AppStates {
    Table,
    AppName,
    State,
    UpdateTime,
}

/// User state table columns
#[derive(Iden)]
enum UserStates {
    Table,
    AppName,
    UserId,
    State,
    UpdateTime,
}

/// Artifacts table columns
#[derive(Iden)]
enum Artifacts {
    Table,
    AppName,
    UserId,
    SessionId,
    Filename,
    Version,
    MimeType,
    Data,
    CreateTime,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create sessions table
        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .col(ColumnDef::new(Sessions::AppName).string().not_null())
                    .col(ColumnDef::new(Sessions::UserId).string().not_null())
                    .col(ColumnDef::new(Sessions::Id).string().not_null())
                    .col(ColumnDef::new(Sessions::State).json().not_null())
                    .col(ColumnDef::new(Sessions::CreateTime).double().not_null())
                    .col(ColumnDef::new(Sessions::UpdateTime).double().not_null())
                    .col(ColumnDef::new(Sessions::Status).string().not_null())
                    .primary_key(Index::create().col(Sessions::AppName).col(Sessions::UserId).col(Sessions::Id))
                    .to_owned(),
            )
            .await?;

        // Create events table
        manager
            .create_table(
                Table::create()
                    .table(Events::Table)
                    .col(ColumnDef::new(Events::Id).string().not_null())
                    .col(ColumnDef::new(Events::AppName).string().not_null())
                    .col(ColumnDef::new(Events::UserId).string().not_null())
                    .col(ColumnDef::new(Events::SessionId).string().not_null())
                    .col(ColumnDef::new(Events::InvocationId).string().not_null())
                    .col(ColumnDef::new(Events::Author).string().not_null())
                    .col(ColumnDef::new(Events::Branch).string())
                    .col(ColumnDef::new(Events::Timestamp).double().not_null())
                    .col(ColumnDef::new(Events::Content).json())
                    .col(ColumnDef::new(Events::Actions).json())
                    .col(ColumnDef::new(Events::Partial).boolean())
                    .col(ColumnDef::new(Events::TurnComplete).boolean())
                    .col(ColumnDef::new(Events::Interrupted).boolean())
                    .col(ColumnDef::new(Events::ErrorCode).string())
                    .col(ColumnDef::new(Events::ErrorMessage).string())
                    .primary_key(Index::create().col(Events::Id))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_events_sessions")
                            .from(Events::Table, vec![Events::AppName, Events::UserId, Events::SessionId])
                            .to(Sessions::Table, vec![Sessions::AppName, Sessions::UserId, Sessions::Id])
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        // Create app states table
        manager
            .create_table(
                Table::create()
                    .table(AppStates::Table)
                    .col(ColumnDef::new(AppStates::AppName).string().not_null())
                    .col(ColumnDef::new(AppStates::State).json().not_null())
                    .col(ColumnDef::new(AppStates::UpdateTime).double().not_null())
                    .primary_key(Index::create().col(AppStates::AppName))
                    .to_owned(),
            )
            .await?;

        // Create user states table
        manager
            .create_table(
                Table::create()
                    .table(UserStates::Table)
                    .col(ColumnDef::new(UserStates::AppName).string().not_null())
                    .col(ColumnDef::new(UserStates::UserId).string().not_null())
                    .col(ColumnDef::new(UserStates::State).json().not_null())
                    .col(ColumnDef::new(UserStates::UpdateTime).double().not_null())
                    .primary_key(Index::create().col(UserStates::AppName).col(UserStates::UserId))
                    .to_owned(),
            )
            .await?;

        // Create artifacts table
        manager
            .create_table(
                Table::create()
                    .table(Artifacts::Table)
                    .col(ColumnDef::new(Artifacts::AppName).string().not_null())
                    .col(ColumnDef::new(Artifacts::UserId).string().not_null())
                    .col(ColumnDef::new(Artifacts::SessionId).string().not_null())
                    .col(ColumnDef::new(Artifacts::Filename).string().not_null())
                    .col(ColumnDef::new(Artifacts::Version).integer().not_null())
                    .col(ColumnDef::new(Artifacts::MimeType).string().not_null())
                    .col(ColumnDef::new(Artifacts::Data).binary().not_null())
                    .col(ColumnDef::new(Artifacts::CreateTime).double().not_null())
                    .primary_key(Index::create()
                        .col(Artifacts::AppName)
                        .col(Artifacts::UserId)
                        .col(Artifacts::SessionId)
                        .col(Artifacts::Filename)
                        .col(Artifacts::Version))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_artifacts_sessions")
                            .from(Artifacts::Table, vec![Artifacts::AppName, Artifacts::UserId, Artifacts::SessionId])
                            .to(Sessions::Table, vec![Sessions::AppName, Sessions::UserId, Sessions::Id])
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        // Create indices for performance
        manager
            .create_index(
                Index::create()
                    .name("idx_events_session")
                    .table(Events::Table)
                    .col(Events::AppName)
                    .col(Events::UserId)
                    .col(Events::SessionId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_events_invocation")
                    .table(Events::Table)
                    .col(Events::InvocationId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_artifacts_session")
                    .table(Artifacts::Table)
                    .col(Artifacts::AppName)
                    .col(Artifacts::UserId)
                    .col(Artifacts::SessionId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop artifacts table
        manager
            .drop_table(Table::drop().table(Artifacts::Table).to_owned())
            .await?;

        // Drop user states table
        manager
            .drop_table(Table::drop().table(UserStates::Table).to_owned())
            .await?;

        // Drop app states table
        manager
            .drop_table(Table::drop().table(AppStates::Table).to_owned())
            .await?;

        // Drop events table
        manager
            .drop_table(Table::drop().table(Events::Table).to_owned())
            .await?;

        // Drop sessions table
        manager
            .drop_table(Table::drop().table(Sessions::Table).to_owned())
            .await?;

        Ok(())
    }
}