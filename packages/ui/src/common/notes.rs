use dioxus::prelude::*;

#[component]
pub fn Notes(notes: Vec<String>) -> Element {
    rsx! {
        aside { class: "notes",
            ul {
                for note in notes {
                    li { {note} }
                }
            }
        }
    }
}
