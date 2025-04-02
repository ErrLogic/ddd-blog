use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    application::dto::post_dto::{CreatePostRequest, PostResponse, UpdatePostRequest},
    domain::services::post_service::PostService,
    shared::error::ApiError,
};

#[derive(Clone)]
pub struct PostRouterState<S: PostService> {
    pub post_service: S,
}

pub fn post_router<S>(post_service: S) -> Router
where
    S: PostService + Clone + Send + Sync + 'static,
{
    let state = PostRouterState { post_service };

    Router::new()
        .route("/", get(get_posts))
        .route("/", post(create_post))
        .route("/:id", get(get_post))
        .route("/:id", put(update_post))
        .route("/:id", delete(delete_post))
        .with_state(state)
}

async fn get_posts<S>(
    State(state): State<PostRouterState<S>>,
) -> Result<impl IntoResponse, ApiError>
where
    S: PostService,
{
    let posts = state.post_service.get_posts().await?;
    let response = posts
        .into_iter()
        .map(PostResponse::from)
        .collect::<Vec<_>>();
    Ok(Json(response))
}

async fn get_post<S>(
    State(state): State<PostRouterState<S>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError>
where
    S: PostService,
{
    let post = state.post_service.get_post(id).await?;
    Ok(Json(PostResponse::from(post)))
}

async fn create_post<S>(
    State(state): State<PostRouterState<S>>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<impl IntoResponse, ApiError>
where
    S: PostService,
{
    payload.validate()?;
    let post = state.post_service.create_post(payload.into()).await?;
    Ok((StatusCode::CREATED, Json(PostResponse::from(post))))
}

async fn update_post<S>(
    State(state): State<PostRouterState<S>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<impl IntoResponse, ApiError>
where
    S: PostService,
{
    payload.validate()?;
    let post = state.post_service.update_post(id, payload.into()).await?;
    Ok(Json(PostResponse::from(post)))
}

async fn delete_post<S>(
    State(state): State<PostRouterState<S>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError>
where
    S: PostService,
{
    state.post_service.delete_post(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
