use askama::Template;
use axum::{Router, routing::get, response::IntoResponse, extract::Path};

use crate::HtmlTemplate;

pub fn router() -> Router {
    Router::new()
        .route("/", get(page))
        .route("/:id",get(post))
}

async fn page() -> impl IntoResponse {
    HtmlTemplate(posts())
}

fn posts() -> Posts<'static> {
    Posts{
        posts:vec![
            Post {
                id: 1,
                title: String::from("title"),
                description: String::from("a content section to include an abstract from"),
                date: String::from("01/01/1970"),
                content: "## eeeeee"
            },
            Post {
                id: 2,
                title: String::from("asldjgn"),
                description: String::from("asdgasdjgnabsogjasg"),
                date: String::from("1/2/4"),
                content: "## test"
            }
        ]
    }
}

fn fetch_post_content<'a>(id: u32) -> String {
    use std::fs::read_to_string;
    let parse = read_to_string(format!("{}.md", id as i32 - 1));
    match parse {
        Ok(x) => x,
        _ => String::from("could not fetch post content")
    }
}

/// Given a post id will dynamically return the page for that post
async fn post(Path(id): Path<u32>) -> impl IntoResponse {
    let post = posts().posts.remove(id as usize - 1);
    let post = Post {
        id: 1,
        title: String::from("this is a test"),
        date: String::from("asg"),
        description: String::from("asdfasdfgasdg"),
        content: fetch_post_content(1).as_str()
    };
    HtmlTemplate(post)
}

#[derive(Template)]
#[template(path="pages/posts.html")]
struct Posts<'a>{
    posts: Vec<Post<'a>>
}

#[derive(Template, Clone)]
#[template(path="pages/posts/post.html")]
struct Post<'a> {
    id: i32,
    title: String,
    description: String,
    date: String,
    content: &'a str
}



