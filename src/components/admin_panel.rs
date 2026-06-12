use dioxus::prelude::*;

use crate::api::GlobalSettings;

#[component]
pub fn AdminPanel() -> Element {
    let server_settings = use_server_future(get_global_settings)?;
    let mut settings = use_signal(move || server_settings().unwrap().unwrap());
    rsx! {
        details {
            summary { "Global Settings" }
            label { r#for: "create_unknown_tags", "Can uploading create new tags?" }
            input {
                id: "create_unknown_tags",
                r#type: "checkbox",
                checked: settings().create_unknown_tags,
                oninput: move |e| {
                    settings.write().create_unknown_tags = e.checked();
                },
            }
            label { r#for: "anyone_can_upload", "Can not logged in user upload?" }
            input {
                id: "anyone_can_upload",
                r#type: "checkbox",
                checked: settings().anyone_can_upload,
                oninput: move |e| {
                    settings.write().anyone_can_upload = e.checked();
                },
            }
            label { r#for: "anyone_can_delete_any_image", "Can not logged in user delete any image?" }
            input {
                id: "anyone_can_delete_any_image",
                r#type: "checkbox",
                checked: settings().anyone_can_delete_any_image,
                oninput: move |e| {
                    settings.write().anyone_can_delete_any_image = e.checked();
                },
            }
        }
        button {
            onclick: move |_| async move {
                apply_global_settings(settings(), false).await.unwrap();
            },
            "Apply Settings"
        }
        button {
            onclick: move |_| async move {
                apply_global_settings(settings(), true).await.unwrap();
            },
            "Save and Apply Settings"
        }
    }
}

#[server(auth: crate::server::Session)]
async fn get_global_settings() -> Result<GlobalSettings> {
    use crate::server::{account::get_perms_of_user, settings::global_settings};

    let perms = get_perms_of_user(auth.id).await?.unwrap_or_default();
    if !perms.can_change_admin_settings {
        return HttpError::forbidden("cant access admin settings without admin")?;
    }

    Ok(global_settings())
}

#[server(auth: crate::server::Session)]
async fn apply_global_settings(settings: GlobalSettings, save: bool) -> Result<()> {
    use crate::server::{
        account::get_perms_of_user,
        settings::{save_settings, set_global_settings},
    };

    let perms = get_perms_of_user(auth.id).await?.unwrap_or_default();
    if !perms.can_change_admin_settings {
        return HttpError::forbidden("cant access admin settings without admin")?;
    }

    info!("setting global settings to {:?}", settings);

    set_global_settings(settings);
    if save {
        save_settings()?;
    }

    Ok(())
}
