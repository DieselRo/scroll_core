// ==================================
// src/adk/sessions/database_service.rs
// ==================================

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, DatabaseTransaction,
    EntityTrait, Order, QueryFilter, QueryOrder, QuerySelect, 
    Set, TransactionTrait,
};
use sea_orm::prelude::Expr;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::adk::common::error::AdkError;
use crate::adk::events::event::Event;
use crate::adk::sessions::base_session_service::BaseSessionService;
use crate::adk::sessions::session::{ListSessionsResponse, Session, SessionStatus, SessionSummary};
use crate::adk::sessions::state::GetSessionConfig;

/// Database-backed session service
pub struct DatabaseSessionService {
    /// Database connection
    db: DatabaseConnection,
}

impl DatabaseSessionService {
    /// Create a new database session service
    pub async fn new(url: &str) -> Result<Self, AdkError> {
        let db = sea_orm::Database::connect(url).await?;
        Ok(Self { db })
    }
    
    /// Convert a database session model to a Session
    async fn model_to_session(
        &self,
        model: entity::sessions::Model,
        include_events: bool,
        max_events: Option<u32>,
    ) -> Result<Session, AdkError> {
        // Parse state from JSON
        let state: HashMap<String, serde_json::Value> = serde_json::from_value(model.state)?;
        
        // Create session
        let mut session = Session {
            id: model.id,
            app_name: model.app_name,
            user_id: model.user_id,
            state,
            events: Vec::new(),
            create_time: model.create_time,
            last_update_time: model.update_time,
            status: match model.status.as_str() {
                "ACTIVE" => SessionStatus::Active,
                "CLOSED" => SessionStatus::Closed,
                _ => SessionStatus::Active, // Default
            },
        };
        
        // Fetch events if requested
        if include_events {
            let mut query = entity::events::Entity::find()
                .filter(
                    Condition::all()
                        .add(entity::events::Column::AppName.eq(session.app_name.clone()))
                        .add(entity::events::Column::UserId.eq(session.user_id.clone()))
                        .add(entity::events::Column::SessionId.eq(session.id.clone())),
                )
                .order_by(entity::events::Column::Timestamp, Order::Asc);
            
            // Apply max_events limit if specified
            if let Some(limit) = max_events {
                query = query.limit(limit as u64);
            }
            
            let event_models = query.all(&self.db).await?;
            
            for event_model in event_models {
                // Convert database model to Event
                let event = self.model_to_event(event_model)?;
                session.events.push(event);
            }
        }
        
        Ok(session)
    }
    
    /// Convert a database event model to an Event
    fn model_to_event(&self, model: entity::events::Model) -> Result<Event, AdkError> {
        // Parse content from JSON
        let content = if let Some(content_json) = model.content {
            Some(serde_json::from_value(content_json)?)
        } else {
            None
        };
        
        // Parse actions from JSON
        let actions = if let Some(actions_json) = model.actions {
            serde_json::from_value(actions_json)?
        } else {
            Default::default()
        };
        
        Ok(Event {
            id: model.id,
            invocation_id: model.invocation_id,
            author: model.author,
            content,
            actions,
            branch: model.branch,
            timestamp: model.timestamp,
            partial: model.partial,
            turn_complete: model.turn_complete,
            interrupted: model.interrupted,
            error_code: model.error_code,
            error_message: model.error_message,
            long_running_tool_ids: None, // Not stored in database
        })
    }
    
    /// Convert an Event to a database event model
    fn event_to_model(
        &self,
        event: &Event,
        app_name: &str,
        user_id: &str,
        session_id: &str,
    ) -> Result<entity::events::ActiveModel, AdkError> {
        // Convert content to JSON
        let content = if let Some(content) = &event.content {
            Some(serde_json::to_value(content)?)
        } else {
            None
        };
        
        // Convert actions to JSON
        let actions = Some(serde_json::to_value(&event.actions)?);
        
        Ok(entity::events::ActiveModel {
            id: Set(event.id.clone()),
            app_name: Set(app_name.to_string()),
            user_id: Set(user_id.to_string()),
            session_id: Set(session_id.to_string()),
            invocation_id: Set(event.invocation_id.clone()),
            author: Set(event.author.clone()),
            branch: Set(event.branch.clone()),
            timestamp: Set(event.timestamp),
            content: Set(content),
            actions: Set(actions),
            partial: Set(event.partial),
            turn_complete: Set(event.turn_complete),
            interrupted: Set(event.interrupted),
            error_code: Set(event.error_code.clone()),
            error_message: Set(event.error_message.clone()),
        })
    }
}

