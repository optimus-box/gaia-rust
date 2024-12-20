use sqlx::{query, query_as};

use crate::model::{User, UserWithGroups};

#[derive(Clone)]
pub struct UserGroupRepository {
    db: sqlx::Pool<sqlx::Postgres>,
}

impl UserGroupRepository {
    pub fn new(db: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<UserWithGroups>, sqlx::Error> {
        let sql = "select * from users";
        let query = query_as(sql);
        let users: Vec<User> = query.fetch_all(&self.db).await?;
        let mut result = vec![];
        for user in users {
            let sql = "select g.* from groups g inner join users_groups ug on g.id = ug.group_id where ug.user_id = $1";
            let query = query_as(sql).bind(user.id);
            let groups = query.fetch_all(&self.db).await?;
            result.push(UserWithGroups::new(user, groups));
        }
        Ok(result)
    }

    pub async fn find_by_user(&self, id: i64) -> Result<UserWithGroups, sqlx::Error> {
        let sql = "select * from users where id = $1";
        let query = query_as(sql).bind(id);
        let data: User = query.fetch_one(&self.db).await?;

        let sql = "select g.* from groups g inner join users_groups ug on g.id = ug.group_id where ug.user_id = $1";
        let query = query_as(sql).bind(id);

        match query.fetch_all(&self.db).await {
            Ok(list) => Ok(UserWithGroups::new(data, list)),
            Err(sqlx::Error::RowNotFound) => Ok(UserWithGroups::new(data, vec![])),
            Err(err) => Err(err),
        }
    }

    pub async fn create(&self, user_id: i64, group_ids: Vec<i64>) -> Result<(), sqlx::Error> {
        self.delete_by_user(user_id).await?;
        let sql = "insert into users_groups (user_id, group_id) select * from unnest($1, $2)";
        let user_ids = vec![user_id; group_ids.len()];
        let query = query(sql).bind(user_ids).bind(group_ids);
        match query.execute(&self.db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub async fn delete_by_user(&self, id: i64) -> Result<(), sqlx::Error> {
        let sql = "delete from users_groups where user_id = $1";
        let query = query(sql).bind(id);
        match query.execute(&self.db).await {
            Ok(_) => Ok(()),
            Err(sqlx::Error::RowNotFound) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
