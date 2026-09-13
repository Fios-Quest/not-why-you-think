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
            }
        }
    }
}
