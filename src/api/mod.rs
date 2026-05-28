use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Image {
    pub id: i64,
    pub link: String,
    pub preview_link: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ImageWithTags {
    pub image: Image,
    pub tags: Vec<String>,
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Cannot create image with unknown tags")]
    CreateUnknownTag,
    #[error("Invalid image link")]
    InvalidImageLink,
    #[error("Not an image")]
    NotAnImage,
}
