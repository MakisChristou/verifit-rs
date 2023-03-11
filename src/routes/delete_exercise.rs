use crate::database::users::Model;
use crate::database::{exercises, exercises::Entity as Exercises, sea_orm_active_enums::Bodypart};
use axum::{extract::Path, http::StatusCode, Extension};
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel};

pub async fn delete_exercise(
    Extension(user): Extension<Model>,
    Path(exercise_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<(), StatusCode> {
    let user = user.into_active_model();

    let exercise = if let Some(exercise) = Exercises::find_by_id(exercise_id)
        .filter(exercises::Column::UserId.eq(user.id.unwrap()))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        exercise.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    Exercises::delete(exercise)
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
