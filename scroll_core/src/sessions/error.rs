use std::fmt;

#[derive(Debug)]
pub enum SessionError {
    NotFound,
    Db(sea_orm::DbErr),
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionError::NotFound => write!(f, "session not found"),
            SessionError::Db(err) => write!(f, "database error: {}", err),
        }
    }
}

impl std::error::Error for SessionError {}

impl From<sea_orm::DbErr> for SessionError {
    fn from(err: sea_orm::DbErr) -> Self {
        SessionError::Db(err)
    }
}
