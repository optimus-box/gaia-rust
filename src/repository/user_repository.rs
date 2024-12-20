use argon2::Config;
use sqlx::{query, query_as, query_scalar};

use crate::{
    model::{User, UserCreate, UserUpdate},
    repository::Error,
};

#[derive(Clone)]
pub struct UserRepository {
    db: sqlx::Pool<sqlx::Postgres>,
    salt: Vec<u8>,
}

impl UserRepository {
    pub fn new(db: sqlx::Pool<sqlx::Postgres>, salt: Vec<u8>) -> Self {
        Self { db, salt }
    }

    pub fn hash_password(&self, password: String) -> Result<Vec<u8>, Error> {
        let config = Config::default();
        match argon2::hash_raw(password.as_bytes(), &self.salt, &config) {
            Ok(hash) => Ok(hash),
            Err(err) => Err(Error::Argon2(err)),
        }
    }

    pub async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let sql = "select * from users";
        match query_as(sql).fetch_all(&self.db).await {
            Ok(list) => Ok(list),
            Err(err) => Err(err),
        }
    }

    pub async fn find_by_id(&self, id: i32) -> Result<User, sqlx::Error> {
        let sql = "select * from users where id = $1";
        match query_as(sql).bind(id).fetch_one(&self.db).await {
            Ok(user) => Ok(user),
            Err(err) => Err(err),
        }
    }

    pub async fn create(&self, req: UserCreate) -> Result<User, Error> {
        let hash = self.hash_password(req.password)?;
        let sql = "insert into users (name, phone, role, email, username, password) values ($1, $2, $3, $4, $5, $6) returning *";
        let query = query_as(sql)
            .bind(req.name)
            .bind(req.phone)
            .bind(req.role)
            .bind(req.email)
            .bind(req.username)
            .bind(hash);
        match query.fetch_one(&self.db).await {
            Ok(user) => Ok(user),
            Err(err) => Err(Error::Sql(err)),
        }
    }

    pub async fn update(&self, id: i32, req: UserUpdate) -> Result<User, sqlx::Error> {
        let sql = "update users set name = $2, phone = $3, role = $4, updated_at = extract(epoch from now()) where id = $1 returning *";
        let query = query_as(sql)
            .bind(id)
            .bind(req.name)
            .bind(req.phone)
            .bind(req.role);
        match query.fetch_one(&self.db).await {
            Ok(user) => Ok(user),
            Err(err) => Err(err),
        }
    }

    pub async fn delete(&self, id: i32) -> Result<(), sqlx::Error> {
        let sql = "update users set deleted_at = extract(epoch from now()) where id = $1";
        let query = query(sql).bind(id);
        match query.execute(&self.db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub async fn exists(&self, id: i32) -> bool {
        let sql = "select exists(select 1 from users where id = $1)";
        let query = query_scalar(sql).bind(id);
        match query.fetch_one(&self.db).await {
            Ok(row) => row,
            Err(_) => false,
        }
    }
}
