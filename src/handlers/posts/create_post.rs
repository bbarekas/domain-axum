// Import necessary modules and types
use axum::extract::State;
use axum::Json;

// Import internal modules and types
use crate::domain::models::post::PostError;
use crate::handlers::posts::{CreatePostRequest, PostResponse};
use crate::infra::repositories::db_repository;


// This is a placeholder to extract JSON data from the request body.
use crate::utils::JsonExtractor;
use crate::AppState;

// Define the handler function for creating a new post
pub async fn create_post(
    State(state): State<AppState>,                  // Extract the application state from the request
    JsonExtractor(new_post): JsonExtractor<CreatePostRequest>,  // Extract JSON data from the request body
) -> Result<Json<PostResponse>, PostError> {
    // Create a NewPostDb instance with data from the JSON request
    let new_post_db = db_repository::NewPostDb {
        title: new_post.title,
        body: new_post.body,
        published: false, // Set the initial 'published' status to false
    };

    // Insert the new post into the database using the repository
    let created_post = db_repository::insert(&state.pool, new_post_db)
        .await
        .map_err(PostError::InfraError)?;  // Handle potential infrastructure errors

    // Create a PostResponse instance from the newly created post
    let post_response = PostResponse {
        id: created_post.id,
        title: created_post.title,
        body: created_post.body,
        published: created_post.published,
    };

    // Return the response as JSON with a success status
    Ok(Json(post_response))
}
