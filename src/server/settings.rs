use serde::{Deserialize, Serialize};
use std::{fs, sync::LazyLock};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlobalSettings {
    pub create_unknown_tags: bool,
    pub anyone_can_change_settings: bool,
    pub anyone_can_delete_any_image: bool,
    pub anyone_can_upload: bool,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            create_unknown_tags: true,
            anyone_can_change_settings: false,
            anyone_can_delete_any_image: false,
            anyone_can_upload: true,
        }
    }
}

const CONFIG_PATH: &str = "config.toml";

pub static GLOBAL_SETTINGS: LazyLock<GlobalSettings> = LazyLock::new(|| {
    let Ok(string) = fs::read_to_string(CONFIG_PATH) else {
        return GlobalSettings::default();
    };
    let Ok(res) = toml::from_str::<GlobalSettings>(&string) else {
        return GlobalSettings::default();
    };
    res
});

pub fn save_settings() -> dioxus::core::Result<()> {
    let content = toml::to_string_pretty(&*GLOBAL_SETTINGS)?;
    fs::write(CONFIG_PATH, content)?;
    Ok(())
}
