use crate::controller::Error;
use crate::model::{Group, GroupCreate, GroupUpdate};
use crate::repository::GroupRepository;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{routing, Json, Router};

pub fn routes(repo: GroupRepository) -> Router {
    Router::new()
        .route("/", routing::get(index))
        .route("/:id", routing::get(show))
        .route("/", routing::post(create))
        .route("/:id", routing::put(update))
        .route("/:id", routing::delete(delete))
        .with_state(repo)
}

pub async fn exists(repo: GroupRepository, id: i64) -> Result<(), (StatusCode, Json<Error>)> {
    if !repo.exists(id).await {
        return Err(Error::not_found("group not found"));
    }
    Ok(())
}

#[axum::debug_handler]
pub async fn index(
    State(repo): State<GroupRepository>,
) -> Result<Json<Vec<Group>>, (StatusCode, Json<Error>)> {
    match repo.find_all().await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(Error::internal(err)),
    }
}

#[axum::debug_handler]
pub async fn show(
    State(repo): State<GroupRepository>,
    Path(id): Path<i64>,
) -> Result<Json<Group>, (StatusCode, Json<Error>)> {
    exists(repo.clone(), id).await?;
    match repo.find_by_id(id).await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(Error::internal(err)),
    }
}

#[axum::debug_handler]
pub async fn create(
    State(repo): State<GroupRepository>,
    Json(req): Json<GroupCreate>,
) -> Result<(StatusCode, Json<Group>), (StatusCode, Json<Error>)> {
    match repo.create(req).await {
        Ok(data) => Ok((StatusCode::CREATED, Json(data))),
        Err(err) => Err(Error::internal(err)),
    }
}

#[axum::debug_handler]
pub async fn update(
    State(repo): State<GroupRepository>,
    Path(id): Path<i64>,
    Json(req): Json<GroupUpdate>,
) -> Result<Json<Group>, (StatusCode, Json<Error>)> {
    exists(repo.clone(), id).await?;
    match repo.update(id, req).await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(Error::internal(err)),
    }
}

#[axum::debug_handler]
pub async fn delete(
    State(repo): State<GroupRepository>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, Json<Error>)> {
    exists(repo.clone(), id).await?;
    match repo.delete(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(Error::internal(err)),
    }
}
