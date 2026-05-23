use common::{
    clipboard::toast_script,
    data::{
        LOGO_NO_BG,
        ME,
        PROJECTS,
    },
};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "flex w-full flex-col lg:flex-row gap-8 py-6 items-center lg:items-start",
            Profile {}
            Projects {}
        }
    }
}

#[component]
fn Profile() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6 lg:w-1/2",
            div { class: "text-center lg:text-left",
                img { class: "w-32 mx-auto lg:mx-0 mt-2 mb-4 hover:scale-120 transition-all duration-300",
                    src: LOGO_NO_BG,
                    alt: "{ME.name}"
                }
                h1 { class: "text-3xl font-bold mb-2 text-purple-light",
                    "{ME.name}"
                }
                p { class: "text-fg opacity-70 mb-4",
                    a { class: "hover:opacity-100 transition-opacity",
                        href: "mailto:{ME.email}",
                        "{ME.email}"
                    }
                    button { class: "hover:cursor-pointer text-gray-500 hover:text-gray-700 transition-colors ml-2",
                        onclick: move |_| {
                            let script = toast_script(ME.email, "Email copied to clipboard");
                            let _ = document::eval(&script);
                        },
                        title: "Copy email address",
                        i { class: "fa fa-copy text-sm opacity-80 hover:opacity-100" }
                    }
                }
                hr { class: "opacity-10" }
                div { class: "px-2 md:px-0 max-w-prose",
                    p { class: "text-fg mt-4",
                        "Self-taught software developer since 2024."
                    }
                    p { class: "text-fg mt-4",
                        "Trying to shift my focus away from the web dev world..."
                    }
                    p { class: "text-fg mt-4",
                        {format!("Currently enjoying and learning: {}.", ME.learning_langs.join(", "))}
                    }
                    p { class: "text-fg mt-4 mb-4",
                        "If you're looking for help with a project or a new teammate, shoot me an email! :]"
                    }
                }
                hr { class: "opacity-10" }
            }

            div { class: "flex flex-wrap gap-8 sm:gap-32 px-2 md:px-0 mx-auto lg:mx-0",
                div {
                    h3 { class: "font-semibold mb-2 text-fg", "Languages" }
                    ul { class: "text-sm text-fg opacity-80 space-y-1",
                        for lang in ME.langs {
                            li { "• {lang}" }
                        }
                    }
                }
                div {
                    h3 { class: "font-semibold mb-2 text-fg", "Frameworks" }
                    ul { class: "text-sm text-fg opacity-80 space-y-1",
                        for framework in ME.frameworks {
                            li { "• {framework}" }
                        }
                    }
                }
            }
            hr { class: "opacity-10" }
            div { class: "flex gap-4 justify-center lg:justify-start",
                for social in ME.socials {
                    a { class: "group flex items-center text-fg hover:text-purple-light transition-all duration-100 overflow-hidden p-1",
                        href: "{social.url}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        i { class: "{social.icon} text-xl" }
                        span { class: "ml-2 hidden sm:flex transition-all duration-100 whitespace-nowrap",
                            "{social.name}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Projects() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6 lg:w-1/2 px-4 lg:px-0",
            h2 { class: "text-2xl font-bold text-fg",
                "Projects (5/{PROJECTS.len()})"
            }

            div { class: "grid sm:grid-cols-2 lg:grid-cols-1 gap-4",
                for project in PROJECTS.iter().take(5) {
                    div { class: "border border-fg p-4 hover:bg-[var(--color-purple-light)] transition-scale hover:scale-110 duration-100",
                        div { class: "flex justify-between items-start mb-2",
                            h3 { class: "font-semibold text-[var(--color-fg)]", "{project.name}" }
                            span { class: "text-xs px-2 py-1 bg-green-light text-fg",
                                "{project.project_type.as_str()}"
                            }
                        }
                        p { class: "text-sm text-fg opacity-80 mb-3",
                            "{project.short_desc}"
                        }
                        div { class: "flex justify-between items-center",
                            span { class: "text-xs text-fg opacity-60",
                                "{project.tech_used_str()}"
                            }
                            div { class: "flex gap-2",
                                if let Some(github_url) = project.github_url() {
                                    a { class: "text-fg hover:opacity-70",
                                        href: "{github_url}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        i { class: "fa-brands fa-github" }
                                    }
                                }
                                if let Some(forgejo_url) = project.forgejo_url() {
                                    a { class: "text-fg hover:opacity-70",
                                        href: "{forgejo_url}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        i { class: "fa-brands fa-git-alt" }
                                    }
                                }
                                if let Some(npm_url) = project.npm_url {
                                    a { class: "text-fg hover:opacity-70",
                                        href: "{npm_url}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        i { class: "fa-brands fa-npm" }
                                    }
                                }
                                if let Some(crate_url) = project.crate_url {
                                    a { class: "text-fg hover:opacity-70",
                                        href: "{crate_url}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        i { class: "fa-brands fa-rust" }
                                    }
                                }
                                if let Some(site_url) = project.site_url {
                                    a { class: "text-fg hover:opacity-70",
                                        href: "{site_url}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        i { class: "fa fa-globe" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
