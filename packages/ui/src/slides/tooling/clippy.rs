use crate::common::notes::Notes;
use dioxus::prelude::*;

const BEFORE: &str = r#"
let arr = [0.0, 0.1, 0.2, 0.3];
let length = arr.len();
let total = arr.iter().sum::<f64>();
let average = total / length as f64;
"#;

const WARNING: &str = r#"
warning: casting `usize` to `f64` may cause a loss of precision [...]
  --> packages/ui/src/slides/tooling/clippy.rs:23:27
   |
23 |     let average = total / length as f64;
   |                           ^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#cast_precision_loss
   = note: `-W clippy::cast-precision-loss` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::cast_precision_loss)]`
"#;

const AFTER: &str = r#"
let arr = [0.0, 0.1, 0.2, 0.3];
let length = arr.len();
let total = arr.iter().sum::<f64>();
#[expect(clippy::cast_precision_loss, reason = "accuracy isn't necessary")]
let average = total / length as f64;
"#;

fn _example() {
    let arr = [0.0, 0.1, 0.2, 0.3];
    let length = arr.len();
    let total = arr.iter().sum::<f64>();
    let average = total / length as f64;
    let _ = average;
}

#[component]
pub fn Clippy() -> Element {
    rsx! {
        section {
            section {
                h3 { "Clippy" }
                Notes {
                    notes: vec![
                        "RustC will prevent you from writing invalid Rust".into(),
                        "Clippy, yes, named after _that_ clippy,".into(),
                        "Prevents common mistakes in valid Rust".into(),
                        "Stuff like inefficient use of memory or references".into(),
                        "This is the one tool people often do tweak the config on".into(),
                        "Lots of lint collections".into(),
                        "Pedantic, Nursery are usually off".into(),
                    ],
                }
            }

            section {
                pre {
                    code { class: "language-rust", "data-trim": true, {BEFORE} }
                }
                Notes {
                    notes: vec![
                        "Example of pedantic lints being over the top".into(),
                        "Still use them anyway".into(),
                        "This code takes a collection of floats, sums them, divides by length".into(),
                        "Length is a usize, usually u64".into(),
                        "Float only represents integers up to 2^55".into(),
                        "It will 'work', but may not be exactly right".into(),
                    ],
                }
            }

            section {
                pre {
                    code { class: "language-text", "data-trim": true, {WARNING} }
                }

                Notes {
                    notes: vec![
                        "Clippy picks this up".into(),
                        "Tells us what and where".into(),
                        "Link to more information".into(),
                    ],
                }
            }

            section {
                pre {
                    code { class: "language-rust", "data-trim": true, {AFTER} }
                }

                Notes {
                    notes: vec![
                        "We can override the lint with an attribute".into(),
                        "I think it's important to give a reason".into(),
                        "Shows I've thought about it, made a decision, and written it down".into(),
                    ],
                }
            }
        }
    }
}
