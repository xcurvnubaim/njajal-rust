use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: T,
}

impl <T> SuccessResponse<T> {
    pub fn new(message: String, data: T) -> Self {
        SuccessResponse {
            success: true,
            message,
            data,
        }
    }
}