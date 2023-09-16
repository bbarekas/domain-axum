// Import necessary modules and types
use axum::extract::rejection::JsonRejection;
use axum_macros::FromRequest;

// Import internal AppError type
use crate::errors::AppError;

// Define a custom extractor for JSON data
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]  // Derive the FromRequest trait with specific configuration
pub struct JsonExtractor<T>(pub T);

// Implement the conversion from JsonRejection to AppError
impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        // Convert the JsonRejection into a BodyParsingError with the rejection message
        AppError::BodyParsingError(rejection.to_string())
    }
}