use serde::{Deserialize, Serialize};
use thiserror::Error;

mod user;
pub use user::{User, Permissions};

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

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GlobalSettings {
    pub create_unknown_tags: bool,
    pub anyone_can_delete_any_image: bool,
    pub anyone_can_upload: bool,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            create_unknown_tags: true,
            anyone_can_delete_any_image: false,
            anyone_can_upload: true,
        }
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Cannot create image with unknown tags")]
    CreateUnknownTag,
    #[error("Invalid image link")]
    InvalidImageLink,
    #[error("Not an image")]
    NotAnImage,
    #[error("Username already exists")]
    UsernameExists,
    #[error("Invalid username or password")]
    FailedLogin,
    #[error("You don't have the permission to perform this action")]
    InvalidPerms,
    #[error("Unknown error")]
    UnknownError,
}
