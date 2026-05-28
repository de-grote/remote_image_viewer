use dioxus::prelude::*;

use crate::{api, components::Image};

const PREVIEW_CSS: Asset = asset!("/assets/styling/image_preview.css");

#[component]
pub fn ImagePreview(image: api::Image) -> Element {
    let nav = navigator();
    rsx! {
        document::Link { rel: "stylesheet", href: PREVIEW_CSS }

        div {
            class: "preview",
            onclick: move |_| {
                nav.push(format!("/img/{}", image.id));
            },
            Image { src: image.preview_link.unwrap_or(image.link) }
        }
    }
}
