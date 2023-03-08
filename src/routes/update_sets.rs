use crate::database::{sea_orm_active_enums::Bodypart, workout_sets, workout_sets::Entity as Sets};
use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::ColumnTrait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestWorkoutSet {
    pub id: Option<i32>,
    pub exercise_name: String,
    pub category: Bodypart,
    pub reps: i32,
    pub weight: f64,
    pub comment: Option<String>,
}

pub async fn atomic_update_set(
    Path(set_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_set): Json<RequestWorkoutSet>,
) -> Result<(), StatusCode> {
    let update_set = workout_sets::ActiveModel {
        id: Set(set_id),
        exercise_name: Set(request_set.exercise_name),
        category: Set(request_set.category),
        reps: Set(request_set.reps),
        weight: Set(request_set.weight),
        comment: Set(request_set.comment),
        ..Default::default()
    };

    Sets::update(update_set)
        .filter(workout_sets::Column::Id.eq(set_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
