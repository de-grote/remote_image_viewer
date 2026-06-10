use crate::{Route, api};
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    let logged_in = use_server_future(get_user_info)?;
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        nav { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Search { tags: None }, "Search" }
            Link { to: Route::Upload {}, "Upload" }

            if let Some(user) = logged_in().unwrap()? {
                Link { to: Route::Home {}, "{user.username}" }
            } else {
                Link { to: Route::Login {}, "Login" }
                Link { to: Route::Register {}, "Register" }
            }
        }

        Outlet::<Route> {}
    }
}

#[server(auth: crate::server::Session)]
async fn get_user_info() -> Result<Option<api::User>> {
    Ok(auth.current_user)
}
