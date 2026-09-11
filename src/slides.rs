use dioxus::prelude::*;

#[component]
pub fn Slides() -> Element {
    rsx! {
        div { class: "reveal",
            div { class: "slides",
                section { h2 { "One" } }
                section { h2 { "Two" } }
            }
        }
    }
}
