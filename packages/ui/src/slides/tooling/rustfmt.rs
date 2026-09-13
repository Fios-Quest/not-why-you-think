use dioxus::prelude::*;

#[component]
pub fn RustFmt() -> Element {
    rsx! {
        section {
            h2 { "rustfmt" }
            p { class: "fragment", "cargo fmt" }
        }
    }
}
