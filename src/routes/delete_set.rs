use crate::database::{
    sea_orm_active_enums::Bodypart, users::Model, workout_sets, workout_sets::Entity as Sets,
};
use axum::Json;
use axum::{extract::Path, http::StatusCode, Extension};
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel};

pub async fn delete_set(
    Extension(user): Extension<Model>,
    Path(set_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<(), StatusCode> {
    let user = user.into_active_model();

    let set = if let Some(set) = Sets::find_by_id(set_id)
        .filter(workout_sets::Column::UserId.eq(user.id.unwrap()))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        set.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    Sets::delete(set)
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

pub async fn delete_sets(
    Extension(user): Extension<Model>,
    Extension(database): Extension<DatabaseConnection>,
    Json(set_ids): Json<Vec<i32>>,
) -> Result<(), StatusCode> {
    let user_id = user.into_active_model().id.unwrap();

    Sets::delete_many()
        .filter(workout_sets::Column::UserId.eq(user_id))
        .filter(workout_sets::Column::Id.is_in(set_ids))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
