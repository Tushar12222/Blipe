use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use serde_derive::Deserialize;
use serde_json::{json, Value};
use std::env;
use std::{collections::VecDeque, i32};
#[derive(Deserialize)]
struct Message {
    job: i32,
}
static mut QUEUE: VecDeque<i32> = VecDeque::new();

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/poll", get(poll))
        .route("/push", post(push));

    async fn poll() -> Json<Value> {
        unsafe {
            if QUEUE.is_empty() {
                return Json(json!({
                    "message": "No jobs in the queue",
                    "job": -1
                }));
            }

            Json(json!({
                "message": "Job poll successfully!",
                "job": QUEUE.pop_front()
            }))
        }
    }

    async fn push(Json(payload): Json<Message>) -> Json<Value> {
        unsafe {
            QUEUE.push_back(payload.job);
        }
        println!("Received");
        Json(json!({
            "success": "Job queued successfully"
        }))
    }
    let args: Vec<String> = env::args().collect();
    println!("Queue running on port {}", args[1]);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args[1]))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
