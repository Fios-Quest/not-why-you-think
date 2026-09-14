use crate::common::notes::Notes;
use dioxus::prelude::*;

const RUST_IS_EASY: Asset = asset!("/assets/rust-is-easy.png");
const RUST_IS_EASY_FUNCTION: Asset = asset!("/assets/rust-is-easy-function.png");
const RUST_IS_EASY_LINK: Asset = asset!("/assets/rust-is-easy-link.png");

#[component]
pub fn RustC() -> Element {
    rsx! {
        section {
            section {
                h2 { "rustc" }
                Notes {
                    notes: vec![
                        "RustC is the compiler".into(),
                        "Rust has a reputation for being a difficult language".into(),
                        "But actually, it's by far one of the most hand hold-y".into(),
                        "Cryptic errors are rare".into(),
                        "No mystery linker issues, thank-you C++ that's time I'll never get back".into(),
                        "Best example comes from former GDS-er Tris".into(),
                    ],
                }
            }

            section {
                h3 { "Rust is Easy - Tris Oaten" }
                div {
                    img { width: "600px", src: RUST_IS_EASY }
                }
                div {
                    img { width: "150px", src: RUST_IS_EASY_LINK }
                }
                Notes {
                    notes: vec![
                        "In his video Rust is Easy".into(),
                        "Here's a link there'll be another one in a sec".into(),
                        "Tris write a 'hello, world' function in JavaScript".into(),
                    ],
                }
            }

            section {
                img { width: "900px", src: RUST_IS_EASY_FUNCTION }
                Notes {
                    notes: vec![
                        "The compiler takes him through step by step".into(),
                        "Helpful error by helpful error".into(),
                        "Until he's fully rewritten the function in Rust".into(),
                        "He does nothing but follow the compilers suggestion".into(),
                        "The reputation Rust has as a hard language is _mostly_ out of date".into(),
                    ],
                }
            }

            section {
                img { width: "400px", src: RUST_IS_EASY_LINK }
                Notes {
                    notes: vec![
                        "Here's the link to that video again".into(),
                        "If there's one Rust YouTuber you should watch".into(),
                        "It's Tris".into(),
                        "If there's two... Chris Biscardi is also very good".into(),
                    ],
                }
            }
        }
    }
}
