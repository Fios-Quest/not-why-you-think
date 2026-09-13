use dioxus::prelude::*;

#[component]
pub fn ItsFast() -> Element {
    rsx! {
        section {
            h2 { "It's fast!" }
        }

        section {
            h2 { "C++, Zig, C" }
        }

        section {
            h2 { "Assembly" }
        }
    }
}

#[component]
pub fn ItsMemorySafe() -> Element {
    rsx! {
        section {
            h2 { "It's memory safe!" }
        }

        section {
            section {
                h2 { "Java, JavaScript, Go, PHP, Python, etc" }
            }

            ItsFast {}
        }
    }
}

#[component]
pub fn ItsBoth() -> Element {
    rsx! {
        section {
            section {
                h2 { "It's both!" }
            }

            section {
                h2 { "Rust is about 4x faster than TypeScript" }
            }

            section {
                h2 { "Who cares?" }
            }

            section {
                h2 { "Not a defining feature" }
            }
        }
    }
}

#[component]
pub fn TheUsualReasons() -> Element {
    rsx! {
        section {
            section {
                h2 { "The usual reasons?" }
            }

            ItsFast {}
        }

        ItsMemorySafe {}

        ItsBoth {}
    }
}
