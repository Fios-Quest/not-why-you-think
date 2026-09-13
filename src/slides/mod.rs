mod title;
mod usual_reasons;
mod boring;
mod speed;

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
            }
        }
    }
}
