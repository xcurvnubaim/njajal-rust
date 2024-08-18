use super::user_routes;
use axum::routing::{get, IntoMakeService};
use axum::{middleware, Router};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::app::state::user_state;
use crate::infrastructure::db::postgres::Database;

pub fn routes(db_conn: Arc<Database>) -> IntoMakeService<Router> {
    let merged_router = {
        let user_state = user_state::UserState::new(&db_conn);


        user_routes::routes().with_state(user_state)
            .merge(Router::new().route("/health", get(|| async { "Healthy..." })))        
    };

    let app_router = Router::new()
        .nest("/api", merged_router)
        .layer(TraceLayer::new_for_http());

    app_router.into_make_service()
}
