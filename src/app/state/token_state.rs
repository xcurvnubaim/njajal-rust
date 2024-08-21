use std::env;

#[derive(Clone)]
pub struct TokenState {
    pub secret: String,
}

impl TokenState {
    pub fn new() -> Self {
        let secret = env::var("SECRET").expect("SECRET must be set");
        Self {
            secret
        }
    }
}