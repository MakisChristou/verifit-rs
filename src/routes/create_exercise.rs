use crate::database::exercises;
use crate::database::sea_orm_active_enums::Bodypart;
use crate::database::users;
use crate::database::users::Entity as Users;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::StatusCode;
use axum::Extension;
use axum::Json;
use axum::TypedHeader;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    title: String,
    priority: Option<String>,
    description: Option<String>,
}

#[derive(Deserialize)]
pub struct RequestExercise {
    name: String,
    bodypart: Bodypart,
    isfavorite: bool,
}

pub async fn create_exercise(
    authorization: TypedHeader<Authorization<Bearer>>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_exercise): Json<RequestExercise>,
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

    let new_exercise = exercises::ActiveModel {
        name: Set(request_exercise.name),
        bodypart: Set(request_exercise.bodypart),
        isfavorite: Set(request_exercise.isfavorite),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    let result = new_exercise.save(&database).await.unwrap();

    dbg!(result);

    Ok(())
}
