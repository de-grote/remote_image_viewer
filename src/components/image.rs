use dioxus::prelude::*;

const IMAGE_CSS: Asset = asset!("/assets/styling/image.css");

#[component]
pub fn Image(src: String) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: IMAGE_CSS }

        img {
            id: "main-image",
            class: "fit-vertical",
            src: "{src}",
            alt: "failed to load image",
        }
    }
}
