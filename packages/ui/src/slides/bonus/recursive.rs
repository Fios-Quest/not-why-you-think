use dioxus::prelude::*;

#[component]
pub fn Recursive() -> Element {
    let slide = include_str!("recursive.rs");
    rsx! {
        section {
            h3 { "RevealJS in Dioxus" }
            pre {
                code { class: "language-rust", "data-trim": true, {slide} }
            }
        }
    }
}
