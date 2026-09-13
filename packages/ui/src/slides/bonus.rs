use dioxus::prelude::*;

#[component]
pub fn Bonus() -> Element {
    let slide = include_str!("bonus.rs");
    rsx! {
        section {
            h3 { "Bonus" }
            pre {
                code { class: "fragment language-rust", "data-trim": true, {slide} }
            }
        }
    }
}
