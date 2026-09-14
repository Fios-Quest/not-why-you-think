use crate::common::notes::Notes;
use dioxus::prelude::*;

#[component]
pub fn RustFmt() -> Element {
    rsx! {
        section {
            h2 { "rustfmt" }
            p { class: "fragment", "cargo fmt" }

            Notes {
                notes: vec![
                    "Rustfmt is very boring".into(),
                    "Again, in a good way".into(),
                    "You run it with 'cargo fmt'".into(),
                    "It formats your code".into(),
                    "Configured out of the box".into(),
                    "You can change the configuration but no one does".into(),
                    "So all Rust looks the same".into(),
                    "No wondering which version of AirBnB closures your writing".into(),
                ],
            }
        }
    }
}
