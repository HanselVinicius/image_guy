use axum::response::Response;
use axum::extract;
use axum::http::{response, StatusCode};
use image::DynamicImage;
use std::io::Cursor;


async fn get_image(img_name:&str) -> DynamicImage {
    let mut path = "imageGuyImages/".to_owned();
    path.push_str(img_name.replace(":", "").as_str());
    let image: DynamicImage = image::open(path).unwrap();
    image
}

pub async fn get_image_handler(extract::Path(id): extract::Path<String>) -> Response{
    let image = get_image(&id).await;
    let mut bytes = Vec::new();
    image.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png).unwrap();

    return response::Builder::new()
        .status(StatusCode::OK)
        .header("Content-Type", "image/png")
        .body(bytes.into())
        .unwrap();
}