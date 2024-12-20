use axum::Router;
use base64::{engine::general_purpose::STANDARD as b64, Engine};
use dotenvy::dotenv;
use repository::{GroupRepository, UserGroupRepository, UserRepository};
use tokio::net::TcpListener;

mod controller;
mod model;
mod repository;

use controller::{groups, users, users_groups};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = database().await;
    http(db).await;
}

async fn database() -> sqlx::Pool<sqlx::Postgres> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("Could not connect to database")
}

async fn http(db: sqlx::Pool<sqlx::Postgres>) {
    let b64_salt = std::env::var("SALT").expect("SALT must be set");
    let salt = b64.decode(b64_salt).unwrap();

    // start repositories
    let group_repo = GroupRepository::new(db.clone());
    let user_repo = UserRepository::new(db.clone(), salt);
    let user_group_repo = UserGroupRepository::new(db);

    // apply routes
    let app = Router::new()
        .nest("/groups", groups::routes(group_repo))
        .nest("/users", users::routes(user_repo))
        .nest("/users", users_groups::routes(user_group_repo));

    let addr = std::env::var("HTTP_ADDR").expect("HTTP_ADDR must be set");
    let tcp = TcpListener::bind(addr)
        .await
        .expect("could not bind to address");
    axum::serve(tcp, app).await.expect("could not start server");
}
