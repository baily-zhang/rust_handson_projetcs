use axum::{Json, Router, routing::get};
use serde::Serialize;
use std::net::SocketAddr;
use tokio::net::TcpListener; // ✅ 新版本推荐用 tokio 的 TcpListener

#[derive(Serialize)]
struct Post {
    title: String,
    date: String,
    summary: String,
}

async fn get_posts() -> Json<Vec<Post>> {
    let posts = vec![
        Post {
            title: "Hello Rust + Next.js".to_string(),
            date: "2025-05-22".to_string(),
            summary: "A blog built with Axum and Tailwind.".to_string(),
        },
        Post {
            title: "Deploying to Vercel".to_string(),
            date: "2025-05-20".to_string(),
            summary: "Learn how to deploy your app in minutes.".to_string(),
        },
    ];

    Json(posts)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/posts", get(get_posts));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Axum running at http://{}", addr);

    // ✅ 使用 axum::serve + TcpListener 而非手动 hyper Server
    axum::serve(listener, app).await.unwrap();
}
