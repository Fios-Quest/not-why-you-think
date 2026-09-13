use dioxus::prelude::*;
use crate::common::item::Item;

#[component]
pub fn Speed() -> Element {
    rsx! {
        section {
            section { h2 { "Speed" } }

            section { h2 { "Less faff" } }

            section {
                h2 { "New TypeScript Project" }
                ol {
                    Item { content: "Install and configure TypeScript", }
                    Item { content: "Install and configure a linter", }
                    Item { content: "Install and configure a style checker", }
                    Item { content: "Install and configure a testing framework", }
                    Item { content: "Fiddle with all the configurations", }
                    Item { content: "Start working on the project" }
                }
            }

            section {
                h2 { "New Rust Project" }
                ol {
                    Item { content: "`cargo new`" }
                    Item { content: "Start working on the project"}
                }
            }
        }
    }
}
