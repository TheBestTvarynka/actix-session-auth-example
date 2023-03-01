use std::fmt::Debug;
use std::io;

use time::{Duration, OffsetDateTime};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    api::{SignInRequest, SignUpRequest, UserResponse},
    db::{AuthRepository, SessionRepository},
    model::{Session, User},
};

#[derive(Debug)]
pub struct AuthService<AuthRepo: AuthRepository + Debug, SessionRepo: SessionRepository + Debug> {
    auth_repo: AuthRepo,
    session_repo: SessionRepo,
}

impl<AuthRepo: AuthRepository + Debug, SessionRepo: SessionRepository + Debug>
    AuthService<AuthRepo, SessionRepo>
{
    pub fn new(auth_repo: AuthRepo, session_repo: SessionRepo) -> Self {
        Self {
            auth_repo,
            session_repo,
        }
    }

    #[instrument(level = "trace", ret)]
    pub fn sign_up(&mut self, user_data: SignUpRequest) -> Result<Uuid, io::Error> {
        let SignUpRequest {
            username,
            email,
            full_name,
            password,
        } = user_data;
        let id = Uuid::new_v4();

        let user = User {
            id,
            username,
            full_name,
            joined_at: OffsetDateTime::now_utc(),
            email,
            password,
        };

        self.auth_repo.add_user(user)?;

        tracing::debug!(auth_repo = ?self.auth_repo);

        Ok(id)
    }

    #[instrument(level = "trace", ret)]
    pub fn sign_in(&mut self, user_data: SignInRequest) -> Result<Uuid, io::Error> {
        tracing::debug!(auth_repo = ?self.auth_repo);

        let user = self.auth_repo.get_user_by_username(&user_data.username)?;

        if user.password != user_data.password {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "wrong password",
            ));
        }

        let session_id = Uuid::new_v4();

        let session = Session {
            id: session_id,
            user_id: user.id,
            expiration_date: OffsetDateTime::now_utc()
                .checked_add(Duration::days(2))
                .unwrap(),
        };
        self.session_repo.add_session(session)?;

        Ok(session_id)
    }

    #[instrument(level = "trace", ret)]
    pub fn profile(&self, session_id: &Uuid) -> Result<UserResponse, io::Error> {
        let session = self.session_repo.get_session(session_id)?;

        if session.expiration_date < OffsetDateTime::now_utc() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "user session is expired. please, login again",
            ));
        }

        let User {
            id,
            username,
            full_name,
            joined_at,
            email,
            password: _,
        } = self.auth_repo.get_user(&session.user_id)?;

        Ok(UserResponse {
            id,
            username,
            full_name,
            email,
            joined_at,
        })
    }
}
