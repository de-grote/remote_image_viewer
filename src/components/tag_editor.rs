use dioxus::prelude::*;
use std::collections::BTreeSet;

#[component]
pub fn TagEditor(tags: Signal<BTreeSet<String>>) -> Element {
    let mut tag_string = use_signal(String::new);

    rsx! {
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
    }
}
