use crate::database::{exercises, exercises::Entity as Exercises, sea_orm_active_enums::Bodypart};
use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::ColumnTrait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestExercise {
    pub id: Option<i32>,
    pub name: String,
    pub bodypart: Bodypart,
    pub isfavorite: bool,
    pub user_id: Option<i32>,
}

pub async fn atomic_update(
    Path(execrise_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_exercise): Json<RequestExercise>,
) -> Result<(), StatusCode> {
    let update_exercise = exercises::ActiveModel {
        id: Set(execrise_id),
        name: Set(request_exercise.name),
        bodypart: Set(request_exercise.bodypart),
        isfavorite: Set(request_exercise.isfavorite),
        user_id: Set(request_exercise.user_id),
    };

    Exercises::update(update_exercise)
        .filter(exercises::Column::Id.eq(execrise_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
