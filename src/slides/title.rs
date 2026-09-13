use dioxus::prelude::*;

#[component]
pub fn Title() -> Element {
    rsx! {
        section {
            h2 { class: "fragment", "Why you should use Rust" }
            h1 { "Not Why You Think" }
        }
    }
}
