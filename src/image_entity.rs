use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use image::DynamicImage;


#[derive(Debug)]
pub struct ImageEntity{
    pub path:String,
    pub image:DynamicImage
}


impl ImageEntity {

    
    pub fn new(path: String,image:String) -> Result<Self,image::ImageError>{

        let bytes =  BASE64_STANDARD.decode(image).unwrap();
        let decoded_image = image::load_from_memory(&bytes)?;
        decoded_image.save(&path).expect("erro ao salvar imagem");

        Ok(ImageEntity{
            path:path.to_string(),
            image:decoded_image
        })

    }

}


