use dioxus::prelude::*;
use libs::clipboard::toast_script;

use super::{
    Route,
    data::{
        ME,
        PROJECTS,
    },
    lines::{
        HiCursor,
        HiLine,
        Line,
        LineNumbers,
        LineType,
    },
};

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "flex w-full",
            HiLine {}
            LineNumbers { max_lines: 70_i32 }
            div { class: "flex flex-col lg:flex-row justify-between w-full",
                Profile {}
                Projects {}
            }
        }
    }
}

// LEFT SIDE HOME PAGE
#[component]
pub fn Profile() -> Element {
    rsx! {
    div { class: "flex flex-col",
        HiCursor {}
        Line { classes: "w-fit relative",
            type_of: LineType::H1,
            text: "Profile",
        }
            Line {}
            div { class: "flex flex-col items-start",
                img { class: "w-36 h-36 object-fit rounded-sm",
                    src: "{ME.image_url}",
                    alt: "{ME.name}"
                }
            }
            Line {}
            div { class: "line-content",
                 // `\u{00A0}` is non-breaking space character for alignment
                p { "Email: \u{00A0}\u{00A0} {ME.email}" }
                button { class: "hover:cursor-pointer text-gray-500 hover:text-gray-700 transition-colors ml-2",
                    onclick: move |_| {
                        let script = toast_script(ME.email, "Email copied to clipboard");
                        let _ = document::eval(&script);
                    },
                    title: "Copy email address",
                    i { class: "fa fa-copy text-sm opacity-80 hover:opacity-100" }
                }
            }
            Line { type_of: LineType::P, text: "Location: {ME.location}" }
            Line {}
            div { class: "grid grid-cols-2 gap-x-8",
                div { class: "flex flex-col",
                    Line { classes: "mb-[-1px]",
                        type_of: LineType::H2,
                        text: "Languages:",
                    }
                    for l in ME.langs {
                        Line { type_of: LineType::P, text: "- {l}" }
                    }
                }
                div { class: "flex flex-col",
                    Line { type_of: LineType::H2, text: "Scripting:" }
                    for s in ME.scripting {
                        Line { type_of: LineType::P, text: "- {s}" }
                    }
                }
            }
            Line {}
            div { class: "grid grid-cols-2 gap-x-8",
                div { class: "flex flex-col",
                    Line { classes: "mb-[-1px]",
                        type_of: LineType::H2,
                        text: "Frameworks:",
                    }
                    for f in ME.frameworks {
                        Line { type_of: LineType::P, text: "- {f}" }
                    }
                }
                div { class: "flex flex-col",
                    Line { classes: "mb-[-1px]",
                        type_of: LineType::H2,
                        text: "Tools:",
                    }
                    for t in ME.tools {
                        Line { type_of: LineType::P, text: "- {t}" }
                    }
                }
            }

            Line {}
            div { class: "flex flex-col",
                img { class: "max-w-md",
                    src: "https://ghchart.rshah.org/592D59/plumj-am",
                    alt: "GitHub Contribution Chart",
                }
            }
            Line {}
            div { class: "line-content flex flex-row gap-x-6 mt-1",
                for s in ME.socials {
                    Link { class: "flex items-center text-white hover:text-[#A66AA2] hover:scale-110",
                        to: "{s.url}",
                        new_tab: true,
                        i { class: "{s.icon} text-lg h-fit w-fit" }
                        p { class: "ml-2",
                            "{s.name}"
                        }
                    }
                }
            }
            Line {}
        }
    }
}

// RIGHT SIDE HOME PAGE
#[component]
pub fn Projects() -> Element {
    rsx! {
        div { class: "text-xl mr-1",
            Line { classes: "w-fit",
                type_of: LineType::H1,
                text: "Projects ({PROJECTS.len()})",
            }
            Line {}
            div { class: "text-left text-white flex flex-col gap-4",
                for p in PROJECTS {
                    div { class: "group text-white w-xl border-1 border-white rounded-md
                            pt-5 px-4 pb-3 hover:bg-[#3C2240] hover:scale-110 relative",
                        Link { class: "block hover:cursor-pointer",
                            to: Route::Project{ name: p.clean_name() },
                            div { class: "flex flex-row justify-between border-b-1 border-white/20",
                                span { class: "pb-2",
                                    "{p.name} "
                                    span { class: "opacity-0 group-hover:opacity-100 text-[#F2EEEB]/80",
                                        "-- {p.short_desc}"
                                    }
                                }
                                span { class: "text-white/60 text-lg",
                                    "[{p.project_type.as_str()}]"
                                }
                            }
                            div { class: "flex flex-row justify-between items-end",
                                span { class: "pt-2 text-sm text-white/60",
                                    "{p.tech_used_str()}"
                                }
                            }
                        }
                        div { class: "absolute bottom-3 right-4 flex gap-2 items-center",
                            if let Some(github_url) = p.github_url() {
                                Link { class: "opacity-80 hover:opacity-100",
                                    to: "{github_url}",
                                    new_tab: true,
                                    rel: "noopener noreferrer",
                                    i { class: "fa-brands fa-github text-white text-lg" }
                                }
                            }
                            if let Some(forgejo_url) = p.forgejo_url() {
                                Link { class: "opacity-80 hover:opacity-100",
                                    to: "{forgejo_url}",
                                    new_tab: true,
                                    rel: "noopener noreferrer",
                                    i { class: "fa-brands fa-git-alt text-white text-lg" }
                                }
                            }
                            if let Some(npm_url) = p.npm_url {
                                Link { class: "opacity-80 hover:opacity-100",
                                    to: "{npm_url}",
                                    new_tab: true,
                                    rel: "noopener noreferrer",
                                    i { class: "fa-brands fa-npm text-white text-lg" }
                                }
                            }
                            if let Some(crate_url) = p.crate_url {
                                Link { class: "opacity-80 hover:opacity-100",
                                    to: "{crate_url}",
                                    new_tab: true,
                                    rel: "noopener noreferrer",
                                    i { class: "fa-brands fa-rust text-white text-lg" }
                                }
                            }
                            if let Some(site_url) = p.site_url {
                                Link { class: "opacity-80 hover:opacity-100",
                                    to: "{site_url}",
                                    new_tab: true,
                                    rel: "noopener noreferrer",
                                    i { class: "fa fa-globe text-white text-lg" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
