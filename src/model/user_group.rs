use serde::Serialize;

use super::{Group, User};

#[derive(Debug, Serialize)]
pub struct UserGroup {
    pub user_id: i64,
    pub group_id: i64,
}

#[derive(Debug, Serialize)]
pub struct UserWithGroups {
    pub id: i64,
    pub name: String,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub email: String,
    pub username: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
    pub groups: Vec<Group>,
}

impl UserWithGroups {
    pub fn new(user: User, groups: Vec<Group>) -> Self {
        Self {
            id: user.id,
            name: user.name,
            phone: user.phone,
            role: user.role,
            email: user.email,
            username: user.username,
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
            groups,
        }
    }
}
