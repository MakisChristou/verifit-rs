use crate::database::{
    exercises::{self, Entity as Exercises},
    sea_orm_active_enums::Bodypart,
    users::Model,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseExercise {
    id: i32,
    name: String,
    bodypart: Bodypart,
    isfavorite: bool,
    user_id: Option<i32>,
}

pub async fn get_one_exercise(
    Extension(user): Extension<Model>,
    Path(exercise_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseExercise>, StatusCode> {

    let user = user.into_active_model();

    let exercise = Exercises::find_by_id(exercise_id)
        .filter(exercises::Column::UserId.eq(user.id.unwrap()))
        .one(&database)
        .await
        .unwrap();

    if let Some(exercise) = exercise {
        return Ok(Json(ResponseExercise {
            id: exercise.id,
            name: exercise.name,
            bodypart: exercise.bodypart,
            isfavorite: exercise.isfavorite,
            user_id: exercise.user_id,
        }));
    }

    Err(StatusCode::NOT_FOUND)
}

pub async fn get_all_exercises(
    Extension(user): Extension<Model>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<Vec<ResponseExercise>>, StatusCode> {
    let user = user.into_active_model();
    let exercises = Exercises::find()
        .filter(exercises::Column::UserId.eq(user.id.unwrap()))
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_exercise| ResponseExercise {
            id: db_exercise.id,
            name: db_exercise.name,
            bodypart: db_exercise.bodypart,
            isfavorite: db_exercise.isfavorite,
            user_id: db_exercise.user_id,
        })
        .collect();

    Ok(Json(exercises))
}
