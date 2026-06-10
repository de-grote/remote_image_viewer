use dioxus::prelude::*;

#[component]
pub fn Register() -> Element {
    let nav = navigator();
    rsx! {
        h1 { "Register" }
        form {
            id: "register",
            onsubmit: move |e: FormEvent| async move {
                e.prevent_default();

                let values = e.values();
                if let FormValue::Text(username) = &values[0].1
                    && let FormValue::Text(password) = &values[1].1
                    && let FormValue::Text(password_repeat) = &values[2].1
                    && username.len() > 0
                    && password == password_repeat
                    && password.len() >= 3
                {
                    if let Ok(()) = register(username.clone(), password.clone()).await {
                        nav.go_back();
                    }
                }
                error!("couldn't make account");
                // TODO error message
            },
            label { r#for: "username", "username" }
            input { id: "username", r#type: "text", name: "username" }
            label { r#for: "password", "password" }
            input { id: "password", r#type: "password", name: "password" }
            label { r#for: "password_repeat", "confirm password" }
            input {
                id: "password_repeat",
                r#type: "password",
                name: "password_repeat",
            }
            button { id: "submit", r#type: "submit", name: "submit", "Register" }
        }
    }
}

#[post("/api/register", auth: crate::server::Session)]
async fn register(username: String, password: String) -> Result<()> {
    use crate::server::account::create_account;
    let id = create_account(&username, &password).await?;
    auth.login_user(id);
    Ok(())
}
