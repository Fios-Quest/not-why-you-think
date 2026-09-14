use crate::common::notes::Notes;
use dioxus::prelude::*;

const FIO: Asset = asset!("/assets/fio.svg");

#[component]
pub fn Title() -> Element {
    rsx! {
        section {

            section {
                h2 { class: "fragment", "The reasons you should try Rust are" }
                h1 { "Not Why You Think" }

                Notes {
                    notes: vec![
                        "Introduce myself".into(),
                        "I'm going to explain why the reasons you should try Rust are not why you think"
                            .into(),
                        "Remind people this was threatened".into(),
                    ],
                }
            }

            section {
                h2 { "Brought to you by Fio's Quest" }
                img { src: FIO, width: 336, height: 186 }

                Notes {
                    notes: vec![
                        "I did make this with my YouTube in mind".into(),
                        "So this might be a video later".into(),
                        "I only spent my time on this, but you guys are I guess my test audience".into(),
                        "Hopefully you enjoy it anyway".into(),
                    ],
                }
            }
        }
    }
}
