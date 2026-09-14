use crate::common::item::Item;
use crate::common::notes::Notes;
use dioxus::prelude::*;

#[component]
pub fn Conclusion() -> Element {
    rsx! {
        section {
            h2 { "Conclusion" }
            ol {
                Item { content: "There are no surprises" }
                Item { content: "You can build really fast" }
                Item { content: "The tooling lets you focus on what matters" }
            }

            Notes { notes: vec![
                                                                                                                                                                                                                                                                    "No surprises".into(),
                                                                                                                                                                                                                                                                    "Build fast".into(),
                                                                                                                                                                                                                                                                    "Tooling lets you focus".into(),
                                                                                                                                                                                                                                                                ] }
        }
    }
}
