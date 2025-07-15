// News summarizer with 
// backend - rust + axum
// DB - SQLite
// Cron job - tokio-cron-scheduler
// AI summaries - OpenAI API
// Hosting - Fly.io
// Frontend - Flutter

mod db;
mod models;
mod summarizer;
mod routes;
mod scraper;

use axum::{routing::get, Router,};
use std::net::SocketAddr;
use db::init_db;
use dotenvy::dotenv; // allows loading of env files
use crate::routes::{fetch_rss_headlines, get_articles};
use crate::scraper::start_cron_jobs;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // load .env file
    dotenv().ok();

    // Connect to DB and initialize schema
    // Initialize database and schema
    let db = init_db().await;
    // Start cron scheduler (e.g. for article fetching and cleanup)
    start_cron_jobs(Arc::new(db.clone())).await;


    // Manually update summaries (optional, or schedule later) - using gpt
    // summarizer::update_missing_summaries(&db).await.unwrap();

    // Build routes with shared DB state
    // let app = Router::new().route("/", get(fetch_rss_headlines));
    let app = Router::new()
        .route("/", get(fetch_rss_headlines))
        .route("/articles", get(get_articles))
        // .with_state(db.clone());
        .with_state(db);


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


// Axum - handling routing, middleware, and request/response processing. 


// // [Extras]
// // For Adding user 
// #[derive(Debug, Serialize, Deserialize)]
// struct User {
//     id: u32,
//     username: String,
// }

// async fn create_user(Json(payload): Json<User>) -> impl IntoResponse {
//     // In a real application, you'd save this user to a database.
//     println!("Creating user: {:?}", payload);
//     (StatusCode::CREATED, Json(payload))
// }

// async fn get_user(id: u32) -> Result<impl IntoResponse, StatusCode> {
//     // In a real application, you'd fetch this user from a database.
//     if id == 1 {
//         let user = User {
//             id: 1,
//             username: "testuser".to_string(),
//         };
//         Ok((StatusCode::OK, Json(user)))
//     } else {
//         Err(StatusCode::NOT_FOUND)
//     }
// }
