use sea_orm::{DbConn, EntityTrait};
use crate::models::session::{Entity as SessionEntity, Model as Session};

pub struct SessionStore {
    db: DbConn,
}

impl SessionStore {
    pub fn new(db: DbConn) -> Self {
        Self { db }
    }

    pub async fn create(&self, session: Session) -> Result<Session, sea_orm::DbErr> {
        SessionEntity::insert(session).exec(&self.db).await?;
        Ok(session)
    }

    pub async fn get_by_id(&self, id: String) -> Result<Option<Session>, sea_orm::DbErr> {
        SessionEntity::find_by_id(id).one(&self.db).await
    }

    pub async fn list_all(&self) -> Result<Vec<Session>, sea_orm::DbErr> {
        SessionEntity::find().all(&self.db).await
    }
}