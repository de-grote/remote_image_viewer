use crate::{components::AdminPanel, get_logged_in_user, get_logged_in_user_perms, get_user_by_id};
use dioxus::prelude::*;

#[component]
pub fn Profile(id: i64) -> Element {
    let user = use_server_future(move || get_user_by_id(id))?;
    let Some(user) = user().unwrap()? else {
        return HttpError::not_found("this guy is NOT real")?;
    };
    let current_user = use_server_future(get_logged_in_user)?;
    let current_perms = use_server_future(get_logged_in_user_perms)?;
    rsx! {
        h1 { "Profile of {user.username}" }
        if current_user().unwrap()?.is_some_and(|me| me.id == user.id) {
            details {
                summary { "Account Settings" }
                "TODO that"
            }
            if current_perms().unwrap()?.can_change_admin_settings {
                h2 { "COOL AND EPIC ADMIN PANEL" }
                AdminPanel {}
            }
        }
    }
}
