use std::convert::Infallible;

use crate::database;
// use crate::database::users::Entity as Users;
use crate::database::users::Model;
use crate::database::{users, users::Entity as Users};
use crate::utils::jwt::create_jwt;
use crate::utils::jwt::is_valid;
use axum::extract::Query;
use axum::http::Response;
use axum::{http::StatusCode, Extension, Json};
use dotenvy_macro::dotenv;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::{error, warn};
use regex::Regex;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use sea_orm::{ColumnTrait, DbErr};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PasswordResetUser {
    username: String,
    new_password: String,
    reset_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseLoginUser {
    username: String,
    id: i32,
    token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseSignupUser {
    username: String,
    id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPasswordResetUser {
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestEmailVerificationUser {
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryParams {
    username: String,
    token: String,
}

fn is_email_valid(email: &str) -> bool {
    let email_regex = Regex::new(r"(?i)^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$").unwrap();
    if !email_regex.is_match(email) {
        warn!("email invalid");
        return false;
    } else {
        return true;
    }
}

fn is_valid_password(password: &str) -> bool {
    // Check if the password is at least 8 characters long
    if password.len() < 8 && password.len() > 128 {
        warn!("password invalid");
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
    warn!("password invalid");
    false
}

pub async fn create_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseSignupUser>, StatusCode> {
    warn!(
        "create_user attempt with username: {} and password: {}",
        request_user.username.to_string(),
        request_user.password.to_string(),
    );

    if !is_email_valid(&request_user.username) || !is_valid_password(&request_user.password) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let expiration_duration_email_token: &'static str = dotenv!("PASSWORD_RESET_EXPIRATION");
    let email_verification_jwt = create_jwt(expiration_duration_email_token)?;

    let new_user = users::ActiveModel {
        username: Set(request_user.username.clone()),
        password: Set(hash_password(request_user.password)?),
        email_token: Set(Some(email_verification_jwt.clone())),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|err| {
        error!("error saving the new user: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    warn!("create user sucessful");

    if let Ok(()) = send_email(
        String::from("Verifit: Email Verification"),
        format!(
            "Dear {},\nTo verify your email click on the following link:\n https://verifit.xyz/users/verify-email?username={}&token={}",
            request_user.username.to_string(),
            request_user.username.to_string(),
            email_verification_jwt,
        ),
        request_user.username.to_string(),
    )
    .await
    {
        Ok(Json(ResponseSignupUser {
            username: new_user.username.unwrap(),
            id: new_user.id.unwrap(),
        }))
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
}

pub async fn login(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseLoginUser>, StatusCode> {
    warn!(
        "login attempt with email: {} and password: {}",
        request_user.username.to_string(),
        request_user.password.to_string(),
    );

    let db_user = Users::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&database)
        .await
        .map_err(|err| {
            error!("error finding the user {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if let Some(db_user) = db_user {
        // Check if email is verified
        if !db_user.is_email_verified {
            warn!("user unauthorized, email is not verified");
            return Err(StatusCode::NOT_ACCEPTABLE);
        }

        // Check if password is correct
        if !verify_password(&request_user.password, &db_user.password)? {
            warn!(
                "user unauthorized, invalid password: {}",
                request_user.password
            );
            return Err(StatusCode::UNAUTHORIZED);
        }

        let expiration_duration: &'static str = dotenv!("TOKEN_EXPIRATION"); // in seconds
        let jwt = create_jwt(expiration_duration)?;
        let new_token = String::from(jwt);
        let mut user = db_user.into_active_model();

        user.token = Set(Some(new_token));

        let saved_user = user.save(&database).await.map_err(|err| {
            error!("error saving the new user: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        warn!("login succesful");

        // do the login here
        Ok(Json(ResponseLoginUser {
            username: saved_user.username.unwrap(),
            id: saved_user.id.unwrap(),
            token: Some(saved_user.token.unwrap().unwrap()),
        }))
    } else {
        warn!("login unsucesasful");
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn logout(
    Extension(database): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<(), StatusCode> {
    warn!("logout attempt with email {}", user.username.to_string());

    let mut user = user.into_active_model();

    user.token = Set(None);
    user.save(&database).await.map_err(|err| {
        error!("error saving user {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    warn!("logout succesful");
    Ok(())
}

pub async fn request_password_reset(
    Extension(database): Extension<DatabaseConnection>,
    Json(password_reset_user): Json<RequestPasswordResetUser>,
) -> Result<(), StatusCode> {
    warn!(
        "request_password_reset email {}",
        password_reset_user.username
    );

    if !is_email_valid(&password_reset_user.username){
        warn!("email invalid");
        return Err(StatusCode::NOT_FOUND);
    }

    // Check if username is in database
    let mut db_user = Users::find()
        .filter(users::Column::Username.eq(password_reset_user.username.clone()))
        .one(&database)
        .await
        .map_err(|err| {
            error!("error finding the user {}", err);
            StatusCode::NOT_FOUND
        })?;

    let mut user = match db_user {
        Some(user) => user.into_active_model(),
        None => return Err(StatusCode::NOT_FOUND),
    };

    warn!("user with email {} found in database", password_reset_user.username);

    user.token = Set(None);
    let expiration_duration: &'static str = dotenv!("PASSWORD_RESET_EXPIRATION");
    let jwt = create_jwt(expiration_duration)?;
    let new_reset_token = String::from(jwt);
    user.reset_code = Set(Some(new_reset_token.clone()));

    let recipient_email = user.username.clone().unwrap();

    user.save(&database).await.map_err(|err| {
        error!("error saving the new user: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Send the recovery email
    if let Ok(()) = send_email(
        String::from("Verifit: Password Reset Code"),
        generate_reset_code(&new_reset_token),
        recipient_email,
    )
    .await
    {
        return Ok(());
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
}

pub async fn change_password(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<PasswordResetUser>,
) -> Result<(), StatusCode> {
    warn!("change_password attempt with email: {}", {
        request_user.username.to_string()
    });

    // Check if username is in database
    let db_user = Users::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&database)
        .await
        .map_err(|err| {
            error!("error finding the user {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let reset_token = db_user.clone().expect("User not found").reset_code.unwrap();

    // Check that token supplied in request is the one we want
    if generate_reset_code(&reset_token) != request_user.reset_code {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check if user has password reset code and not expired
    match is_valid(&reset_token) {
        Ok(true) => {
            warn!("Password reset token is valid");
        }
        Ok(false) => {
            warn!("Password reset token is invalid (bool)");
            return Err(StatusCode::BAD_REQUEST);
        }
        Err(e) => {
            warn!("Password reset token is invalid (status code)");
            return Err(e);
        }
    }

    // ToDo: Check if hashed reset code matches the one sent by user

    // Check if new password is valid
    if !is_valid_password(&request_user.new_password) {
        warn!("New password is invalid");
        return Err(StatusCode::BAD_REQUEST);
    }

    // Update the user's password
    let mut user: users::ActiveModel = db_user.unwrap().into();
    user.password = Set(hash_password(request_user.new_password)?);
    user.reset_code = Set(None);
    user.update(&database).await.map_err(|err| {
        error!("error updating the user's password {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    });

    warn!("password changed succesfully");

    Ok(())
}

async fn send_email(title: String, message: String, username: String) -> Result<(), StatusCode> {
    let email_username: &'static str = dotenv!("EMAIL_USERNAME");
    let email_password: &'static str = dotenv!("EMAIL_PASSWORD");
    let smtp_server: &'static str = dotenv!("SMTP_SERVER");

    warn!("Sending to email: {}", username);

    let email = Message::builder()
        .from(format!("<{}>", email_username).parse().unwrap())
        .to(format!("<{}>", username).parse().unwrap())
        .subject(title)
        .header(ContentType::TEXT_PLAIN)
        .body(message)
        .unwrap();

    let creds = Credentials::new(email_username.to_owned(), email_password.to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => {
            warn!("Email sent successfully!");
            Ok(())
        }
        Err(e) => {
            warn!("Could not send email: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn verify_email(
    Extension(database): Extension<DatabaseConnection>,
    Query(params): Query<QueryParams>,
) -> Result<Response<String>, StatusCode> {
    format!(
        "Received query parameters: username = {}, token = {}",
        params.username, params.token,
    );

    // Check if username is in database
    let db_user = Users::find()
        .filter(users::Column::Username.eq(params.username))
        .one(&database)
        .await
        .map_err(|err| {
            error!("error finding the user {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if db_user.clone().unwrap().is_email_verified {
        warn!("email already verified");
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check if token is valid
    if let Ok(validity) = is_valid(&params.token) {
        if !validity {
            warn!("invalid token");
            return Ok(send_http_response("Token invalid"));
        }
    } else {
        error!("is_valid returned an error");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Check if user token matches the querry parameter
    if db_user.clone().unwrap().email_token != Some(params.token) {
        return Ok(send_http_response("Token invalid"));
    }

    // Update user in database with verified email
    let mut user: users::ActiveModel = db_user.unwrap().into();
    user.is_email_verified = Set(true);
    user.update(&database).await.map_err(|err| {
        error!("error updating the user's is_email_verified field {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    });

    warn!("email verified succesfully");
    return Ok(send_http_response("email verified"));
}

fn send_http_response(response_text: &str) -> Response<String> {
    let html = format!("<html><body><h1>{}</h1></body></html>", response_text);
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(html)
        .unwrap()
}

pub async fn request_email_verification(
    Extension(database): Extension<DatabaseConnection>,
    Json(email_verification_user): Json<RequestEmailVerificationUser>,
) -> Result<(), StatusCode> {
    let username = email_verification_user.username.clone();

    warn!("request_email_verification email {}", username,);

    // Check if username is in database
    let mut db_user = Users::find()
        .filter(users::Column::Username.eq(username.clone()))
        .one(&database)
        .await
        .map_err(|err| {
            error!("error finding the user {}", err);
            StatusCode::NOT_FOUND
        })?;

    let mut user = match db_user.clone() {
        Some(user) => user.into_active_model(),
        None => return Err(StatusCode::NOT_FOUND),
    };

    // Generate new email verification token
    let expiration_duration_email_token: &'static str = dotenv!("PASSWORD_RESET_EXPIRATION");
    let email_verification_jwt = create_jwt(expiration_duration_email_token)?;

    // Update the user's email verification token
    let mut user: users::ActiveModel = db_user.unwrap().into();
    user.email_token = Set(Some(email_verification_jwt.clone()));
    user.update(&database).await.map_err(|err| {
        error!("error updating the user's password {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    });

    if let Ok(()) = send_email(
        String::from("Verifit: Email Verification"),
        format!(
            "Dear {},\nTo verify your email click on the following link:\n https://verifit.xyz/users/verify-email?username={}&token={}",
            username,
            username,
            email_verification_jwt,
        ),
        username)
    .await
    {
        Ok(())
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
}

fn hash_password(password: String) -> Result<String, StatusCode> {
    bcrypt::hash(password, 10).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn verify_password(password: &str, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn generate_reset_code(jwt_token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(jwt_token.as_bytes());
    let hash_result = hasher.finalize();

    // 6 decimal digits
    hash_result[..3]
        .iter()
        .map(|c| c.to_string())
        .collect::<String>()
}
