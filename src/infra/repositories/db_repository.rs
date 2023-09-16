use diesel::{
    ExpressionMethods, Insertable, PgTextExpressionMethods, QueryDsl, Queryable, RunQueryDsl,
    Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


use crate::domain::models::post::PostModel;
use crate::infra::db::schema::posts;
use crate::infra::errors::{adapt_infra_error, InfraError};

// Define a struct representing the database schema for posts
#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = posts)] // Use the 'posts' table
#[diesel(check_for_backend(diesel::pg::Pg))] // Check compatibility with PostgreSQL
pub struct PostDb {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool,
}

// Define a struct for inserting new posts into the database
#[derive(Deserialize, Insertable)]
#[diesel(table_name = posts)] // Use the 'posts' table
pub struct NewPostDb {
    pub title: String,
    pub body: String,
    pub published: bool,
}

// Define a struct for filtering posts
#[derive(Deserialize)]
pub struct PostsFilter {
    published: Option<bool>,
    title_contains: Option<String>,
}

// Function to insert a new post into the database
pub async fn insert(
    pool: &deadpool_diesel::postgres::Pool,
    new_post: NewPostDb,
) -> Result<PostModel, InfraError> {
    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    // Insert the new post into the 'posts' table, returning the inserted post
    let res = conn
        .interact(|conn| {
            diesel::insert_into(posts::table)
                .values(new_post)
                .returning(PostDb::as_returning()) // Return the inserted post
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    // Adapt the database representation to the application's domain model
    Ok(adapt_post_db_to_post(res))
}

// Function to retrieve a post from the database by its ID
pub async fn get(
    pool: &deadpool_diesel::postgres::Pool,
    id: Uuid,
) -> Result<PostModel, InfraError> {
    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    // Query the 'posts' table to retrieve the post by its ID
    let res = conn
        .interact(move |conn| {
            posts::table
                .filter(posts::id.eq(id))
                .select(PostDb::as_select()) // Select the post
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    // Adapt the database representation to the application's domain model
    Ok(adapt_post_db_to_post(res))
}

// Function to retrieve a list of posts from the database with optional filtering
pub async fn get_all(
    pool: &deadpool_diesel::postgres::Pool,
    filter: PostsFilter,
) -> Result<Vec<PostModel>, InfraError> {
    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    // Build a dynamic query for retrieving posts
    let res = conn
        .interact(move |conn| {
            let mut query = posts::table.into_boxed::<diesel::pg::Pg>();

            // Apply filtering conditions if provided
            if let Some(published) = filter.published {
                query = query.filter(posts::published.eq(published));
            }

            if let Some(title_contains) = filter.title_contains {
                query = query.filter(posts::title.ilike(format!("%{}%", title_contains)));
            }

            // Select the posts matching the query
            query.select(PostDb::as_select()).load::<PostDb>(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    // Adapt the database representations to the application's domain models
    let posts: Vec<PostModel> = res
        .into_iter()
        .map(|post_db| adapt_post_db_to_post(post_db))
        .collect();

    Ok(posts)
}

// Function to adapt a database representation of a post to the application's domain model
fn adapt_post_db_to_post(post_db: PostDb) -> PostModel {
    PostModel {
        id: post_db.id,
        title: post_db.title,
        body: post_db.body,
        published: post_db.published,
    }
}
