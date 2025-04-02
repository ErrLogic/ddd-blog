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
    application::dto::user_dto::{CreateUserRequest, UpdateUserRequest, UserResponse},
    domain::services::user_service::UserService,
    shared::error::ApiError,
};

#[derive(Clone)]
pub struct UserRouterState<S: UserService> {
    pub user_service: S,
}

pub fn user_router<S>(user_service: S) -> Router
where
    S: UserService + Clone + Send + Sync + 'static,
{
    let state = UserRouterState { user_service };

    Router::new()
        .route("/", post(create_user))
        .route("/:id", get(get_user))
        .route("/:id", put(update_user))
        .route("/:id", delete(delete_user))
        .route("/email/:email", get(get_user_by_email))
        .with_state(state)
}

async fn get_user<S>(
    State(state): State<UserRouterState<S>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError>
where
    S: UserService,
{
    let user = state.user_service.find(id).await?;
    Ok(Json(UserResponse::from(user)))
}

async fn get_user_by_email<S>(
    State(state): State<UserRouterState<S>>,
    Path(email): Path<String>,
) -> Result<impl IntoResponse, ApiError>
where
    S: UserService,
{
    let user = state.user_service.find_by_email(&email).await?;
    Ok(Json(UserResponse::from(user)))
}

async fn create_user<S>(
    State(state): State<UserRouterState<S>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, ApiError>
where
    S: UserService,
{
    payload.validate()?;
    let user = state.user_service.create(payload.into()).await?;
    Ok((StatusCode::CREATED, Json(UserResponse::from(user))))
}

async fn update_user<S>(
    State(state): State<UserRouterState<S>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, ApiError>
where
    S: UserService,
{
    payload.validate()?;
    let user = state.user_service.update(id, payload.into()).await?;
    Ok(Json(UserResponse::from(user)))
}

async fn delete_user<S>(
    State(state): State<UserRouterState<S>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError>
where
    S: UserService,
{
    state.user_service.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
