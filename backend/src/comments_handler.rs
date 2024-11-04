use std::collections::HashMap;

use axum::*;
use extract::{Path, State};
use http::StatusCode;
use response::IntoResponse;

use crate::{
    created, ok, ApiError, ApiResult, Comment, NewComment, ProcessedComment, ServerState, User,
};

pub async fn create_comment(
    Path(post_id): Path<i32>,
    Extension(identity): Extension<User>,
    State(state): State<ServerState>,
    Json(comment): Json<NewComment>,
) -> ApiResult<impl IntoResponse> {
    let result = sqlx::query_as::<_, Comment>(
        r#"
        INSERT INTO comments (
            author, post, content, replying_to
        )
        VALUES (
            $1, $2, $3, $4
        )
        RETURNING id, author, post, content, replying_to, created_at
        "#,
    )
    .bind(identity.id)
    .bind(post_id)
    .bind(comment.content)
    .bind(comment.replying_to)
    .fetch_one(&state.pool)
    .await;

    match result {
        Ok(comment) => created!(comment),
        Err(e) => Err(ApiError::werr(
            "Error creating comment.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn get_comments_of_post(
    Path(post_id): Path<i32>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    match sqlx::query_as::<_, Comment>("SELECT * FROM comments WHERE post = $1")
        .bind(post_id)
        .fetch_all(&state.pool)
        .await
    {
        Ok(response) => ok!(response),
        Err(e) => Err(ApiError::werr(
            "Error retrieving all comments on post.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

pub async fn get_comments_of_post_tree(
    Path(post_id): Path<i32>,
    State(state): State<ServerState>,
) -> ApiResult<impl IntoResponse> {
    match sqlx::query_as::<_, ProcessedComment>(
        r#"
        SELECT 
            comments.id,
            users.name AS author_name,
            comments.content,
            comments.replying_to,
            comments.created_at
        FROM comments
        JOIN posts ON comments.post = posts.id
        LEFT JOIN users ON comments.author = users.id
        WHERE comments.post = $1
        ORDER BY comments.created_at
        "#,
    )
    .bind(post_id)
    .fetch_all(&state.pool)
    .await
    {
        Ok(response) => {
            ok!(build_comment_tree(response))
        }
        Err(e) => Err(ApiError::werr(
            "Error retrieving all comments on post.",
            StatusCode::BAD_REQUEST,
            e,
        )),
    }
}

fn build_comment_tree(comments: Vec<ProcessedComment>) -> HashMap<String, Vec<ProcessedComment>> {
    let mut comments_by_parents: HashMap<String, Vec<ProcessedComment>> = HashMap::new();
    for comment in comments {
        comments_by_parents
            .entry(comment.replying_to.unwrap_or_default().to_string())
            .or_default()
            .push(comment);
    }
    comments_by_parents
}