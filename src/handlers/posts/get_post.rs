// Import necessary modules and types
use axum::extract::State;
use axum::Json;
use uuid::Uuid;

// Import internal modules and types
use crate::domain::models::post::{PostError, PostModel};
use crate::handlers::posts::PostResponse;
use crate::infra::errors::InfraError;
use crate::infra::repositories::db_repository;

// Import PathExtractor for extracting the post_id from the request path
use crate::utils::PathExtractor;
use crate::AppState;

// Define the handler function for retrieving a specific post by its ID
pub async fn get_post(
    State(state): State<AppState>,                    // Extract the application state from the request
    PathExtractor(post_id): PathExtractor<Uuid>,       // Extract the post_id from the request path
) -> Result<Json<PostResponse>, PostError> {
    // Use the post_repository to fetch the post based on its ID
    let post = db_repository::get(&state.pool, post_id)
        .await
        .map_err(|db_error| match db_error {
            // Map infrastructure errors to custom PostError types
            InfraError::InternalServerError => PostError::InternalServerError,
            InfraError::NotFound => PostError::NotFound(post_id),
        })?;

    // Convert the retrieved PostModel to a PostResponse
    Ok(Json(adapt_post_to_post_response(post)))
}

// Helper function to adapt a PostModel to a PostResponse
fn adapt_post_to_post_response(post: PostModel) -> PostResponse {
    PostResponse {
        id: post.id,
        title: post.title,
        body: post.body,
        published: post.published,
    }
}