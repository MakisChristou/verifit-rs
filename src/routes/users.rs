use crate::database::users;
use crate::database::users::Entity as Users;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::TypedHeader;
use axum::{http::StatusCode, Extension, Json};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
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

pub async fn login(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let db_user = Users::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(db_user) = db_user {
        let new_token = String::from("qwjhqwehkjhdas");
        let mut user = db_user.into_active_model();

        user.token = Set(Some(new_token));

        let saved_user = user
            .save(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // do the login here
        Ok(Json(ResponseUser {
            username: saved_user.username.unwrap(),
            id: saved_user.id.unwrap(),
            token: Some(saved_user.token.unwrap().unwrap()),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn logout(
    authorization: TypedHeader<Authorization<Bearer>>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<(), StatusCode> {
    let token = authorization.token();

    let mut user = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        user.into_active_model()
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    user.token = Set(None);
    user.save(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
