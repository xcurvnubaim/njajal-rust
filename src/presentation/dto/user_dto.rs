use serde::{Deserialize, Serialize};

use crate::domain::entities::user_entities::GetUser;
#[derive(Clone, Serialize, Deserialize)]
pub struct GetAllUserDTO {
    pub data: Vec<GetUser>,
}