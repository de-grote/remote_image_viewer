use crate::api::GlobalSettings;
use std::{
    fs,
    sync::{LazyLock, RwLock},
};

const CONFIG_PATH: &str = "config.toml";

static GLOBAL_SETTINGS: LazyLock<RwLock<GlobalSettings>> = LazyLock::new(|| {
    if let Ok(string) = fs::read_to_string(CONFIG_PATH)
        && let Ok(res) = toml::from_str::<GlobalSettings>(&string)
    {
        RwLock::new(res)
    } else {
        RwLock::new(GlobalSettings::default())
    }
});

pub fn global_settings() -> GlobalSettings {
    GLOBAL_SETTINGS
        .read()
        .expect("couldn't get global settings")
        .clone()
}

pub fn set_global_settings(new_settings: GlobalSettings) {
    *GLOBAL_SETTINGS
        .write()
        .expect("couldn't write to global settings") = new_settings;
}

pub fn save_settings() -> dioxus::core::Result<()> {
    let content = toml::to_string_pretty(&global_settings())?;
    fs::write(CONFIG_PATH, content)?;
    Ok(())
}
