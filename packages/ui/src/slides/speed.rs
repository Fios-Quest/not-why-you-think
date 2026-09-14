use crate::common::item::Item;
use crate::common::notes::Notes;
use dioxus::prelude::*;

#[component]
pub fn Speed() -> Element {
    rsx! {
        section {
            section {
                h2 { "Speed" }
            }

            section {
                h2 { "Less faff" }
            }

            section {
                h2 { "New TypeScript Project" }
                ol {
                    Item { content: "Install and configure TypeScript" }
                    Item { content: "Install and configure a linter" }
                    Item { content: "Install and configure a style checker" }
                    Item { content: "Install and configure a testing framework" }
                    Item { content: "Fiddle with all the configurations" }
                    Item { content: "Start working on the project" }
                }

                Notes {
                    notes: vec![
                        "TypeScript".into(),
                        "Linter".into(),
                        "Style Checker".into(),
                        "Testing Framework".into(),
                        "Fiddle".into(),
                        "Get to work".into(),
                        "Takes me an hour".into(),
                    ],
                }
            }

            section {
                h2 { "New Rust Project" }
                ol {
                    Item { content: "`cargo new`" }
                    Item { content: "Start working on the project" }

                    Notes { notes: vec![
                                                                                                                                                                                                                                                                                                                                                                                                    "Cargo new".into(),
                                                                                                                                                                                                                                                                                                                                                                                                    "Get to work".into(),
                                                                                                                                                                                                                                                                                                                                                                                                    "Comes with all tooling".into(),
                                                                                                                                                                                                                                                                                                                                                                                                ] }
                }
            }
        }
    }
}
