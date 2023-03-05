use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{prelude::DateTimeWithTimeZone, DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};

use crate::database::{sea_orm_active_enums::Bodypart, workout_sets::Entity as WorkoutSets};

#[derive(Deserialize, Serialize)]
pub struct ResponseWorkoutSet {
    pub date: DateTimeWithTimeZone,
    pub exercise_name: String,
    pub category: Bodypart,
    pub reps: i32,
    pub weight: f64,
    pub comment: Option<String>,
    pub user_id: Option<i32>,
}

pub async fn get_one_workout_set(
    Path(set_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseWorkoutSet>, StatusCode> {
    let workout_set = WorkoutSets::find_by_id(set_id)
        .one(&database)
        .await
        .unwrap();

    if let Some(workout_set) = workout_set {
        return Ok(Json(ResponseWorkoutSet {
            date: workout_set.date,
            exercise_name: workout_set.exercise_name,
            category: workout_set.category,
            reps: workout_set.reps,
            weight: workout_set.weight,
            comment: workout_set.comment,
            user_id: workout_set.user_id,
        }));
    }

    Err(StatusCode::NOT_FOUND)
}

pub async fn get_all_workout_sets(
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<Vec<ResponseWorkoutSet>>, StatusCode> {
    let workout_sets = WorkoutSets::find()
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_workout_set| ResponseWorkoutSet {
            date: db_workout_set.date,
            exercise_name: db_workout_set.exercise_name,
            category: db_workout_set.category,
            reps: db_workout_set.reps,
            weight: db_workout_set.weight,
            comment: db_workout_set.comment,
            user_id: db_workout_set.user_id,
        })
        .collect();

    Ok(Json(workout_sets))
}
