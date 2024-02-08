use serde::Deserialize;

#[warn(dead_code)]
#[derive(Debug,Deserialize)]
pub struct CreateImage{
    pub path:String,
    pub image:String
}