use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::entities::user_entities::GetUser;

#[derive(Clone, Serialize, Deserialize)]
pub struct GetUserDTO {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetUserLoginDTO {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub token: String,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct GetAllUserDTO {
    pub users: Vec<GetUser>,
}

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct CreateUserDTO {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
}