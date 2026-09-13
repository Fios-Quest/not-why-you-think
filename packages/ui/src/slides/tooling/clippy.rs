use dioxus::prelude::*;

const BEFORE: &str = r#"
let arr = [0.0, 0.1, 0.2, 0.3];
let average = arr.iter().sum::<f64>() / arr.len() as f64;
"#;

const AFTER: &str = r#"
let arr = [0.0, 0.1, 0.2, 0.3];
#[expect(clippy::cast_precision_loss, reason = "accuracy isn't necessary")]
let average = arr.iter().sum::<f64>() / arr.len() as f64;
"#;

#[component]
pub fn Clippy() -> Element {
    rsx! {
        section {
            section {
                h3 { "Clippy" }
            }

            section {
                pre {
                    code { class: "language-rust", "data-trim": true, {BEFORE} }
                }
            }

            section {
                pre {
                    code { class: "language-rust", "data-trim": true, {AFTER} }
                }
            }
        }
    }
}
