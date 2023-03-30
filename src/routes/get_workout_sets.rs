use crate::database::users;
use crate::database::users::Entity as Users;
use crate::database::users::Model;
use crate::database::workout_sets;
use crate::database::{sea_orm_active_enums::Bodypart, workout_sets::Entity as WorkoutSets};
use axum::{extract::Path, http::StatusCode, Extension, Json};
use log::warn;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{prelude::DateTimeWithTimeZone, DatabaseConnection, EntityTrait, IntoActiveModel};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ResponseWorkoutSet {
    pub id: i32,
    pub date: DateTimeWithTimeZone,
    pub exercise_name: String,
    pub category: Bodypart,
    pub reps: i32,
    pub weight: f64,
    pub comment: Option<String>,
    pub user_id: Option<i32>,
}

pub async fn get_one_workout_set(
    Extension(user): Extension<Model>,
    Path(set_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseWorkoutSet>, StatusCode> {
    warn!("set fetched by user: {}", user.username);
    let user = user.into_active_model();

    let workout_set = WorkoutSets::find_by_id(set_id)
        .filter(workout_sets::Column::UserId.eq(user.id.unwrap()))
        .one(&database)
        .await
        .unwrap();

    if let Some(workout_set) = workout_set {
        return Ok(Json(ResponseWorkoutSet {
            id: workout_set.id,
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
    Extension(user): Extension<Model>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<Vec<ResponseWorkoutSet>>, StatusCode> {
    let user = user.into_active_model();

    let workout_sets: Vec<ResponseWorkoutSet> = WorkoutSets::find()
        .filter(workout_sets::Column::UserId.eq(user.id.unwrap()))
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_workout_set| ResponseWorkoutSet {
            id: db_workout_set.id,
            date: db_workout_set.date,
            exercise_name: db_workout_set.exercise_name,
            category: db_workout_set.category,
            reps: db_workout_set.reps,
            weight: db_workout_set.weight,
            comment: db_workout_set.comment,
            user_id: db_workout_set.user_id,
        })
        .collect();

    warn!("{} sets fetched by user: {}", workout_sets.len(), user.username.unwrap());
    
    Ok(Json(workout_sets))
}
