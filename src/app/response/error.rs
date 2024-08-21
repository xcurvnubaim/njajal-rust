use serde::{Deserialize, Serialize};
use thiserror::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
    pub error: String,
}

impl ErrorResponse {
    pub fn new(message: String, error: String) -> Self {
        ErrorResponse {
            success : false,
            message,
            error
        }
    }
}