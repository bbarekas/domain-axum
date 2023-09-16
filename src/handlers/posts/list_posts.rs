// Import necessary modules and types
use axum::extract::{Query, State};
use axum::Json;

// Import internal modules and types
use crate::domain::models::post::{PostError, PostModel};
use crate::handlers::posts::{ListPostsResponse, PostResponse};
use crate::infra::repositories::db_repository::{get_all, PostsFilter};
use crate::AppState;

// Define the handler function for listing posts with optional query parameters
pub async fn list_posts(
    State(state): State<AppState>,      // Extract the application state from the request
    Query(params): Query<PostsFilter>,  // Extract query parameters for filtering posts
) -> Result<Json<ListPostsResponse>, PostError> {
    // Use the `get_all` function to retrieve a list of posts based on the provided query parameters
    let posts = get_all(&state.pool, params)
        .await
        .map_err(|_| PostError::InternalServerError)?;

    // Convert the retrieved list of PostModel instances to a ListPostsResponse
    Ok(Json(adapt_posts_to_list_posts_response(posts)))
}

// Helper function to adapt a single PostModel to a PostResponse
fn adapt_post_to_post_response(post: PostModel) -> PostResponse {
    PostResponse {
        id: post.id,
        title: post.title,
        body: post.body,
        published: post.published,
    }
}

// Helper function to adapt a list of PostModel instances to a ListPostsResponse
fn adapt_posts_to_list_posts_response(posts: Vec<PostModel>) -> ListPostsResponse {
    // Map each PostModel to a PostResponse and collect them into a Vec<PostResponse>
    let posts_response: Vec<PostResponse> =
        posts.into_iter().map(adapt_post_to_post_response).collect();

    // Create a ListPostsResponse containing the list of PostResponses
    ListPostsResponse {
        posts: posts_response,
    }
}