use crate::common::notes::Notes;
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

            Notes { notes: vec![
                                                                                                                                                                                                                                                                    "Right, it's memory safe but...".into(),
                                                                                                                                                                                                                                                                ] }
        }

        section {
            section {
                h2 { "Java, JavaScript, Go, PHP, Python, etc" }

                Notes {
                    notes: vec![
                        "Java, JavaScript, Go, PHP, Python".into(),
                        "If it has a garbage collector its safe".into(),
                    ],
                }
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

                Notes {
                    notes: vec![
                        "Ok but its both fast _and_ memory safe".into(),
                        "Yeah, no other crossover there, that is pretty unique to Rust".into(),
                    ],
                }
            }

            section {
                h2 { "Rust is about 4x faster than TypeScript" }

                Notes {
                    notes: vec![
                        "4x faster than TypeScript".into(),
                        "Use TS as I love TS".into(),
                        "Lots of comparisons online".into(),
                        "I have my own anecdote too".into(),
                        "Question is...".into(),
                    ],
                }
            }

            section {
                h2 { "Who cares?" }

                Notes {
                    notes: vec![
                        "Who cares".into(),
                        "Web Servers don't spend much time on compute".into(),
                        "Even when you get a return 4x faster this could be ms".into(),
                        "Could make an argument for scale".into(),
                    ],
                }
            }

            section {
                h2 { "Not a defining feature" }

                Notes { notes: vec![
                                                                                                                                                                                                                                                                                                                                    "Not a defining feature".into()
                                                                                                                                                                                                                                                                                                                                ] }
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

                Notes { notes: vec!["What are the usual reasons (down for fast, right for memory safe)".into()] }
            }

            ItsFast {}
        }

        ItsMemorySafe {}

        ItsBoth {}
    }
}
