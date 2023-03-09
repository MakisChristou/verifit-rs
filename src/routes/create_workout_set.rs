use std::sync::Barrier;

use crate::database::sea_orm_active_enums::Bodypart;
use crate::database::users;
use crate::database::users::Entity as Users;
use crate::database::workout_sets;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::StatusCode;
use axum::{Extension, Json, TypedHeader};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
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
    authorization: TypedHeader<Authorization<Bearer>>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_workout_set): Json<RequestWorkoutSet>,
) -> Result<(), StatusCode> {
    let token = authorization.token();

    let user = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        user
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let new_workout_set = workout_sets::ActiveModel {
        exercise_name: Set(request_workout_set.exercise_name),
        category: Set(request_workout_set.category),
        reps: Set(request_workout_set.reps),
        weight: Set(request_workout_set.weight),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    let result = new_workout_set.save(&database).await.unwrap();

    dbg!(result);

    Ok(())
}
