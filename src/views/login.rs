use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let nav = navigator();
    rsx! {
        h1 { "Login" }
        form {
            id: "login",
            onsubmit: move |e: FormEvent| async move {
                e.prevent_default();

                let values = e.values();
                if let FormValue::Text(username) = &values[0].1
                    && let FormValue::Text(password) = &values[1].1
                {
                    if let Ok(()) = login(username.clone(), password.clone()).await {
                        nav.go_back();
                    }
                }
                error!("couldn't log in");
                // TODO error message
            },
            label { r#for: "username", "username" }
            input { id: "username", r#type: "text", name: "username" }
            label { r#for: "password", "password" }
            input { id: "password", r#type: "password", name: "password" }
            button { id: "submit", r#type: "submit", name: "submit", "Login" }
        }
    }
}

#[post("/api/login", auth: crate::server::Session)]
async fn login(username: String, password: String) -> Result<()> {
    use crate::server::account::check_login_details;
    let id = check_login_details(&username, &password).await?;
    auth.login_user(id);
    Ok(())
}

