use axum::extract::Request;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use validator::Validate;

use crate::app::response::error::ErrorResponse;
use crate::app::response::success::SuccessResponse;
use crate::presentation::dto::token::Claims;
use crate::presentation::dto::user_dto::{CreateUserDTO, GetUserDTO, GetUserLoginDTO};
use crate::{
    app::state::user_state::UserState,
    presentation::dto::user_dto::GetAllUserDTO,
};


pub async fn get_user(
    State(user_state): State<UserState>,
) -> Result<(StatusCode, Json<SuccessResponse<GetAllUserDTO>>), (StatusCode, Json<ErrorResponse>)> {
    match user_state.user_usecase.get_all_users().await {
        Ok(users) => {

            let res = SuccessResponse::new(
                "Successfully fetched all users".to_string(),
                GetAllUserDTO { users },
            );

            Ok((StatusCode::OK, Json(res)))
        }
        Err(err) => {
            // Handle the error appropriately
            // You might want to return a more meaningful error response or log the error
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(err),
            ))
        }
    }
}

pub async fn create_user(
    State(user_state): State<UserState>,
    Json(payload): Json<CreateUserDTO>,
) -> Result<(StatusCode, Json<SuccessResponse<GetUserDTO>>), (StatusCode, Json<ErrorResponse>)> {

    match payload.validate() {
        Ok(_) => {}
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new("Validation error".to_string(), err.to_string())),
            ));
        }
    }

    match user_state
        .user_usecase
        .create_user(payload.name, payload.email, payload.password)
        .await
    {
        Ok(user) => {
            let res = SuccessResponse::new(
                "Successfully created a new user".to_string(),
                GetUserDTO {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                },
            );

            Ok((StatusCode::CREATED, Json(res)))
        }
        Err(err) => {
            // Handle the error appropriately
            // You might want to return a more meaningful error response or log the error
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(err),
            ))
        }
    }
}

pub async fn login(
    State(user_state): State<UserState>,
    Json(payload): Json<CreateUserDTO>,
) -> Result<(StatusCode, Json<SuccessResponse<GetUserLoginDTO>>), (StatusCode, Json<ErrorResponse>)> {
    match user_state
        .user_usecase
        .login_user(payload.email, payload.password)
        .await
    {
        Ok((user, token)) => {
            let res = SuccessResponse::new(
                "Successfully logged in".to_string(),
                GetUserLoginDTO {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                    token : token
                },
            );

            Ok((StatusCode::OK, Json(res)))
        }
        Err(err) => {
            // Handle the error appropriately
            // You might want to return a more meaningful error response or log the error
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(err),
            ))
        }
    }
}

pub async fn get_me(
    State(user_state): State<UserState>,
    req: Request
) -> Result<(StatusCode, Json<SuccessResponse<GetUserDTO>>), (StatusCode, Json<ErrorResponse>)> {

    let user = req.extensions().get::<Claims>()
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("User not found in request extensions".to_string(), "UserNotFound".to_string())),
            )
        })?;
    match user_state.user_usecase.get_me(user.id).await {
        Ok(user) => {
            let res = SuccessResponse::new(
                "Successfully fetched user".to_string(),
                GetUserDTO {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                },
            );

            Ok((StatusCode::OK, Json(res)))
        }
        Err(err) => {
            // Handle the error appropriately
            // You might want to return a more meaningful error response or log the error
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(err),
            ))
        }
    }
}