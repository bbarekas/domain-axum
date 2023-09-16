// Import necessary modules and types
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

// Define an enumeration for custom application errors
#[derive(Debug)]
pub enum AppError {
    InternalServerError,        // Represents an internal server error
    BodyParsingError(String),   // Represents an error related to request body parsing
}

// Define a util to create an internal server error
pub fn internal_error<E>(_err: E) -> AppError {
    AppError::InternalServerError
}

// Implement the `IntoResponse` trait for the `AppError` enumeration
impl IntoResponse for AppError {
    // Define the conversion to an Axum response
    fn into_response(self) -> axum::response::Response {
        // Define status and error message based on the error variant
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal Server Error"),
            ),
            Self::BodyParsingError(message) => (
                StatusCode::BAD_REQUEST,
                format!("Bad request error: {}", message),
            ),
        };

        // Create a JSON response containing the error message
        (status, Json(json!({ "message": err_msg }))).into_response()
    }
}
