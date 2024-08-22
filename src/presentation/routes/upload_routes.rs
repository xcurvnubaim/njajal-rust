use axum::{
    middleware, 
    routing::{get, post}, 
    Router
};
use crate::{
    app::middleware::verify_token::auth, 
    presentation::handler::{upload_handler, user_handler::{self, get_me}}
};

pub fn routes() -> Router {
    // Define routes that don't require authentication
    let unauthenticated_routes = Router::new()
        .route("/", post(upload_handler::upload));

    // Define routes that require authentication
    let authenticated_routes = Router::new()
        .layer(middleware::from_fn(auth))
        .merge(unauthenticated_routes); // Apply authentication middleware here

    // Combine both routers, with authenticated routes nested under `/user`
    let router = Router::new()
        .nest("/upload", authenticated_routes); // Routes requiring authentication


    router
}
