use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Post {
    id: &'static str,
    title: &'static str,
    body: &'static str,
}

pub async fn get_post() -> Json<Post> {
    Json(Post {
        id: "welcome-post",
        title: "Stored XSS and CSRF Demo",
        body: "This is a vulnerable training post. Leave a comment below.",
    })
}
