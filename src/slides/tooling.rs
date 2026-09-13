use dioxus::prelude::*;

mod rustc;
mod testing;
mod rustfmt;
mod clippy;
mod rustdoc;

#[component]
pub fn Tooling() -> Element {
    rsx! {

        section { h2 { "Tooling" } }

        rustc::RustC {}

        testing::Testing {}

        rustfmt::RustFmt {}

        clippy::Clippy {}

        rustdoc::RustDoc {}
    }
}