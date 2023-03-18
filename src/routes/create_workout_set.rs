use std::sync::Barrier;

use crate::database::sea_orm_active_enums::Bodypart;
use crate::database::users::Entity as Users;
use crate::database::users::{self, Model};
use crate::database::workout_sets;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::StatusCode;
use axum::{Extension, Json, TypedHeader};
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::{prelude::DateTimeWithTimeZone, DatabaseConnection};
use sea_orm::{ActiveModelTrait, Set};
use sea_orm::{ColumnTrait, IntoActiveModel};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestWorkoutSet {
    pub exercise_name: String,
    pub date: DateTimeWithTimeZone,
    pub category: Bodypart,
    pub reps: i32,
    pub weight: f64,
    pub comment: Option<String>,
}

pub async fn create_workout_set(
    Extension(user): Extension<Model>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_workout_set): Json<RequestWorkoutSet>,
) -> Result<Json<i32>, StatusCode> {
    let user = user.into_active_model();

    let new_workout_set = workout_sets::ActiveModel {
        exercise_name: Set(request_workout_set.exercise_name),
        date: Set(request_workout_set.date),
        category: Set(request_workout_set.category),
        reps: Set(request_workout_set.reps),
        weight: Set(request_workout_set.weight),
        user_id: Set(Some(user.id.unwrap())),
        comment: Set(request_workout_set.comment),
        ..Default::default()
    };

    let result = new_workout_set.save(&database).await.unwrap();

    Ok((Json(result.id.unwrap())))
}

pub async fn create_workout_sets(
    Extension(user): Extension<Model>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_workout_set_vector): Json<Vec<RequestWorkoutSet>>,
) -> Result<(), StatusCode> {
    // No one in their right mind has done so many sets in their life
    if request_workout_set_vector.len() > 3_650_000 {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let user_id = user.into_active_model().id.unwrap();
    let mut workout_sets_to_insert = Vec::new();

    for request_workout_set in request_workout_set_vector {
        let new_workout_set = workout_sets::ActiveModel {
            exercise_name: Set(request_workout_set.exercise_name),
            date: Set(request_workout_set.date),
            category: Set(request_workout_set.category),
            reps: Set(request_workout_set.reps),
            weight: Set(request_workout_set.weight),
            user_id: Set(Some(user_id)),
            comment: Set(request_workout_set.comment),
            ..Default::default()
        };
        workout_sets_to_insert.push(new_workout_set);
    }

    workout_sets::Entity::insert_many(workout_sets_to_insert)
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
