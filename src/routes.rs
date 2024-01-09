use askama::Template;
use axum::{
    Router,
    routing::get,
    response::IntoResponse
};
use crate::*;

// Top level subpages
mod posts;
mod contact;

pub fn router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .nest("/posts", posts::router())
        .nest("/contact", contact::router())
}

async fn hello_world() -> impl IntoResponse {
    HtmlTemplate(HomeTemplate)
}

#[derive(Template)]
#[template(path = "pages/index.html")]
struct HomeTemplate;