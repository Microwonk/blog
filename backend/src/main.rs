use anyhow::Context;
use microblog::{
    auth_handler, comments_handler, logs_handler, media_handler, post_handler, rss_handler,
    user_handler, ServerState,
};
use sqlx::postgres::PgPoolOptions;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::{delete, get, post},
    Router,
};

use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .context("Failed to connect to Postgres")?;

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let secret_key = std::env::var("SECRET_KEY").context("SECRET_KEY must be set")?;

    let state = ServerState { pool, secret_key };

    let router = Router::new()
        .merge(unauthenticated_routes(state.clone()))
        .merge(authenticated_routes(state.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await?;

    Ok(())
}

fn unauthenticated_routes(state: ServerState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin([
            "https://blog.nicolas-frey.com"
                .parse::<HeaderValue>()
                .unwrap(),
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:3000".parse::<HeaderValue>().unwrap(),
        ])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    Router::new()
        // get token
        .route("/login", post(auth_handler::login))
        // create user, get token
        .route("/register", post(auth_handler::register))
        // get media from db with media_id
        .route("/media/:media_id", get(media_handler::get_media))
        // get single post from slug
        .route("/post/:slug", get(post_handler::get_post))
        // get all comments from a post
        .route(
            "/post/:id/comments",
            get(comments_handler::get_comments_of_post),
        )
        // get all comments from a post, as a tree structure
        .route(
            "/post/:id/comments/tree",
            get(comments_handler::get_comments_of_post_tree),
        )
        // get all posts from user with path
        .route("/user/:id/posts", get(post_handler::get_posts_by_user))
        // get all posts
        .route("/posts", get(post_handler::get_all_posts))
        // get upload
        .route("/upload/:id", get(media_handler::get_upload))
        // test route
        .route("/rss", get(rss_handler::rss))
        .with_state(state)
        .layer(cors)
}

fn authenticated_routes(state: ServerState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin([
            "https://blog.nicolas-frey.com"
                .parse::<HeaderValue>()
                .unwrap(),
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:3000".parse::<HeaderValue>().unwrap(),
        ])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    Router::new()
        // # ADMIN ROUTES
        // upload media (with file upload)
        .route("/post/:post_id/upload", post(media_handler::upload))
        // get every single media
        .route("/media", get(media_handler::get_all_media))
        // get every single media from a post
        .route(
            "/post/:post_id/media",
            get(media_handler::get_all_media_by_post),
        )
        // delete media
        .route("/upload/:id", delete(media_handler::delete_media))
        // create a post (if admin)
        .route("/user/post", post(post_handler::create_post))
        // once post is created, can be updated, fetched and deleted
        .route(
            "/user/post/:id",
            delete(post_handler::delete_post)
                .put(post_handler::update_post)
                .get(post_handler::get_post_by_id),
        )
        .route("/user/post/:id/release", post(post_handler::release_post))
        .route(
            "/user/post/:id/unrelease",
            post(post_handler::unrelease_post),
        )
        // get all posts from a user by their identity (token)
        .route("/user/posts", get(post_handler::get_posts_by_identity))
        // get all users
        .route("/users", get(user_handler::get_all_users))
        // get all users that are admins
        .route("/users/admins", get(user_handler::get_all_admin_users))
        // # AUTHENTICATED ROUTES
        // check if the user is an admin for frontend purposes
        .route("/user/admin", get(user_handler::is_admin))
        // get profile and update profile (user)
        .route(
            "/profile",
            get(user_handler::get_profile).put(user_handler::change_profile),
        )
        .route(
            "/user/:id",
            delete(user_handler::delete_user).put(user_handler::update_user),
        )
        // post a new comment on a post
        .route("/post/:id/comment", post(comments_handler::create_comment))
        // delete comment (also edit in the future if needed)
        .route("/comment/:id", delete(comments_handler::delete_comment))
        // get all logs
        .route("/admin/logs", get(logs_handler::get_all_logs))
        // auth middleware
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_handler::auth_guard,
        ))
        .with_state(state)
        .layer(cors)
}
