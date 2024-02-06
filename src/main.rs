mod handlers;
mod image_entity;

use axum::{Router};

use std::{env, fs};
use axum::routing::{get, post};
use tower_http::cors::{Any, CorsLayer};


#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    create_picture_dir();
    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .route("/",get(hello()))
        .route("/v1/image",post(handlers::post_image) )
        .route("/v1/image:{id}",get(handlers::get_image));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener,app).await.unwrap();
    Ok(())
}

fn hello() -> &'static str {
    return "hello"
}


fn create_picture_dir() {
    fs::create_dir_all("imageGuyImages").expect("ERROR CREATING IMAGE DIRECTORY");
}




