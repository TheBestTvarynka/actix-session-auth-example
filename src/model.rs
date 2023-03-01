use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub full_name: String,
    pub joined_at: OffsetDateTime,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expiration_date: OffsetDateTime,
}
