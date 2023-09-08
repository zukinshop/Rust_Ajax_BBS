use actix_multipart::form::{text::Text,MultipartForm};

#[derive(MultipartForm)]
pub struct Report{
    pub report:Text<String>,
}