#[async_trait]
impl BaseSessionService for DatabaseSessionService {
    async fn create_session(
        &self,
        app_name: &str,
        user_id: &str,
        state: Option<HashMap<String, serde_json::Value>>,
        session_id: Option<&str>,
    ) -> Result<Session, AdkError> {
        let id = session_id.map(|s| s.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string());
        
        // Create session model
        let now = Utc::now().timestamp_millis() as f64 / 1000.0;
        let session_model = entity::sessions::ActiveModel {
            app_name: Set(app_name.to_string()),
            user_id: Set(user_id.to_string()),
            id: Set(id.clone()),
            state: Set(serde_json::to_value(state.unwrap_or_default())?),
            create_time: Set(now),
            update_time: Set(now),
            status: Set("ACTIVE".to_string()),
        };
        
        // Insert into database
        let session_model = session_model.insert(&self.db).await?;
        
        // Convert to Session
        self.model_to_session(session_model, false, None).await
    }
    
    async fn get_session(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
        config: Option<GetSessionConfig>,
    ) -> Result<Option<Session>, AdkError> {
        // Get session model
        let session_model = entity::sessions::Entity::find()
            .filter(
                Condition::all()
                    .add(entity::sessions::Column::AppName.eq(app_name))
                    .add(entity::sessions::Column::UserId.eq(user_id))
                    .add(entity::sessions::Column::Id.eq(session_id)),
            )
            .one(&self.db)
            .await?;
        
        if let Some(session_model) = session_model {
            // Get config
            let config = config.unwrap_or_default();
            
            // Convert to Session
            let session = self.model_to_session(
                session_model,
                config.include_events,
                config.max_events,
            ).await?;
            
            Ok(Some(session))
        } else {
            Ok(None)
        }
    }
    
    async fn list_sessions(
        &self,
        app_name: &str,
        user_id: &str,
    ) -> Result<ListSessionsResponse, AdkError> {
        // Get session models
        let session_models = entity::sessions::Entity::find()
            .filter(
                Condition::all()
                    .add(entity::sessions::Column::AppName.eq(app_name))
                    .add(entity::sessions::Column::UserId.eq(user_id)),
            )
            .order_by(entity::sessions::Column::UpdateTime, Order::Desc)
            .all(&self.db)
            .await?;
        
        // Convert to SessionSummary
        let sessions = session_models
            .into_iter()
            .map(|model| SessionSummary {
                id: model.id,
                app_name: model.app_name,
                user_id: model.user_id,
                create_time: model.create_time,
                last_update_time: model.update_time,
                status: match model.status.as_str() {
                    "ACTIVE" => SessionStatus::Active,
                    "CLOSED" => SessionStatus::Closed,
                    _ => SessionStatus::Active, // Default
                },
            })
            .collect();
        
        Ok(ListSessionsResponse {
            sessions,
            next_page_token: None,
        })
    }
    
    async fn delete_session(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
    ) -> Result<(), AdkError> {
        // Start a transaction
        let txn = self.db.begin().await?;
        
        // Delete session events
        entity::events::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(entity::events::Column::AppName.eq(app_name))
                    .add(entity::events::Column::UserId.eq(user_id))
                    .add(entity::events::Column::SessionId.eq(session_id)),
            )
            .exec(&txn)
            .await?;
        
