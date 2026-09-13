use dioxus::prelude::*;

mod clippy;
mod rustc;
mod rustdoc;
mod rustfmt;
mod testing;

#[component]
pub fn Tooling() -> Element {
    rsx! {

        section {
            h2 { "Tooling" }
        }

        rustc::RustC {}

        testing::Testing {}

        rustfmt::RustFmt {}

        clippy::Clippy {}

        rustdoc::RustDoc {}
    }
}
