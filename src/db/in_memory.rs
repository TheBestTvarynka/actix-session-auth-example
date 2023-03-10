use std::{collections::HashMap, io};

use tracing::instrument;
use uuid::Uuid;

use crate::model::{Session, User};

use super::{AuthRepository, SessionRepository};

#[derive(Default, Debug)]
pub struct InMemoryAuthRepo {
    users: HashMap<Uuid, User>,
}

impl AuthRepository for InMemoryAuthRepo {
    #[instrument(level = "debug", ret)]
    fn get_user(&self, id: &Uuid) -> Result<User, io::Error> {
        self.users
            .get(id)
            .map(Clone::clone)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "user not found"))
    }

    #[instrument(level = "debug", ret, fields(users = ?self.users, username = user.username), skip_all)]
    fn add_user(&mut self, user: User) -> Result<(), io::Error> {
        let id = user.id;

        self.users.insert(id, user);

        Ok(())
    }

    #[instrument(level = "debug", ret)]
    fn get_user_by_username(&self, username: &str) -> Result<User, io::Error> {
        tracing::debug!(?username, users = ?self.users, "");

        self.users
            .values()
            .find(|user| user.username == username)
            .map(Clone::clone)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "user not found"))
    }
}

#[derive(Default, Debug)]
pub struct InMemorySessionRepo {
    sessions: HashMap<Uuid, Session>,
}

impl SessionRepository for InMemorySessionRepo {
    fn get_session(&self, id: &Uuid) -> Result<Session, io::Error> {
        self.sessions
            .get(id)
            .map(Clone::clone)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "session not found"))
    }

    fn add_session(&mut self, session: Session) -> Result<(), io::Error> {
        let id = session.id;

        self.sessions.insert(id, session);

        Ok(())
    }
}
