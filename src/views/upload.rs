use crate::{
    api::ServerError,
    components::{Image, TagEditor},
};
use dioxus::prelude::*;
use std::collections::BTreeSet;

#[component]
pub fn Upload() -> Element {
    let nav = navigator();
    let mut image_string = use_signal(String::new);
    let tags = use_signal(BTreeSet::<String>::new);

    rsx! {
        h1 { "remote image viewer uploader :tm:" }
        input {
            id: "upload-image",
            oninput: move |e| {
                image_string.set(e.value());
            },
        }
        Image { src: image_string }
        h2 { "tags" }
        TagEditor { tags }
        button {
            onclick: move |_| async move {
                match upload(image_string(), tags()).await {
                    Ok(id) => {
                        nav.push(format!("/img/{}", id));
                    }
                    Err(err) => {
                        error!("{:?}", err);
                    }
                }
            },
            "upload"
        }
    }
}

#[post("/api/upload_img", auth: crate::server::Session)]
async fn upload(link: String, tags: BTreeSet<String>) -> Result<i64> {
    use crate::server::{db, queries::upload_new_image};
    use axum_session_auth::HasPermission;

    let user = auth.current_user.unwrap_or_default();

    if !user.has("upload", &Some(db().await)).await {
        return Err(ServerError::InvalidPerms.into());
    }

    let client = reqwest::Client::new();
    let res = client
        .head(&link)
        .send()
        .await
        .map_err(|_| ServerError::InvalidImageLink)?
        .error_for_status()?;
    let headers = res.headers();
    if !headers
        .get("content-type")
        .and_then(|val| val.to_str().ok())
        .is_some_and(|str| str.starts_with("image"))
    {
        return Err(ServerError::NotAnImage.into());
    }

    upload_new_image(link, tags).await
}
