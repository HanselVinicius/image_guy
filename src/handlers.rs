
use axum::Json;
use axum::http::StatusCode;
use image::io::Reader as ImageReader;

use crate::image_entity::{CreateImage, ImageEntity};

pub async fn post_image(Json(payload):  Json<CreateImage>) -> StatusCode {
   return match ImageEntity::new(payload.path, payload.image) {
       Ok(_) => StatusCode::CREATED,
       Err(err) => {
           eprintln!("Error creating ImageEntity: {:?}", err);
           StatusCode::BAD_REQUEST
       }
   };

    return StatusCode::INTERNAL_SERVER_ERROR
}


pub async fn get_image(){
    let image = ImageReader::open("imageGuyImages/imag.jpg").unwrap();
    println!("{:?}", image.decode())
}