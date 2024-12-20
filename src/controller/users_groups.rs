use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing, Json, Router,
};

use crate::{model::UserWithGroups, repository::UserGroupRepository};

use super::Error;

pub fn routes(repo: UserGroupRepository) -> Router {
    Router::new()
        .route("/group", routing::get(index))
        .route("/:id/group", routing::get(show))
        .route("/:id/group", routing::post(create))
        .with_state(repo)
        .into()
}

pub async fn index(
    State(repo): State<UserGroupRepository>,
) -> Result<Json<Vec<UserWithGroups>>, (StatusCode, Json<Error>)> {
    match repo.find_all().await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(Error::from_sql(err)),
    }
}

pub async fn show(
    State(repo): State<UserGroupRepository>,
    Path(id): Path<i64>,
) -> Result<Json<UserWithGroups>, (StatusCode, Json<Error>)> {
    match repo.find_by_user(id).await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(Error::from_sql(err)),
    }
}

pub async fn create(
    State(repo): State<UserGroupRepository>,
    Path(id): Path<i64>,
    Json(req): Json<Vec<i64>>,
) -> Result<StatusCode, (StatusCode, Json<Error>)> {
    match repo.create(id, req).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(err) => Err(Error::from_sql(err)),
    }
}
