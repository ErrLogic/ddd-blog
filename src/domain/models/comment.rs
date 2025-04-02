use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::infrastructure::database::schema::comments;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = comments)]
pub struct Comment {
    pub id: Uuid,
    pub content: String,
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = comments)]
pub struct NewComment {
    pub content: String,
    pub post_id: Uuid,
    pub author_id: Uuid,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = comments)]
pub struct UpdateCommentData {
    pub content: Option<String>,
    #[diesel(column_name = "updated_at")]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateComment {
    #[validate(length(min = 1))]
    pub content: String,
    pub post_id: Uuid,
    pub author_id: Uuid,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateComment {
    #[validate(length(min = 1))]
    pub content: Option<String>,
}
