use dioxus::prelude::*;

#[component]
pub fn Recursive() -> Element {
    let slide = include_str!("recursive.rs");
    rsx! {
         section {
            h3 { "Bonus" }
            pre {
                code { class: "fragment language-rust", "data-trim": true, {slide} }
            }
        }
    }
}