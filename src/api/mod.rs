use serde::{Deserialize, Serialize};


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
