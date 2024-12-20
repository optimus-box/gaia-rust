use sqlx::{query, query_scalar, types::Json};

use crate::{
    model::{Group, GroupCreate, GroupUpdate},
    repository::Error,
};

#[derive(Clone)]
pub struct GroupRepository {
    db: sqlx::Pool<sqlx::Postgres>,
}

impl GroupRepository {
    pub fn new(db: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<Group>, Error> {
        let sql = "select * from groups";
        match sqlx::query_as(sql).fetch_all(&self.db).await {
            Ok(groups) => Ok(groups),
            Err(err) => Err(Error::Sql(err)),
        }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Group, Error> {
        let sql = "select * from groups where id = $1";
        match sqlx::query_as(sql).bind(id).fetch_one(&self.db).await {
            Ok(group) => Ok(group),
            Err(err) => Err(Error::Sql(err)),
        }
    }

    pub async fn create(&self, req: GroupCreate) -> Result<Group, Error> {
        let sql = "insert into groups (name, description, roles) values ($1, $2, $3) returning *";
        let query = sqlx::query_as(sql)
            .bind(req.name)
            .bind(req.description)
            .bind(Json(req.roles));
        match query.fetch_one(&self.db).await {
            Ok(group) => Ok(group),
            Err(err) => Err(Error::Sql(err)),
        }
    }

    pub async fn update(&self, id: i64, req: GroupUpdate) -> Result<Group, Error> {
        let sql = "update groups set name = $2 where id = $1 returning *";
        let query = sqlx::query_as(sql)
            .bind(id)
            .bind(req.name)
            .bind(req.description)
            .bind(Json(req.roles));
        match query.fetch_one(&self.db).await {
            Ok(group) => Ok(group),
            Err(err) => Err(Error::Sql(err)),
        }
    }

    pub async fn delete(&self, id: i64) -> Result<(), Error> {
        let sql = "update groups set deleted_at = extract(epoch from now()) where id = $1";
        let query = query(sql).bind(id);
        match query.execute(&self.db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Sql(err)),
        }
    }

    pub async fn exists(&self, id: i64) -> bool {
        let sql = "select exists(select 1 from groups where id = $1)";
        let query = query_scalar(sql).bind(id);
        match query.fetch_one(&self.db).await {
            Ok(row) => row,
            Err(_) => false,
        }
    }
}
