
use std::io::Cursor;

use axum::response::Response;
use axum::{extract, Json};
use axum::http::{response, StatusCode};
use image::DynamicImage;

use crate::image_entity::{CreateImage, ImageEntity};

pub async fn post_image(Json(payload):  Json<CreateImage>) -> StatusCode {
   return match ImageEntity::new(payload.path, payload.image) {
       Ok(_) => StatusCode::CREATED,
       Err(err) => {
           eprintln!("Error creating ImageEntity: {:?}", err);
           StatusCode::BAD_REQUEST
       }
   };
}



async fn get_image(img_name:&str) -> DynamicImage {
    let mut path = "imageGuyImages/".to_owned();
    path.push_str(img_name);
    let image: DynamicImage = image::open(path).unwrap();
    image
}

pub async fn image_handler(extract::Path(id): extract::Path<String>) -> Response{
    let image = get_image(&id).await;
    let mut bytes = Vec::new();
    image.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png).unwrap();

    return response::Builder::new()
        .status(StatusCode::OK)
        .header("Content-Type", "image/png")
        .body(bytes.into())
        .unwrap();
}

