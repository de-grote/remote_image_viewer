use dioxus::prelude::*;

#[component]
pub fn SearchBar(value: Option<String>) -> Element {
    let nav = navigator();
    let mut input_string = use_signal(|| "".to_string());
    rsx! {
        input {
            id: "searchbar",
            r#type: "text",
            value: value.unwrap_or_default(),
            oninput: move |e| {
                input_string.set(e.data.value());
            },
            onkeydown: move |e| {
                if e.data.clone().code() == Code::Enter {
                    let query_string = if input_string.is_empty() { "" } else { "?tags=" };
                    nav.push(format!("/search{}{}", query_string, input_string));
                }
            },
            onsubmit: move |e| {
                nav.push(format!("/search?tags={}", e.value()));
            },
        }
    }
}
