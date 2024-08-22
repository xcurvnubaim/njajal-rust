use super::{upload_routes, user_routes};
use axum::routing::{get, IntoMakeService};
use axum::Router;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::app::state::{token_state, user_state};
use crate::infrastructure::db::postgres::Database;

pub fn routes(db_conn: Arc<Database>) -> IntoMakeService<Router> {
    let merged_router = {
        let user_state = user_state::UserState::new(&db_conn);

        user_routes::routes()
            .with_state(user_state)
            .merge(upload_routes::routes())
            .merge(Router::new().route("/health", get(|| async { "Healthy..." })))
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app_router = Router::new()
        .nest("/api", merged_router)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    app_router.into_make_service()
}