        // Delete session
        let result = entity::sessions::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(entity::sessions::Column::AppName.eq(app_name))
                    .add(entity::sessions::Column::UserId.eq(user_id))
                    .add(entity::sessions::Column::Id.eq(session_id)),
            )
            .exec(&txn)
            .await?;
        
        // Commit transaction
        txn.commit().await?;
        
        if result.rows_affected == 0 {
            Err(AdkError::NotFound(format!("Session not found: {}", session_id)))
        } else {
            Ok(())
        }
    }
    
    async fn append_event(
        &self,
        session: &mut Session,
        event: Event,
    ) -> Result<Event, AdkError> {
        // Start a transaction
        let txn = self.db.begin().await?;
        
        // Convert event to model
        let event_model = self.event_to_model(
            &event,
            &session.app_name,
            &session.user_id,
            &session.id,
        )?;
        
        // Insert event
        event_model.insert(&txn).await?;
        
        // Update session update time
        let now = Utc::now().timestamp_millis() as f64 / 1000.0;
        entity::sessions::Entity::update_many()
            .filter(
                Condition::all()
                    .add(entity::sessions::Column::AppName.eq(&session.app_name))
                    .add(entity::sessions::Column::UserId.eq(&session.user_id))
                    .add(entity::sessions::Column::Id.eq(&session.id)),
            )
            .col_expr(entity::sessions::Column::UpdateTime, Expr::value(now))
            .exec(&txn)
            .await?;
        
        // Commit transaction
        txn.commit().await?;
        
        // Update session
        session.last_update_time = now;
        session.add_event(event.clone());
        
        Ok(event)
    }
    
    async fn update_session_state(
        &self,
        session: &mut Session,
        state: HashMap<String, serde_json::Value>,
    ) -> Result<(), AdkError> {
        // Update local state
        for (key, value) in &state {
            session.set_state_value(key.clone(), value.clone());
        }
        
        // Update database
        let now = Utc::now().timestamp_millis() as f64 / 1000.0;
        entity::sessions::Entity::update_many()
            .filter(
                Condition::all()
                    .add(entity::sessions::Column::AppName.eq(&session.app_name))
                    .add(entity::sessions::Column::UserId.eq(&session.user_id))
                    .add(entity::sessions::Column::Id.eq(&session.id)),
            )
            .col_expr(entity::sessions::Column::State, Expr::value(serde_json::to_value(&session.state)?))
            .col_expr(entity::sessions::Column::UpdateTime, Expr::value(now))
            .exec(&self.db)
            .await?;
        
        session.last_update_time = now;
        
        Ok(())
    }
    
    async fn close_session(
        &self,
        session: &mut Session,
    ) -> Result<(), AdkError> {
        // Update local state
        session.close();
        
        // Update database
        let now = Utc::now().timestamp_millis() as f64 / 1000.0;
        entity::sessions::Entity::update_many()
            .filter(
                Condition::all()
                    .add(entity::sessions::Column::AppName.eq(&session.app_name))
                    .add(entity::sessions::Column::UserId.eq(&session.user_id))
                    .add(entity::sessions::Column::Id.eq(&session.id)),
            )
            .col_expr(entity::sessions::Column::Status, Expr::value("CLOSED"))
            .col_expr(entity::sessions::Column::UpdateTime, Expr::value(now))
            .exec(&self.db)
            .await?;
        
        session.last_update_time = now;
        
        Ok(())
    }
}

/// Module containing entity definitions
mod entity {
    /// Sessions table entity
    pub mod sessions {
        use sea_orm::entity::prelude::*;
        
        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "sessions")]
        pub struct Model {
            #[sea_orm(primary_key, column_name = "app_name")]
            pub app_name: String,
            
            #[sea_orm(primary_key, column_name = "user_id")]
            pub user_id: String,
            
            #[sea_orm(primary_key, column_name = "id")]
            pub id: String,
            
            pub state: serde_json::Value,
            pub create_time: f64,
            pub update_time: f64,
            pub status: String,
        }
        
        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {
            #[sea_orm(has_many = "super::events::Entity")]
            Events,
        }
        
        impl Related<super::events::Entity> for Entity {
            fn to() -> RelationDef {
                Relation::Events.def()
            }
        }
        
        impl ActiveModelBehavior for ActiveModel {}
    }
    
    /// Events table entity
    pub mod events {
        use sea_orm::entity::prelude::*;
        
        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "events")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub id: String,
            
            pub app_name: String,
            pub user_id: String,
            pub session_id: String,
            pub invocation_id: String,
            pub author: String,
            
            #[sea_orm(nullable)]
            pub branch: Option<String>,
            
            pub timestamp: f64,
            
            #[sea_orm(nullable)]
            pub content: Option<serde_json::Value>,
            
            #[sea_orm(nullable)]
            pub actions: Option<serde_json::Value>,
            
            #[sea_orm(nullable)]
            pub partial: Option<bool>,
            
            #[sea_orm(nullable)]
            pub turn_complete: Option<bool>,
            
            #[sea_orm(nullable)]
            pub interrupted: Option<bool>,
            
            #[sea_orm(nullable)]
            pub error_code: Option<String>,
            
            #[sea_orm(nullable)]
            pub error_message: Option<String>,
        }
        
        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {
            #[sea_orm(belongs_to = "super::sessions::Entity", from = "Column::SessionId", to = "super::sessions::Column::Id", on_delete = "Cascade")]
            Session,
        }
        
        impl Related<super::sessions::Entity> for Entity {
            fn to() -> RelationDef {
                Relation::Session.def()
            }
        }
        
        impl ActiveModelBehavior for ActiveModel {}
    }
}