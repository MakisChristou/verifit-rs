use crate::database::exercises::Entity as Exercises;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    priority: Option<String>,
    description: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseExercise {
    name: String,
    bodypart: String,
    is_favorite: bool,
}

pub async fn get_one_exercise(
    Path(exercise_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseExercise>, StatusCode> {
    let exercise = Exercises::find_by_id(exercise_id).one(&database).await.unwrap();

    if let Some(exercise) = exercise {
        return Ok(Json(ResponseExercise {
            name: exercise.name,
            bodypart: exercise.bodypart,
            is_favorite: exercise.isfavorite,
        }));
    }

    Err(StatusCode::NOT_FOUND)
}
