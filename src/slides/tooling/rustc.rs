use dioxus::prelude::*;

const RUST_IS_EASY: Asset = asset!("/assets/rust-is-easy.png");
const RUST_IS_EASY_FUNCTION: Asset = asset!("/assets/rust-is-easy-function.png");
const RUST_IS_EASY_LINK: Asset = asset!("/assets/rust-is-easy-link.png");

#[component]
pub fn RustC() -> Element {
    rsx! {
        section {
            section { h2 { "rustc" } }

            section { h3 { "Rust is Easy - Tris Oaten" } }

            section {
                img {
                    width: "900px",
                    src: RUST_IS_EASY,
                }
            }

            section {
                img {
                    width: "900px",
                    src: RUST_IS_EASY_FUNCTION,
                }
            }

            section  {
                img {
                    width: "400px",
                    src: RUST_IS_EASY_LINK,
                }
            }
        }
    }
}