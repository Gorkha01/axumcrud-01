use axum::{Extension, Json, extract::Path, http::StatusCode};

use crate::{
    models::{User, UserInfo},
    service::UserService,
};

pub async fn list_users(service: Extension<UserService>) -> Result<Json<Vec<User>>, StatusCode> {
    // get user

    match service.list_users().await {
        Ok(users) => Ok(Json(users)),
        Err(err) => {
            eprintln!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user_by_id(
    service: Extension<UserService>,
    Path(id): Path<u64>,
) -> Result<Json<User>, StatusCode> {
    // get user by id

    match service.get_user_by_id(id.try_into().unwrap()).await {
        Ok(user) => Ok(Json(user)),
        Err(err) => {
            eprintln!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_user(
    service: Extension<UserService>,
    Json(user): Json<UserInfo>,
) -> StatusCode {
    // create user

    match service.create_user(user).await {
        Ok(()) => StatusCode::OK,

        Err(err) => {
            eprintln!("{:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn update_user(
    service: Extension<UserService>,
    Path(id): Path<u64>,
    Json(user): Json<UserInfo>,
) -> StatusCode {
    // update user
    match service.update_user(id.try_into().unwrap(), user).await {
        Ok(_) => StatusCode::OK,

        Err(err) => {
            eprintln!("{:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn delete_user(service: Extension<UserService>, Path(id): Path<u64>) -> StatusCode {
    // delete user
    match service.delete_user(id.try_into().unwrap()).await {
        Ok(()) => StatusCode::NO_CONTENT,

        Err(err) => {
            eprintln!("{:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
