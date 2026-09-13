use dioxus::prelude::*;

#[component]
fn Step(step: String) -> Element {
    rsx! {
        li {
            class: "fragment",
            {step}
        }
    }
}

#[component]
pub fn Speed() -> Element {
    rsx! {
        section {
            section { h2 { "Speed" } }

            section { h2 { "Less faff" } }

            section {
                h2 { "New TypeScript Project" }
                ol {
                    Step { step: "Install and configure TypeScript", }
                    Step { step: "Install and configure a linter", }
                    Step { step: "Install and configure a style checker", }
                    Step { step: "Install and configure a testing framework", }
                    Step { step: "Fiddle with all the configurations", }
                    Step { step: "Start working on the project" }
                }
            }

            section {
                h2 { "New Rust Project" }
                ol {
                    Step { step: "`cargo new`" }
                    Step { step: "Start working on the project"}
                }
            }
        }
    }
}
