use axum::{routing::{get, post}, Router};
use crate::{app::state::user_state::UserState, presentation::handler::user_handler};

pub fn routes() -> Router<UserState> {
    Router::new().route("/user", get(user_handler::get_user))
}