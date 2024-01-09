use askama::Template;
use axum::{Router, routing::get, response::IntoResponse};

use crate::HtmlTemplate;

pub fn router() -> Router {
    Router::new()
        .route("/", get(page))
}

async fn page() -> impl IntoResponse {
    HtmlTemplate(ContactPage)
}

#[derive(Template)]
#[template(path="pages/contact.html")]
struct ContactPage;