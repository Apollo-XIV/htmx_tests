use tower_http::services::ServeDir;


pub fn service() -> ServeDir {
    let asset_path = std::env::current_dir().unwrap();
    ServeDir::new(format!("{}/assets", asset_path.to_str().unwrap()))
}