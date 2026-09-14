use crate::common::item::Item;
use crate::common::notes::Notes;
use dioxus::prelude::*;

#[component]
pub fn Speed() -> Element {
    rsx! {
        section {
            section {
                h2 { "Speed" }

                Notes {
                    notes: vec![
                        "I said I didn't _really_ care that Rust was fast".into(),
                        "I still don't, this is about speed to complete work".into(),
                        "As we've just seen with the TS example...".into(),
                        "Our code can pass all out checks and still be broken".into(),
                        "You might be able to deliver _something_ faster than Rust".into(),
                        "But if it's an embarrassing mess it's not really complete".into(),
                    ],
                }
            }

            section {
                h2 { "Less faff" }

                Notes {
                    notes: vec![
                        "Rust is just less faff".into(),
                        "If it compiles and your tests pass".into(),
                        "Much more confident that you're done".into(),
                        "There's another way its 'speedier too'".into(),
                        "This is something thats a finite amount of time but happens _every_ time, at least to me"
                            .into(),
                    ],
                }
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
                }
                Notes { notes: vec![
                    "Cargo new".into(),
                    "Get to work".into(),
                    "Comes with all tooling".into(),
                ] }
            }
        }
    }
}
