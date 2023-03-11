use crate::database::users::Entity as Users;
use crate::{database::users, utils::jwt::is_valid};
use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::ColumnTrait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};

pub async fn guard<T>(mut request: Request<T>, next: Next<T>) -> Result<Response, StatusCode> {
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(StatusCode::BAD_REQUEST)?
        .token()
        .to_owned();
    let database = request
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let user = Users::find()
        .filter(users::Column::Token.eq(Some(token.clone())))
        .one(database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    is_valid(&token)?; // validating token after getting from the database to obfuscate timing differences

    let Some(user) = user
    else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
