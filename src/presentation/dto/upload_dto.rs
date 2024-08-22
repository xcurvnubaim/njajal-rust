use serde::{Deserialize, Serialize};



#[derive(Clone, Serialize, Deserialize)]
pub struct UploadFileDTO {
    pub file_name: String,
}