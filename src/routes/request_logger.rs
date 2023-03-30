use axum::{
    extract::{Extension, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn request_logger<T: std::fmt::Debug>(
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, StatusCode> {
    // let peer_addr = request.headers();

    println!("{:?}", request.uri());

    // let authorization: Option<TypedHeader<Authorization<Bearer>>> =
    //     request.headers().typed_get();

    // if let Some(auth) = authorization {
    //     let token = auth.into_inner().0.token().to_string();
    //     println!("Request from IP: {}, token: {}", peer_addr, token);
    // } else {
    //     println!("Request from IP: {}", peer_addr);
    // }

    let response = next.run(request).await;

    Ok(response)
}
