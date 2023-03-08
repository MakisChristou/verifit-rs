use crate::database::{sea_orm_active_enums::Bodypart, workout_sets, workout_sets::Entity as Sets};
use axum::{extract::Path, http::StatusCode, Extension};
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel};

pub async fn delete_set(
    Path(set_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<(), StatusCode> {
    let set = if let Some(set) = Sets::find_by_id(set_id)
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
