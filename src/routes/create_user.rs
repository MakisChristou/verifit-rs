use crate::database::users;
use axum::{http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: Option<String>,
}

pub async fn create_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let new_user = users::ActiveModel {
        username: Set(request_user.username),
        password: Set(request_user.password),
        token: Set(Some(String::from("Testtoken"))),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser {
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap(),
    }))
}
