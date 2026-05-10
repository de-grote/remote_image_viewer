use crate::{
    api::{self, ImageWithTags},
    components::Image,
};
use dioxus::prelude::*;

#[component]
pub fn ImageView(id: i64) -> Element {
    let image = use_loader(move || get_image(id))?;
    let nav = navigator();

    rsx! {
        "{id}",
        Image { src: image.read().image.link.clone() }
        ol {
            for tag in image.read().tags.clone() {
                li {
                    onclick: move |_| { nav.push(format!("/search?tags={}", tag)); },
                    "{tag}"
                }
            }
        }
    }
}

#[server]
async fn get_image(id: i64) -> Result<api::ImageWithTags> {
    use crate::server::queries::{get_image, get_tags};

    let (image, tags) = tokio::try_join!(get_image(id), get_tags(id))?;

    Ok(ImageWithTags { image, tags })
}
