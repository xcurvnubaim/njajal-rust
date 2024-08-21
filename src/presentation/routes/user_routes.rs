use axum::{routing::{get, post}, Router};
use crate::{app::state::user_state::UserState, presentation::handler::user_handler};

pub fn routes() -> Router<UserState> {
    let router = Router::new()
        .route("/", get(user_handler::get_user))
        .route("/register", post(user_handler::create_user))
        .route("/login", post(user_handler::login));

    return Router::new()
        .nest("/user", router);
}