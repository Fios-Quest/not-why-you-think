use dioxus::html::a::content;
use dioxus::prelude::*;
use crate::common::item::Item;
use crate::slides::bonus::recursive::Recursive;

mod recursive;

#[component]
pub fn Bonus() -> Element {

    rsx! {
        section {
            section {
                h3 { "What can you do" }

                ul {
                    Item { content: "System Tools" }
                    Item { content: "Web Servers" }
                    Item { content: "Web Sites" }
                }
            }

            Recursive {}
        }
    }
}
