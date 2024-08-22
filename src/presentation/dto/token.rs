use serde::{Serialize, Deserialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub role: String,
    pub exp: i64,
}