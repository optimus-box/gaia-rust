use std::fmt;

mod group_repository;
mod user_group_repository;
mod user_repository;
pub use group_repository::GroupRepository;
pub use user_group_repository::UserGroupRepository;
pub use user_repository::UserRepository;

pub enum Error {
    Sql(sqlx::Error),
    Argon2(argon2::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Sql(err) => write!(f, "{}", err),
            Error::Argon2(err) => write!(f, "{}", err),
        }
    }
}
