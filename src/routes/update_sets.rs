use crate::database::users::Model;
use crate::database::{sea_orm_active_enums::Bodypart, workout_sets, workout_sets::Entity as Sets};
use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{ColumnTrait, IntoActiveModel, TransactionTrait};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestWorkoutSet {
    pub id: Option<i32>,
    pub exercise_name: String,
    pub category: Bodypart,
    pub reps: i32,
    pub weight: f64,
    pub comment: Option<String>,
}

pub async fn atomic_update_set(
    Extension(user): Extension<Model>,
    Path(set_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_set): Json<RequestWorkoutSet>,
) -> Result<(), StatusCode> {
    let user = user.into_active_model();

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
        .filter(workout_sets::Column::UserId.eq(user.id.unwrap()))
        .filter(workout_sets::Column::Id.eq(set_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

pub async fn atomic_update_sets(
    Extension(user): Extension<Model>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_sets): Json<Vec<RequestWorkoutSet>>,
) -> Result<(), StatusCode> {
    let user_id = user.into_active_model().id.unwrap();

    let txn = database
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Loop through the request_sets and update each set in the database
    for request_set in request_sets.iter() {
        let set_id = request_set.id.unwrap();

        let update_set = workout_sets::ActiveModel {
            id: Set(set_id),
            exercise_name: Set(request_set.exercise_name.clone()),
            category: Set(request_set.category.clone()),
            reps: Set(request_set.reps),
            weight: Set(request_set.weight),
            comment: Set(request_set.comment.clone()),
            ..Default::default()
        };

        Sets::update(update_set)
            .filter(workout_sets::Column::UserId.eq(user_id))
            .filter(workout_sets::Column::Id.eq(set_id))
            .exec(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
