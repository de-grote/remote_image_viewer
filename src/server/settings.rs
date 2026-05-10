use serde::{Deserialize, Serialize};


#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlobalSettings {
    create_unknown_tags: bool,
}