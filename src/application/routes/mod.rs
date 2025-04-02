pub mod comment_routes;
pub mod post_routes;
pub mod user_routes;

use axum::Router;
use axum::routing::get;

use crate::domain::services::{
    comment_service::CommentService, post_service::PostService, user_service::UserService,
};

pub fn create_routes<C, P, U>(comment_service: C, post_service: P, user_service: U) -> Router
where
    C: CommentService + Clone + Send + Sync + 'static,
    P: PostService + Clone + Send + Sync + 'static,
    U: UserService + Clone + Send + Sync + 'static,
{
    Router::new()
        .nest("/posts", post_routes::post_router(post_service))
        .nest("/users", user_routes::user_router(user_service))
        .nest("/comments", comment_routes::comment_router(comment_service))
        .route("/health", get(|| async { "OK" }))
}
