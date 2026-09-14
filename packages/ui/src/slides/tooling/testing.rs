use crate::common::notes::Notes;
use dioxus::prelude::*;

const TEST_EXAMPLE: &str = r#"
pub fn add_one(n: u32) -> u32 {
    n + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        assert_eq!(add_one(1), 2);
    }
}
"#;

#[component]
pub fn Testing() -> Element {
    rsx! {
        section {
            section {
                h3 { "Testing" }

                Notes {
                    notes: vec![
                        "Rust comes with a testing framework".into(),
                        "Its extremely barebones... and that's also a good thing".into(),
                    ],
                }
            }
            section {
                pre {
                    code {
                        class: "language-rust",
                        "data-trim": true,
                        "data-line-numbers": "1-3|5-13|5|9|11|",
                        {TEST_EXAMPLE}
                    }
                }

                Notes {
                    notes: vec![
                        "So when we write tests we write it near the code being tested".into(),
                        "Here's a function that takes a number and adds one".into(),
                        "We put test code into a module which just groups code togehter".into(),
                        "Then we can use conditional compilation to ignore the whole module when not testing"
                            .into(),
                        "Any function marked with this test attribute gets run by the test runner"
                            .into(),
                        "There's a tiny handful of assertion macros".into(),
                        "There are frameworks that offer loads of functionality... I never use them"
                            .into(),
                    ],
                }
            }
        }
    }
}
