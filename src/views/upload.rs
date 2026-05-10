use std::collections::BTreeSet;

use crate::components::Image;
use dioxus::prelude::*;

#[component]
pub fn Upload() -> Element {
    let nav = navigator();
    let mut image_string = use_signal(String::new);
    let mut tags = use_signal(BTreeSet::<String>::new);
    let mut tag_string = use_signal(String::new);
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
        input {
            value: tag_string,
            id: "upload-tags",
            oninput: move |e| {
                tag_string.set(e.value());
            },
            onkeydown: move |e| {
                if e.code() == Code::Enter {
                    let mut t = tags();
                    t.extend(tag_string().split_ascii_whitespace().map(ToOwned::to_owned));
                    tags.set(t);
                    tag_string.clear();
                }
            },
        }
        ul {
            for tag in tags().iter().cloned() {
                li {
                    "{tag}"
                    button {
                        onclick: move |_| {
                            let mut t = tags();
                            t.remove(&tag);
                            tags.set(t);
                        },
                        "-"
                    }
                }
            }
        }
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

#[post("/upload_img")]
async fn upload(link: String, tags: BTreeSet<String>) -> Result<i64> {
    use crate::server::queries::upload_new_image;

    upload_new_image(link, tags).await
}
