use crate::{
    api,
    components::{ImagePreview, SearchBar},
};
use dioxus::prelude::*;

#[component]
pub fn Search(tags: Option<String>) -> Element {
    let tags = tags.unwrap_or_default();
    info!("tags: {:?}", tags);

    let mut tags_signal = use_signal(|| tags.clone());
    if *tags_signal.read() != tags {
        tags_signal.set(tags.clone());
    }

    let images = use_loader(move || {
        let tag_list = tags_signal
            .read()
            .split_ascii_whitespace()
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();

        info!("tag list: {:?}", tag_list);

        search(tag_list)
    })?;

    info!("images: {:?}", images);

    rsx! {
        SearchBar { value: tags }

        div {
            for img in images.iter() {
                ImagePreview { image: img.clone() }
            }
        }
    }
}

#[server]
async fn search(tags: Vec<String>) -> Result<Vec<api::Image>> {
    use crate::server::queries::{search_all, search_tags};

    if tags.is_empty() {
        search_all().await
    } else {
        search_tags(tags).await
    }
}
