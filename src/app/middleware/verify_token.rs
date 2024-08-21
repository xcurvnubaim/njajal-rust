use axum::{
    http::{self, StatusCode},
    response::Response,
    middleware::Next,
    extract::Request,
};
use jsonwebtoken::{decode, DecodingKey, Validation, errors::ErrorKind};
use std::env;

use crate::app::response::error::ErrorResponse;
use crate::presentation::dto::token::Claims;

pub async fn auth(
    mut req: Request,
    next: Next,
) -> Response {
    // Extract token from headers
    let headers = req.headers().clone();
    let token = match extract_token_from_headers(headers) {
        Ok(token) => token,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(axum::body::Body::from("Invalid Authorization header"))
                .unwrap();
        }
    };

    // Decode the JWT token
    let secret = env::var("SECRET").expect("SECRET must be set");
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let token_data = match decode::<Claims>(&token, &decoding_key, &Validation::default()) {
        Ok(data) => data,
        Err(err) => {
            let (status, message) = match err.kind() {
                ErrorKind::ExpiredSignature => (StatusCode::UNAUTHORIZED, "Token expired".to_string()),
                ErrorKind::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
                ErrorKind::InvalidIssuer => (StatusCode::UNAUTHORIZED, "Invalid issuer".to_string()),
                ErrorKind::InvalidSignature => (StatusCode::UNAUTHORIZED, "Invalid signature".to_string()),
                _ => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
            };
            return Response::builder()
                .status(status)
                .body(axum::body::Body::from(message))
                .unwrap();
        }
    };

    // Create and insert the user into request extensions
    let user = Claims {
        id: token_data.claims.id,
        role: token_data.claims.role,
        exp: token_data.claims.exp,
    };
    req.extensions_mut().insert(user);

    // Proceed to the next middleware/handler
    let res = next.run(req).await;
    res
}

fn extract_token_from_headers(headers: http::HeaderMap) -> Result<String, ErrorResponse> {
    if let Some(auth_header) = headers.get(http::header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            let parts: Vec<&str> = auth_str.split_whitespace().collect();
            if parts.len() == 2 && parts[0] == "Bearer" {
                return Ok(parts[1].to_string());
            }
        }
    }
    Err(ErrorResponse::new(
        "Invalid Authorization header".to_string(),
        "InvalidAuthHeader".to_string(),
    ))
}
