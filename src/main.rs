// use axum::{
//     routing::{get, post},
//     http::StatusCode,
//     Json, Router,
//     response::IntoResponse,
//     extract::rejection::JsonRejection,
// };
// use serde::{Deserialize, Serialize};
// use validator::{Validate, ValidationErrors};
// use axum::response::Response;

// #[tokio::main]
// async fn main() {
//     // Initialize tracing
//     tracing_subscriber::fmt::init();

//     // Build our application with routes
//     let app = Router::new()
//         .route("/", get(root))
//         .route("/users", post(create_user));

//     // Run our app with hyper, listening globally on port 3000
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

// // Basic handler that responds with a static string
// async fn root() -> &'static str {
//     "Hello, World! test"
// }

// // Handler to create a user with validation and error handling
// async fn create_user(
//     result: Result<Json<CreateUser>, JsonRejection>,
// ) -> impl IntoResponse {
//     match result {
//         Ok(Json(payload)) => {
//             // Validate the input
//             if let Err(validation_errors) = payload.validate() {
//                 return validation_error_response(validation_errors);
//             }

//             // Application logic for creating a user
//             let user = User {
//                 id: 1337,
//                 username: payload.username,
//                 password: payload.password,
//             };

//             // Return a JSON response with a status code of `201 Created`
//             (StatusCode::CREATED, Json(user)).into_response()
//         }
//         Err(err) => {
//             // Handle JSON parsing errors
//             json_parsing_error_response(err)
//         }
//     }
// }

// // Function to handle validation errors and return a proper response
// fn validation_error_response(err: ValidationErrors) -> Response {
//     let error_message = err.to_string();
//     (
//         StatusCode::BAD_REQUEST,
//         Json(serde_json::json!({
//             "error": error_message
//         })),
//     )
//     .into_response()
// }

// // Function to handle JSON parsing errors and return a proper response
// fn json_parsing_error_response(err: JsonRejection) -> Response {
//     (
//         StatusCode::BAD_REQUEST,
//         Json(serde_json::json!({
//             "error": format!("{}", err)
//         })),
//     )
//     .into_response()
// }

// // The input to our `create_user` handler
// #[derive(Deserialize, Validate)]
// struct CreateUser {
//     #[validate(length(min = 3, message = "username must be at least 3 characters"))]
//     username: String,
//     #[validate(length(min = 8, message = "password must be at least 8 characters"))]
//     password: String,
// }

// // The output to our `create_user` handler
// #[derive(Serialize)]
// struct User {
//     id: u64,
//     username: String,
//     password: String,
// }

mod domain;

fn main() {
    domain::entities::user_entities::hello("hello");
}