use crate::database::exercises;
use crate::database::sea_orm_active_enums::Bodypart;
use crate::database::users;
use crate::database::users::Entity as Users;
use crate::database::users::Model;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::StatusCode;
use axum::Extension;
use axum::Json;
use axum::TypedHeader;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    title: String,
    priority: Option<String>,
    description: Option<String>,
}

#[derive(Deserialize)]
pub struct RequestExercise {
    name: String,
    bodypart: Bodypart,
    isfavorite: bool,
}

pub async fn create_exercise(
    Extension(user): Extension<Model>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_exercise): Json<RequestExercise>,
) -> Result<(), StatusCode> {
    let user = user.into_active_model();

    let new_exercise = exercises::ActiveModel {
        name: Set(request_exercise.name),
        bodypart: Set(request_exercise.bodypart),
        isfavorite: Set(request_exercise.isfavorite),
        user_id: Set(Some(user.id.unwrap())),
        ..Default::default()
    };

    let result = new_exercise.save(&database).await.unwrap();

    Ok(())
}
