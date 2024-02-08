use axum::Json;
use axum::http:: StatusCode;

use crate::create_image::CreateImage;
use crate::image_entity:: ImageEntity;


pub async fn post_image(Json(payload):  Json<CreateImage>) -> StatusCode {
   return match ImageEntity::new(payload.path, payload.image) {
       Ok(_) => StatusCode::CREATED,
       Err(err) => {
           eprintln!("Error creating ImageEntity: {:?}", err);
           StatusCode::BAD_REQUEST
       }
   };
}





