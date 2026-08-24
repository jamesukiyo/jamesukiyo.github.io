#![forbid(unsafe_code)]

mod home;
mod keyboard;
mod layouts;
mod lines;
mod project;
mod scroll;

use dioxus::prelude::*;
use home::Home;
use layouts::Layout;
use libs::data;
use project::Project;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
	#[layout(Layout)]
	#[route("/")]
	Home {},
	#[route("/project/:name")]
	Project { name: String },
	#[route("/:..route")]
	PageNotFound { route: Vec<String> },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/gen-tailwind.css") }
        document::Link { rel: "icon", href: asset!("/favicon.ico") }
        document::Stylesheet { href: "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/devicon.min.css" }
        document::Script { src: "https://kit.fontawesome.com/6972f6e365.js", crossorigin: "anonymous", fetchpriority: "high" }
        div {
            Router::<Route> {}
        }
    }
}

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "404" }
    }
}
