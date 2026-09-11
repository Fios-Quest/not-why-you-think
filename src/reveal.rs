use dioxus::prelude::*;

#[component]
pub fn Reveal() -> Element {
    rsx! {
        Meta { name: "viewport", content:"width=device-width, initial-scale=1.0" }
        document::Link { rel: "stylesheet", href: "reveal/reset.css" }
        document::Link { rel: "stylesheet", href: "reveal/reveal.css" }
        document::Link { rel: "stylesheet", href: "reveal/theme/black.css", id: "theme" }
        document::Link { rel: "stylesheet", href: "reveal/plugin/highlight/monokai.css" }
        document::Script {
            r#type: "module",
            "
            import Reveal from './reveal/reveal.mjs';
		    import Zoom from './reveal/plugin/zoom.mjs';
		    import Notes from './reveal/plugin/notes.mjs';
		    import Search from './reveal/plugin/search.mjs';
		    import Markdown from './reveal/plugin/markdown.mjs';
		    import Highlight from './reveal/plugin/highlight.mjs';

            Reveal.initialize({{
                controls: true,
                progress: true,
                center: true,
                hash: true,

                plugins: [Zoom, Notes, Search, Markdown, Highlight],
            }});
            "
        }
    }
}