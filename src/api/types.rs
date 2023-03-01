use serde_derive::{Deserialize, Serialize};
use time::{serde::rfc3339, OffsetDateTime};
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct SignUpRequest {
    pub username: String,
    pub email: String,
    #[serde(rename(deserialize = "fullName"))]
    pub full_name: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct SignInRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub full_name: String,
    pub email: String,
    #[serde(with = "rfc3339")]
    pub joined_at: OffsetDateTime,
}
