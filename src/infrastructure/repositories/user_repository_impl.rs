use std::sync::Arc;

use async_trait::async_trait;
use diesel::{QueryDsl, RunQueryDsl, prelude::*};
use uuid::Uuid;

use crate::{
    domain::models::user::{NewUser, UpdateUserData, User},
    domain::repositories::UserRepository,
    infrastructure::database::connection::PgPool,
    shared::error::ApiError,
};

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find(&self, _id: Uuid) -> Result<User, ApiError> {
        use crate::infrastructure::database::schema::users::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        users
            .filter(id.eq(id))
            .select(User::as_select())
            .first(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    async fn find_by_email(&self, _email: &str) -> Result<User, ApiError> {
        use crate::infrastructure::database::schema::users::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        users
            .filter(email.eq(email))
            .select(User::as_select())
            .first(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    async fn create(&self, user: User) -> Result<User, ApiError> {
        use crate::infrastructure::database::schema::users::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        let new_user = NewUser {
            username: user.username,
            email: user.email,
            password_hash: user.password_hash,
        };

        diesel::insert_into(users)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    async fn update(&self, user_id: Uuid, user: User) -> Result<User, ApiError> {
        use crate::infrastructure::database::schema::users::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        let update_data = UpdateUserData {
            username: Some(user.username),
            email: Some(user.email),
            password_hash: Some(user.password_hash),
            updated_at: Some(chrono::Local::now().naive_local()),
        };

        diesel::update(users.filter(id.eq(user_id)))
            .set(&update_data)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    async fn delete(&self, user_id: Uuid) -> Result<(), ApiError> {
        use crate::infrastructure::database::schema::users::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        diesel::delete(users.filter(id.eq(user_id)))
            .execute(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl UserRepository for Arc<UserRepositoryImpl> {
    async fn find(&self, id: Uuid) -> Result<User, ApiError> {
        self.as_ref().find(id).await
    }

    async fn find_by_email(&self, email: &str) -> Result<User, ApiError> {
        self.as_ref().find_by_email(email).await
    }

    async fn create(&self, user: User) -> Result<User, ApiError> {
        self.as_ref().create(user).await
    }

    async fn update(&self, user_id: Uuid, user: User) -> Result<User, ApiError> {
        self.as_ref().update(user_id, user).await
    }

    async fn delete(&self, user_id: Uuid) -> Result<(), ApiError> {
        self.as_ref().delete(user_id).await
    }
}