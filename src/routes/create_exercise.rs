use crate::database::exercises;
use crate::database::sea_orm_active_enums::Bodypart;
use axum::Extension;
use axum::Json;
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
    is_favorite: bool,
}

pub async fn create_exercise(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_exercise): Json<RequestExercise>,
) {
    let new_exercise = exercises::ActiveModel {
        name: Set(request_exercise.name),
        bodypart: Set(request_exercise.bodypart),
        isfavorite: Set(request_exercise.is_favorite),
        ..Default::default()
    };

    let result = new_exercise.save(&database).await.unwrap();

    dbg!(result);
}
