mod application;
mod domain;
mod infrastructure;
mod shared;

use application::routes::{self};
use axum::{Router, routing::get, serve};
use domain::services::{
    comment_service::CommentServiceImpl, post_service::PostServiceImpl,
    user_service::UserServiceImpl,
};
use dotenvy::dotenv;
use infrastructure::database::connection::init_pool;
use infrastructure::repositories::{
    comment_repository_impl::CommentRepositoryImpl, post_repository_impl::PostRepositoryImpl,
    user_repository_impl::UserRepositoryImpl,
};
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Initialize database pool
    let pool = init_pool().expect("Failed to create database pool");

    // Initialize repositories
    let post_repository = Arc::new(PostRepositoryImpl::new(pool.clone()));
    let user_repository = Arc::new(UserRepositoryImpl::new(pool.clone()));
    let comment_repository = Arc::new(CommentRepositoryImpl::new(pool));

    // Initialize services
    let post_service = Arc::new(PostServiceImpl::new(Arc::clone(&post_repository)));
    let user_service = Arc::new(UserServiceImpl::new(Arc::clone(&user_repository)));
    let comment_service = Arc::new(CommentServiceImpl::new(Arc::clone(&comment_repository)));

    // Create router with all routes
    let app = Router::new()
        .nest(
            "/api",
            routes::create_routes(comment_service, post_service, user_service),
        )
        .route("/health", get(|| async { "OK" }));

    // Bind listener
    let listener = TcpListener::bind("127.0.0.1:5000").await.unwrap();
    println!("Server running on {}", listener.local_addr().unwrap());

    // Serve using Axum best practice
    serve(listener, app).await.unwrap();
}
