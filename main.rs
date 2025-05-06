use axum::{
    Extension, Router,
    routing::{delete, get, post, put},
};
use controller::{create_user, delete_user, get_user_by_id, list_users, update_user};
use service::UserService;

mod controller;
mod models;
mod service;

#[tokio::main]
async fn main() {
    println!("tsrting service ....!");
    let user_service = UserService::new().await.expect("cannot connect to database from main");
    let app = Router::new()
        .route("/user", get(list_users))
        .route("/user/{id}", get(get_user_by_id))
        .route("/user", post(create_user))
        .route("/user/{id}", put(update_user))
        .route("/user/{id}", delete(delete_user))
        .layer(Extension(user_service));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("listening ...!");

    axum::serve(listener, app).await.unwrap();
}
