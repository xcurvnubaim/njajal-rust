use dotenv::dotenv;
use infrastructure::db::postgres::DatabaseTrait;
use presentation::routes;
use std::sync::Arc;
use tokio;

mod domain;
mod infrastructure;
mod presentation;
mod app;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = infrastructure::db::postgres::Database::init_pool()
        .await
        .unwrap_or_else(|e| panic!("Database error: {}", e.to_string()));
    // build our application with a single route
    


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes::root_routes::routes(Arc::new(pool))).await.unwrap();
}