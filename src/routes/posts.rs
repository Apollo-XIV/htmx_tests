use std::fs::read_to_string;

use crate::HtmlTemplate;
use askama::Template;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};
use serde::Deserialize;

pub fn router() -> Router {
    Router::new().route("/", get(page)).route("/:id", get(post))
}

async fn page() -> impl IntoResponse {
    let posts = Posts {
        posts: get_posts_metadata()
            .into_iter()
            .map(
                |PostMeta {
                     id,
                     title,
                     description,
                     date,
                 }| Post {
                    id: id.clone(),
                    title,
                    description,
                    date,
                    content: fetch_post_content(&id),
                },
            )
            .collect(),
    };
    HtmlTemplate(posts)
}

fn get_posts_metadata() -> Vec<PostMeta> {
    let target = read_to_string(assets_path(format!("posts/meta.ron"))).unwrap();
    ron::from_str(&target).unwrap()
}

#[derive(Deserialize, Debug)]
struct PostMeta {
    id: String,
    title: String,
    description: String,
    date: u32,
}

fn fetch_post_content(id: &String) -> String {
    let parse = read_to_string(assets_path(format!("posts/{id}.md")));
    match parse {
        Ok(x) => x,
        Err(err) => String::from(format!("{}", err)),
    }
}
// teensy tiny change :3
/// Given a post id will dynamically return the page for that post
async fn post(Path(post_id): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    get_posts_metadata()
        .into_iter()
        .find_map(
            |PostMeta {
                 id,
                 title,
                 description,
                 date,
             }| match post_id == id {
                true => Some(HtmlTemplate(Post {
                    id: id.clone(),
                    title,
                    description,
                    date,
                    content: fetch_post_content(&id),
                })),
                false => None,
            },
        )
        .ok_or(StatusCode::NOT_FOUND)

    // HtmlTemplate(post)
}

#[derive(Template)]
#[template(path = "pages/posts.html")]
struct Posts {
    posts: Vec<Post>,
}

#[derive(Template, Clone)]
#[template(path = "pages/posts/post.html")]
struct Post {
    id: String,
    title: String,
    description: String,
    date: u32,
    content: String,
}

fn assets_path(append: String) -> String {
    let asset_path = std::env::current_dir().unwrap();
    format!("{}/assets/{}", asset_path.to_str().unwrap(), append)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scratch() {
        dbg!(assets_path(format!("posts/meta.ron")));
    }
}
