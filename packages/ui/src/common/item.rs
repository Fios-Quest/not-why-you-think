use dioxus::prelude::*;

#[component]
pub fn Item(content: String) -> Element {
    rsx! {
        li {
            class: "fragment",
            {content}
        }
    }
}