use crate::database::users::Entity as Users;
use crate::database::users::{self, Model};
use crate::utils::jwt::create_jwt;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::TypedHeader;
use axum::{http::StatusCode, Extension, Json};
use regex::Regex;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: Option<String>,
}

fn is_email_valid(email: &str) -> bool {
    let email_regex = Regex::new(r"(?i)^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

fn is_valid_password(password: &str) -> bool {
    // Check if the password is at least 8 characters long
    if password.len() < 8 {
        return false;
    }

    // Check if the password contains at least one letter and one number
    let mut has_letter = false;
    let mut has_number = false;

    for ch in password.chars() {
        if ch.is_alphabetic() {
            has_letter = true;
        } else if ch.is_numeric() {
            has_number = true;
        }

        // If both conditions are met, return true
        if has_letter && has_number {
            return true;
        }
    }

    // If either condition is not met, return false
    false
}

pub async fn create_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    if !is_email_valid(&request_user.username) || !is_valid_password(&request_user.password) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let jwt = create_jwt()?;

    let new_user = users::ActiveModel {
        username: Set(request_user.username),
        password: Set(hash_password(request_user.password)?),
        token: Set(Some(jwt)),
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
        if !verify_password(request_user.password, &db_user.password)? {
            return Err(StatusCode::UNAUTHORIZED);
        }
        let jwt = create_jwt()?;
        let new_token = String::from(jwt);
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
    Extension(database): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<(), StatusCode> {
    let mut user = user.into_active_model();

    user.token = Set(None);
    user.save(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

fn hash_password(password: String) -> Result<String, StatusCode> {
    bcrypt::hash(password, 10).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn verify_password(password: String, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
