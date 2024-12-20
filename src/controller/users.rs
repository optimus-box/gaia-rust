use crate::controller::Error;
use crate::model::{User, UserCreate, UserUpdate};
use crate::repository::UserRepository;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{routing, Json, Router};

pub fn routes(repo: UserRepository) -> Router {
    Router::new()
        .route("/", routing::get(index))
        .route("/:id", routing::get(show))
        .route("/", routing::post(create))
        .route("/:id", routing::put(update))
        .route("/:id", routing::delete(delete))
        .with_state(repo)
}

pub async fn exists(repo: UserRepository, id: i32) -> Result<(), (StatusCode, Json<Error>)> {
    if !repo.exists(id).await {
        return Err(Error::not_found("user not found"));
    }
    Ok(())
}

#[axum::debug_handler]
pub async fn index(
    State(repo): State<UserRepository>,
) -> Result<Json<Vec<User>>, (StatusCode, Json<Error>)> {
    match repo.find_all().await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(Error::from_sql(err)),
    }
}

#[axum::debug_handler]
pub async fn show(
    State(repo): State<UserRepository>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, Json<Error>)> {
    exists(repo.clone(), id).await?;
    match repo.find_by_id(id).await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(Error::from_sql(err)),
    }
}

#[axum::debug_handler]
pub async fn create(
    State(repo): State<UserRepository>,
    Json(req): Json<UserCreate>,
) -> Result<(StatusCode, Json<User>), (StatusCode, Json<Error>)> {
    match repo.create(req).await {
        Ok(data) => Ok((StatusCode::CREATED, Json(data))),
        Err(err) => Err(Error::internal(err)),
    }
}

#[axum::debug_handler]
pub async fn update(
    State(repo): State<UserRepository>,
    Path(id): Path<i32>,
    Json(req): Json<UserUpdate>,
) -> Result<Json<User>, (StatusCode, Json<Error>)> {
    exists(repo.clone(), id).await?;
    match repo.update(id, req).await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(Error::from_sql(err)),
    }
}

#[axum::debug_handler]
pub async fn delete(
    State(repo): State<UserRepository>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, Json<Error>)> {
    exists(repo.clone(), id).await?;
    match repo.delete(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(Error::from_sql(err)),
    }
}
