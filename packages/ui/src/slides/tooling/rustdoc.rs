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
            }

            section {
                "data-background-image": DOC_IMAGE,
                "data-background-size": "cover",
                "data-background-position": "top left",
            }

            section {
                pre {
                    code {
                        "data-trim": true,
                        {DOC_TEST_ERROR}
                    }
                }
            }
        }
    }
}
