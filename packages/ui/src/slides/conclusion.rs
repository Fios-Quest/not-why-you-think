use crate::common::item::Item;
use dioxus::prelude::*;

#[component]
pub fn Conclusion() -> Element {
    rsx! {
        section {
            h2 { "Conclusion" }
            ol {
                Item { content: "There are no surprises" }
                Item { content: "You can build really fast" }
                Item { content: "The tooling lets you focus on the things that matter" }
            }
        }
    }
}
