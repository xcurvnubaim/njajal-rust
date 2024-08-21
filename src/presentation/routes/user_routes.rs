use axum::{
    middleware, 
    routing::{get, post}, 
    Router
};
use crate::{
    app::{
        middleware::verify_token::auth, 
        state::{token_state::TokenState, user_state::UserState}
    }, 
    presentation::handler::user_handler::{self, get_me}
};

pub fn routes() -> Router<UserState> {
    // Define routes that don't require authentication
    let unauthenticated_routes = Router::new()
        .route("/register", post(user_handler::create_user))
        .route("/login", post(user_handler::login));

    // Define routes that require authentication
    let authenticated_routes = Router::new()
        .route("/", get(user_handler::get_user))
        .route("/me", get(get_me))
        .layer(middleware::from_fn(auth))
        .merge(unauthenticated_routes); // Apply authentication middleware here

    // Combine both routers, with authenticated routes nested under `/user`
    let router = Router::new()
        .nest("/user", authenticated_routes); // Routes requiring authentication


    router
}
