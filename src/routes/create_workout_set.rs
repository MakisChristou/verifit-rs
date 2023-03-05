use crate::database::sea_orm_active_enums::Bodypart;
use crate::database::workout_sets;
use axum::{Extension, Json};
use sea_orm::{prelude::DateTimeWithTimeZone, DatabaseConnection};
use sea_orm::{ActiveModelTrait, Set};
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
    Extension(database): Extension<DatabaseConnection>,
    Json(request_workout_set): Json<RequestWorkoutSet>,
) {
    let new_workout_set = workout_sets::ActiveModel {
        exercise_name: Set(request_workout_set.exercise_name),
        category: Set(request_workout_set.category),
        reps: Set(request_workout_set.reps),
        weight: Set(request_workout_set.weight),
        ..Default::default()
    };

    let result = new_workout_set.save(&database).await.unwrap();

    dbg!(result);
}
