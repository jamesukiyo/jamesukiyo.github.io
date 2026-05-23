use dioxus::prelude::*;

use super::{
    Route,
    data::{
        ME,
        PROJECTS,
        VERSION,
    },
    keyboard::{
        KEYMAPS,
        use_keyboard_handler,
    },
    scroll::use_scroll_snapping,
};

fn get_nerd_file_info(route: &Route) -> (String, String) {
    // Cannot move out of `route.name` as enum variant `Project` which is behind
    // a shared reference. Move occurs because `name` is of type String - so no
    // Copy trait.
    #[expect(clippy::pattern_type_mismatch)]
    match route {
        Route::Home {} | Route::PageNotFound { .. } => {
            ("portfolio/main.rs".to_owned(), "rust".to_owned())
        },
        Route::Project { name } => {
            let tech = PROJECTS
                .iter()
                .find(|p| p.clean_name() == *name)
                .map_or("rust", |p| p.main_tech_used())
                .to_lowercase();

            let ext = match tech.as_str() {
                "rust" => "rs",
                "c++" => "cpp",
                "sveltekit" => "svelte",
                "vimscript" => "vim",
                "nushell" => "nu",
                _ => &tech,
            };

            // Strip project name extensions and use underscores for file name.
            let clean_name = name
                .replace(['.', '-'], "_")
                .replace("_nvim", "")
                .replace("_vim", "")
                .replace("_rs", "")
                .replace("_github_io", "");

            (format!("portfolio/{clean_name}.{ext}"), tech)
        },
    }
}

#[component]
// Clippy complains about `keydown` if we don't allow absolute_paths.
#[expect(clippy::absolute_paths)]
pub fn Layout() -> Element {
    use_scroll_snapping();
    rsx! {
        div {
            onkeydown: use_keyboard_handler(),
            tabindex: 0,
            autofocus: true,
            style: "outline: none;",
            div { class: "min-h-screen flex flex-col min-w-full",
                Navbar {}
                main { class: "flex flex-col grow max-w-6xl w-full mx-auto border-x-1 border-t-1 border-white/20 bg-[#0f1116]",
                    Outlet::<Route> {}
                }
                Footer {}
            }
        }
    }
}

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { class: "h-12 bg-[#0f1116]"}
        div { class: "fixed top-0 z-99 left-0 right-0 bg-[#16161e] h-12 w-full flex flex-row items-center max-w-6xl px-3 mx-auto border-x-1 border-t-1 border-x-white/20 border-t-white/20",
            // left col
            div { class: "flex items-center",
                Link {
                    class: "flex items-center hover:text-[#A66AA2] hover:scale-150",
                    to: Route::Home {},
                    i { class: "fa-solid fa-house" }
                }
            }
            // center col
            div { class: "flex-1 flex justify-center",
                div { class: "flex items-center gap-x-3 text-sm",
                    i { class: "devicon-vim-plain text-2xl" }
                    div { class: "flex items-center gap-x-4",
                        for k in KEYMAPS {
                            div { class: "flex items-center gap-x-2",
                                kbd { class: "bg-gray-700 px-1 rounded text-md", "{k.key}" }
                                span { class: "text-sm! font-mono", "{k.desc}" }
                            }
                        }
                    }
                }
            }
            // right col
            div { class: "flex flex-row gap-x-4 items-center",
                p { class: "text-sm text-white/90",
                    "v{VERSION}"
                }
                for s in ME.socials {
                    Link { class: "text-white hover:text-[#A66AA2] hover:scale-150",
                        to: "{s.url}",
                        new_tab: true,
                        i { class: "{s.icon}" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Footer() -> Element {
    let curr_route = use_route::<Route>();

    let (filename, filetype) = get_nerd_file_info(&curr_route);
    let filetype_component = {
        let (icon, display) = match filetype.as_str() {
            "lua" => ("lua", "lua"),
            "bash" => ("bash", "bash"),
            "c++" => ("cplusplus", "cpp"),
            "sveltekit" | "svelte" => ("svelte", "svelte"),
            "vimscript" => ("vim", "vim"),
            "nushell" | "nu" => ("nu", "nu"),
            _ => ("rust", "rust"),
        };

        if icon.starts_with("nu") {
            rsx! {
                span { class: "flex items-center",
                    i { class: "fa-solid fa-terminal mr-1 text-sm" }
                    "nu"
                }
            }
        } else {
            rsx! {
                span { class: "flex items-center",
                    i { class: "devicon-{icon}-plain mr-1" }
                    "{display}"
                }
            }
        }
    };

    rsx! {
        div { class: "fixed max-w-6xl bottom-0 left-0 right-0 bg-[#16161e] text-white
                text-md flex z-50 items-center mx-auto",
            // mode
            div { class: "bg-[#A66AA2] text-[#F2EEEB] px-3",
                "NORMAL"
            }
            // branch
            div { class: "bg-[#592D59] text-[#F2EEEB] px-3",
                span { class: "flex items-center",
                    i { class: "devicon-git-plain mr-1" }
                    "master"
                }
            }
            // file name
            div { class: "bg-[#3C2240] text-[#F2EEEB] px-3",
                "{filename}"
            }
            // spacer
            div { class: "flex-grow" }
            // file type and position + copyright
            div { class: "flex items-center",
                div { class: "text-white/30 text-xs mr-2",
                    "Copyright © 2025-present - PlumJam <git@plumj.am>"
                }
                div { class: "bg-[#3C2240] text-[#F2EEEB] px-3 flex items-center",
                    {filetype_component}
                }

                div { class: "bg-[#592D59] text-[#F2EEEB] px-3",
                    "42:12"
                }

                div { class: "bg-[#A66AA2] text-[#F2EEEB] px-3",
                    "100%"
                }
            }
        }
    }
}
