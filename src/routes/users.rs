use crate::database::users::Entity as Users;
use crate::database::users::{self, Model};
use crate::utils::jwt::create_jwt;
use axum::{http::StatusCode, Extension, Json};
use dotenvy_macro::dotenv;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::{error, warn};
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPasswordResetUser {
    username: String,
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
    if password.len() < 8 && password.len() > 128{
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
) -> Result<Json<ResponseUser>, StatusCode> {
    warn!(
        "create_user attempt with username: {}",
        request_user.username.to_string()
    );

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
    .map_err(|err| {
        error!("error saving the new user: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    warn!("create user sucessful");

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
    warn!("login attempt with email: {}", {
        request_user.username.to_string()
    });

    let db_user = Users::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&database)
        .await
        .map_err(|err| {
            error!("error finding the user {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if let Some(db_user) = db_user {
        if !verify_password(request_user.password, &db_user.password)? {
            warn!("user unauthorized");
            return Err(StatusCode::UNAUTHORIZED);
        }
        let jwt = create_jwt()?;
        let new_token = String::from(jwt);
        let mut user = db_user.into_active_model();

        user.token = Set(Some(new_token));

        let saved_user = user.save(&database).await.map_err(|err| {
            error!("error saving the new user: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        warn!("login succesful");

        // do the login here
        Ok(Json(ResponseUser {
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
)-> Result<(), StatusCode>{    
    warn!("request_password_reset email {}", password_reset_user.username);

    // Check if username is in database
    let mut db_user = Users::find()
        .filter(users::Column::Username.eq(password_reset_user.username))
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

    user.token = Set(None);
    let jwt = create_jwt()?;
    let new_reset_code = String::from(jwt);
    user.reset_code = Set(Some(new_reset_code.clone()));
    
    let recipient_email = user.username.clone().unwrap();

    user.save(&database).await.map_err(|err| {
        error!("error saving the new user: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Send the recovery email
    if let Ok(()) = send_reset_email(new_reset_code, recipient_email).await {
        return Ok(());
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
}

pub async fn change_password(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
)-> Result<(), StatusCode>{

    // Check if username is in database
    // Check if user has password reset code and not expired
    // Change the user's password

    Ok(())
}

async fn send_reset_email(new_reset_code: String, username: String)
 -> Result<(), StatusCode>{

    let email_username: &'static str = dotenv!("EMAIL_USERNAME");
    let email_password: &'static str = dotenv!("EMAIL_PASSWORD");
    let smtp_server: &'static str = dotenv!("SMTP_SERVER");

    let email = Message::builder()
    .from(format!("<{}>", email_username).parse().unwrap())
    .to(format!("<{}>", username).parse().unwrap())
    .subject("Password Reset Code")
    .header(ContentType::TEXT_PLAIN)
    .body(new_reset_code)
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
        },
        Err(e) => {
            warn!("Could not send email: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

fn hash_password(password: String) -> Result<String, StatusCode> {
    bcrypt::hash(password, 10).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn verify_password(password: String, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
