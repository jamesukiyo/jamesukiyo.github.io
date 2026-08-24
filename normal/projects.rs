use dioxus::prelude::*;
use libs::data::PROJECTS;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div { class: "w-full py-6",
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 px-4 lg:px-0",
                for project in PROJECTS {
                    div { class: "border border-fg p-4 hover:bg-purple-light transition-scale hover:scale-105 md:hover:scale-110 duration-100 relative hover:z-10",
                        div { class: "flex justify-between items-start mb-2",
                            h3 { class: "font-semibold text-fg", "{project.name}" }
                            span { class: "text-xs px-2 py-1 bg-green-light text-fg",
                                "{project.project_type.as_str()}"
                            }
                        }
                        p { class: "text-sm text-fg opacity-80 mb-3",
                            "{project.short_desc}"
                        }
                        span { class: "text-xs text-fg opacity-60 block mb-3",
                            "{project.tech_used_str()}"
                        }
                        div { class: "absolute bottom-3 right-4 flex gap-2",
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
