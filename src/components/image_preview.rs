use dioxus::prelude::*;

use crate::{api, components::Image};

#[component]
pub fn ImagePreview(image: api::Image) -> Element {
    let nav = navigator();
    rsx! {
        div {
            onclick: move |_| { nav.push(format!("/img/{}", image.id)); },
            Image {
                src: image.preview_link.unwrap_or(image.link)
            }
        }
    }
}
