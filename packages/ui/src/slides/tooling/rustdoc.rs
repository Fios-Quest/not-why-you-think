use crate::common::notes::Notes;
use dioxus::prelude::*;

const DOC_IMAGE: Asset = asset!("/assets/doc.png");

const DOC: &str = include_str!("../../../../../example/add_one_example/src/lib.rs");

const DOC_TEST_ERROR: &str = r#"
---- example/add_one_example/src/lib.rs - add_one (line 3) stdout ----
Test executable failed (exit status: 101).

stderr:

thread 'main' (4579789) panicked at [...]/doctest_bundle_2024.rs:8:1:
assertion `left == right` failed
  left: 2
 right: 3
"#;

#[component]
pub fn RustDoc() -> Element {
    rsx! {
        section {
            section {
                h2 { "rustdoc" }

                Notes { notes: vec![
                                                                "Rustdoc is where things get _really_ interesting".into(),
                                                            ] }
            }

            section {
                pre {
                    code {
                        class: "language-rust",
                        "data-trim": true,
                        "data-line-numbers": "8-10|1-7|",
                        {DOC}
                    }
                }

                Notes {
                    notes: vec![
                        "Here's out add_one function again".into(),
                        "We add a doc block above with whatever description and examples we want".into(),
                    ],
                }
            }

            section {
                "data-background-image": DOC_IMAGE,
                "data-background-size": "cover",
                "data-background-position": "top left",

                Notes {
                    notes: vec![
                        "When we build the documentation with cargo doc, it produces this".into(),
                        "Again, everyone does this, all documentation looks the same, you get really used to it"
                            .into(),
                        "No unnecessary thought here".into(),
                        "We've got the name of the thing being documented, its signature, a description and an example"
                            .into(),
                        "And, ah, it says adding 1 to 1 give 3... that's not right".into(),
                        "Luckily...".into(),
                    ],
                }
            }

            section {
                pre {
                    code { "data-trim": true, {DOC_TEST_ERROR} }
                }

                Notes {
                    notes: vec![
                        "When we run cargo test, it _also_ runs our example code".into(),
                        "This is why you'll usually see examples written with assertions in them".into(),
                        "It stops your documentation being, or becoming wrong over time".into(),
                        "This even counts for the purposes of coverage so you don't have to duplicate your tests"
                            .into(),
                    ],
                }
            }
        }
    }
}
