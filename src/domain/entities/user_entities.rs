// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    // pub password: String,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}
