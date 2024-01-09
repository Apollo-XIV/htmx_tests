use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::Router;
use htmx_tests::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "with_axum_htmx_askama=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
 
    info!("initializing router...");
    
    let router = Router::new()
        .nest_service("/assets", assets::service()) // assets directory
        .nest("/", routes::router()) // root
        ; 

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
 
