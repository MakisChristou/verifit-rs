use crate::database::{exercises::Entity as Exercises, sea_orm_active_enums::Bodypart};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseExercise {
    name: String,
    bodypart: Bodypart,
    isfavorite: bool,
}

pub async fn get_one_exercise(
    Path(exercise_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseExercise>, StatusCode> {
    let exercise = Exercises::find_by_id(exercise_id)
        .one(&database)
        .await
        .unwrap();

    if let Some(exercise) = exercise {
        return Ok(Json(ResponseExercise {
            name: exercise.name,
            bodypart: exercise.bodypart,
            isfavorite: exercise.isfavorite,
        }));
    }

    Err(StatusCode::NOT_FOUND)
}

pub async fn get_all_exercises(
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<Vec<ResponseExercise>>, StatusCode> {
    let exercises = Exercises::find()
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_exercise| ResponseExercise {
            name: db_exercise.name,
            bodypart: db_exercise.bodypart,
            isfavorite: db_exercise.isfavorite,
        })
        .collect();

    Ok(Json(exercises))
}
