use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;

// Import internal handlers and the AppState type
use crate::handlers::posts::{create_post, get_post, list_posts};
use crate::AppState;

// Define the main application router
pub fn app_router(state: AppState) -> Router<AppState> {
    // Create a new Router for the application
    Router::new()
        // Define a route for the root path "/"
        .route("/", get(root))
        // Nest a sub-router under the path "/v1/posts"
        .nest("/v1/posts", posts_routes(state.clone()))
        // Define a fallback handler for 404 Not Found errors
        .fallback(handler_404)
}

// Handler for the root path "/"
async fn root() -> &'static str {
    "Server is running!"  // Return a simple message indicating the server is running
}

// Fallback handler for 404 Not Found errors
async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,               // Set the HTTP status code to 404 Not Found
        "The requested resource was not found",  // Provide an error message
    )
}

// Define a sub-router for handling posts-related routes
fn posts_routes(state: AppState) -> Router<AppState> {
    // Create a new Router for posts-related routes
    Router::new()
        // Define a route for creating a new post using the HTTP POST method
        .route("/", post(create_post))
        // Define a route for listing posts using the HTTP GET method
        .route("/", get(list_posts))
        // Define a route for retrieving a specific post by ID using the HTTP GET method
        .route("/:id", get(get_post))
        // Provide the application state to this sub-router
        .with_state(state)
}
