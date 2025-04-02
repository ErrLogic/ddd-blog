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
    application::dto::comment_dto::{CommentResponse, CreateCommentRequest, UpdateCommentRequest},
    domain::services::comment_service::CommentService,
    shared::error::ApiError,
};

#[derive(Clone)]
pub struct CommentRouterState<S: CommentService> {
    pub comment_service: S,
}

pub fn comment_router<S>(comment_service: S) -> Router
where
    S: CommentService + Clone + Send + Sync + 'static,
{
    let state = CommentRouterState { comment_service };

    Router::new()
        .route("/", post(create_comment))
        .route("/post/:post_id", get(get_comments_for_post))
        .route("/:id", get(get_comment))
        .route("/:id", put(update_comment))
        .route("/:id", delete(delete_comment))
        .with_state(state)
}

async fn get_comments_for_post<S>(
    State(state): State<CommentRouterState<S>>,
    Path(post_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError>
where
    S: CommentService,
{
    let comments = state.comment_service.find_by_post(post_id).await?;
    let response = comments
        .into_iter()
        .map(CommentResponse::from)
        .collect::<Vec<_>>();
    Ok(Json(response))
}

async fn get_comment<S>(
    State(state): State<CommentRouterState<S>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError>
where
    S: CommentService,
{
    let comment = state.comment_service.find(id).await?;
    Ok(Json(CommentResponse::from(comment)))
}

async fn create_comment<S>(
    State(state): State<CommentRouterState<S>>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<impl IntoResponse, ApiError>
where
    S: CommentService,
{
    payload.validate()?;
    let comment = state.comment_service.create(payload.into()).await?;
    Ok((StatusCode::CREATED, Json(CommentResponse::from(comment))))
}

async fn update_comment<S>(
    State(state): State<CommentRouterState<S>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCommentRequest>,
) -> Result<impl IntoResponse, ApiError>
where
    S: CommentService,
{
    payload.validate()?;
    let comment = state.comment_service.update(id, payload.into()).await?;
    Ok(Json(CommentResponse::from(comment)))
}

async fn delete_comment<S>(
    State(state): State<CommentRouterState<S>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError>
where
    S: CommentService,
{
    state.comment_service.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
