use std::convert::Infallible;
use axum::http::StatusCode;
use axum::{extract::State, http::Error, Json};
use serde::Serialize;

use crate::{app::state::user_state::UserState, infrastructure::repositories::user_repositories::UserRepositoryTrait, presentation::dto::user_dto::GetAllUserDTO};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

pub async fn get_user(
    State(user_state): State<UserState>,
) -> Result<Json<GetAllUserDTO>, (StatusCode, Json<ErrorResponse>)> {
    match user_state.user_repository.get_all_users().await {
        Ok(users) => {
            // Map users to DTO and return as JSON
            let dto = GetAllUserDTO { data: users };
            Ok(Json(dto))
        }
        Err(err) => {
            // Handle the error appropriately
            // You might want to return a more meaningful error response or log the error
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: err.to_string(),
                }),
            ))
        }
    }
}