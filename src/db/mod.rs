use std::io;

use uuid::Uuid;

use crate::model::{Session, User};

pub mod in_memory;

pub trait AuthRepository {
    fn get_user(&self, id: &Uuid) -> Result<User, io::Error>;
    fn get_user_by_username(&self, username: &str) -> Result<User, io::Error>;
    fn add_user(&mut self, user: User) -> Result<(), io::Error>;
}

pub trait SessionRepository {
    fn get_session(&self, id: &Uuid) -> Result<Session, io::Error>;
    fn add_session(&mut self, session: Session) -> Result<(), io::Error>;
}
