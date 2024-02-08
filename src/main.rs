mod post_image_handler;
mod image_entity;
mod create_image;
mod get_image_handler;
use axum::Router;

use std::fs;
use axum::routing::{get, post};
use tower_http::cors::{Any, CorsLayer};


#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    create_picture_dir();
    let _cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .route("/v1/image",post(post_image_handler::post_image) )
        .route("/v1/image:{id}",get(get_image_handler::get_image_handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener,app).await.unwrap();
    Ok(())
}



fn create_picture_dir() {
    fs::create_dir_all("imageGuyImages").expect("ERROR CREATING IMAGE DIRECTORY");
}




