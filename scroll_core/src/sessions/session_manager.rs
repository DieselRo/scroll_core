use crate::sessions::session_store::SessionStore;
use crate::models::session::Model as Session;

pub struct SessionManager {
    store: SessionStore,
}

impl SessionManager {
    pub fn new(store: SessionStore) -> Self {
        Self { store }
    }

    pub async fn start_new(&self, user_id: Option<String>) -> Result<Session, sea_orm::DbErr> {
        let session = Session {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            created_at: chrono::Utc::now(),
            ..Default::default()
        };
        self.store.create(session.clone()).await?;
        Ok(session)
    }

    pub async fn resume(&self, session_id: String) -> Result<Option<Session>, sea_orm::DbErr> {
        self.store.get_by_id(session_id).await
    }
}
