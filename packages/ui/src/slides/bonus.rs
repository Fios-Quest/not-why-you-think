use crate::common::item::Item;
use crate::common::notes::Notes;
use crate::slides::bonus::recursive::Recursive;
use dioxus::prelude::*;

mod recursive;

const THIS_REPO_LINK: Asset = asset!("/assets/this-repo-link.png");

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

                Notes {
                    notes: vec![
                        "Excellent for system tools".into(),
                        "Similarly great for web servers".into(),
                        "Did you know you can make front ends".into(),
                        "Eagle-eyed might have noticed I used Reveal JS".into(),
                        "Obviously I used Rust".into(),
                    ],
                }
            }

            Recursive {}

            section {
                h3 { "github.com/Fios-Quest/not-why-you-think" }
                img { width: "400px", src: THIS_REPO_LINK }

                Notes {
                    notes: vec![
                        "Thank you so much for listen".into(),
                        "If you're curious you can find the code here".into(),
                        "Any questions".into(),
                    ],
                }
            }
        }
    }
}
