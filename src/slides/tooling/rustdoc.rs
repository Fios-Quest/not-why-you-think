use dioxus::prelude::*;

const DOC: &str = r#"
/// Adds one to a number
///
/// ```
/// use example::add_one;
///
/// assert_eq!(add_one(1), 3);
/// ```
pub fn add_one(n: u32) -> u32 {
    n + 1
}
"#;

#[component]
pub fn RustDoc() -> Element {
    rsx! {
        section {
            section { h2 { "rustdoc" } }
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
        }
    }
}