use crate::{get_logged_in_user, get_user_by_id};
use dioxus::prelude::*;

#[component]
pub fn Profile(id: i64) -> Element {
    let current_user = use_server_future(get_logged_in_user)?;
    let user = use_server_future(move || get_user_by_id(id))?;
    let Some(user) = user().unwrap()? else {
        return HttpError::not_found("this guy is NOT real")?;
    };
    rsx! {
        h1 { "Profile of {user.username}" }
        if current_user().unwrap()?.is_some_and(|me| me.id == user.id) {
            p {"YO THATS YOU"}
        }
    }
}
