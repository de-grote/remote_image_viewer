use crate::{
    api::ImageWithTags,
    components::{Image, TagEditor},
};
use dioxus::prelude::*;
use std::collections::BTreeSet;

#[component]
pub fn ImageView(id: i64) -> Element {
    let image = use_loader(move || get_image(id))?;
    let nav = navigator();
    let mut edit_mode = use_signal(|| false);
    let mut tags = use_signal(|| image.read().tags.iter().cloned().collect());
    let new_tags = use_signal(|| tags());

    rsx! {
        "{id}"
        Image { src: image.read().image.link.clone() }
        if edit_mode() {
            TagEditor { tags: new_tags }
            button {
                onclick: move |_| async move {
                    if new_tags() == tags() {
                        edit_mode.toggle();
                        return;
                    }
                    let (added, removed) = tags_to_diff(new_tags(), tags());
                    match edit_tags(id, added, removed).await {
                        Ok(()) => {
                            tags.set(new_tags());
                            edit_mode.toggle();
                        }
                        Err(err) => error!("could not update tags {:?}", err),
                    }
                },
                "save changes"
            }
        } else {
            ol {
                for tag in tags() {
                    li {
                        onclick: move |_| {
                            nav.push(format!("/search?tags={}", tag));
                        },
                        "{tag}"
                    }
                }
            }
            button {
                onclick: move |_| {
                    edit_mode.toggle();
                },
                "edit"
            }
        }
    }
}

fn tags_to_diff(
    new: BTreeSet<String>,
    old: BTreeSet<String>,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let added = new.difference(&old).cloned().collect();
    let removed = old.difference(&new).cloned().collect();

    (added, removed)
}

#[server]
async fn get_image(id: i64) -> Result<ImageWithTags> {
    use crate::server::queries::{get_image, get_tags};

    let (image, tags) = tokio::try_join!(get_image(id), get_tags(id))?;

    Ok(ImageWithTags { image, tags })
}

#[server]
async fn edit_tags(
    id: i64,
    added_tags: BTreeSet<String>,
    removed_tags: BTreeSet<String>,
) -> Result<()> {
    use crate::server::queries::update_tags;

    let (added, removed) = tags_to_diff(added_tags, removed_tags);

    update_tags(id, added, removed).await?;
    Ok(())
}
