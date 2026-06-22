use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[derive(Deserialize)]
struct ExecRequest {
    cmd: String,
}

#[derive(Serialize)]
struct ExecResponse {
    stdout: String,
    stderr: String,
}

// INTENTIONALLY VULNERABLE — training target
async fn exec(Json(payload): Json<ExecRequest>) -> Json<ExecResponse> {
    match Command::new("sh").arg("-c").arg(&payload.cmd).output().await {
        Ok(output) => Json(ExecResponse {
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }),
        Err(e) => Json(ExecResponse {
            stdout: String::new(),
            stderr: e.to_string(),
        }),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/exec", post(exec));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    println!("rce target listening on http://0.0.0.0:9000");
    axum::serve(listener, app).await.unwrap();
}
