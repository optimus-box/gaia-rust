use crate::repository;
use axum::{http::StatusCode, Json};
use serde::Serialize;

pub mod groups;
pub mod users;
pub mod users_groups;

#[derive(Debug, Serialize)]
pub struct Error {
    message: String,
    field: Option<String>,
}

impl Error {
    pub fn internal(err: repository::Error) -> (StatusCode, Json<Self>) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Self {
                message: err.to_string(),
                field: None,
            }),
        )
    }

    pub fn from_sql(err: sqlx::Error) -> (StatusCode, Json<Self>) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Self {
                message: err.to_string(),
                field: None,
            }),
        )
    }

    pub fn not_found(message: &str) -> (StatusCode, Json<Self>) {
        (
            StatusCode::NOT_FOUND,
            Json(Self {
                message: message.to_string(),
                field: None,
            }),
        )
    }

    // pub fn bad_request(message: &str, field: &str) -> (StatusCode, Json<Self>) {
    //     (
    //         StatusCode::BAD_REQUEST,
    //         Json(Self {
    //             message: message.to_string(),
    //             field: Some(field.to_string()),
    //         }),
    //     )
    // }
}
