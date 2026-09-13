mod boring;
mod speed;
mod title;
mod tooling;
mod usual_reasons;

use dioxus::prelude::*;

#[component]
pub fn Slides() -> Element {
    rsx! {
        div { class: "reveal",
            div { class: "slides",
                title::Title {}
                usual_reasons::TheUsualReasons {}
                boring::ItsBoring {}
                speed::Speed {}
                tooling::Tooling {}
            }
        }
    }
}
