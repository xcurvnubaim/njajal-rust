use serde::{Deserialize, Serialize};

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