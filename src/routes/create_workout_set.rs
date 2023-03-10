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
    pub category: Bodypart,
    pub reps: i32,
    pub weight: f64,
    pub comment: Option<String>,
}

pub async fn create_workout_set(
    Extension(user): Extension<Model>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_workout_set): Json<RequestWorkoutSet>,
) -> Result<(), StatusCode> {
    let user = user.into_active_model();

    let new_workout_set = workout_sets::ActiveModel {
        exercise_name: Set(request_workout_set.exercise_name),
        category: Set(request_workout_set.category),
        reps: Set(request_workout_set.reps),
        weight: Set(request_workout_set.weight),
        user_id: Set(Some(user.id.unwrap())),
        ..Default::default()
    };

    let result = new_workout_set.save(&database).await.unwrap();

    dbg!(result);

    Ok(())
}
