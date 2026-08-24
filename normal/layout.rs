use dioxus::prelude::*;
use libs::data::{
    LOGO_NO_BG,
    VERSION,
};

use super::{
    Route,
    theme::ThemeSwitcher,
};

struct NavLink {
    route: Route,
    text:  &'static str,
}

static NAV_LINKS: &[NavLink] = &[
    NavLink {
        route: Route::Home {},
        text:  "HOME",
    },
    NavLink {
        route: Route::Projects {},
        text:  "PROJECTS",
    },
    NavLink {
        route: Route::Blog {},
        text:  "BLOG",
    },
    NavLink {
        route: Route::Contact {},
        text:  "CONTACT",
    },
];

#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "min-h-screen flex flex-col min-w-full",
            Navbar {}
            main { class: "flex flex-col grow max-w-5xl w-full mx-auto",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}

#[component]
fn Navbar() -> Element {
    let curr_route = use_route::<Route>();

    let check_route_active = |nav_route: &Route| -> bool {
        match (nav_route, &curr_route) {
            (&Route::Blog {}, &Route::Blog {} | &Route::BlogPost { .. }) => true,
            _ => nav_route == &curr_route,
        }
    };

    rsx! {
        div { class: "h-16 border-l-1 border-b-1 border-fg w-full max-w-5xl mx-auto relative",
            div { class: "grid grid-cols-4 w-full h-full text-md sm:text-xl md:text-2xl tracking",
                for link in NAV_LINKS {
                    { let is_active = check_route_active(&link.route);
                    rsx! {
                    Link { class: format!(
                            "group flex items-center justify-center w-full h-full border-fg hover:bg-[var(--color-hover)] transition-scale duration-100 hover:scale-120 hover:shadow-[inset_0_0_0_1px_var(--color-fg)] hover:border-0 border-r-1 hover:z-10 {}",
                            if is_active { "text-[var(--color-fg)] bg-[var(--color-active)] hover:bg-[var(--color-active)] text-base-light border-fg border-r-1"} else { "" }
                        ),
                        active_class: "text-[var(--color-fg)] bg-[var(--color-active)] hover:bg-[var(--color-active)] text-base-light border-fg border-r-1",
                        to: link.route.clone(),
                        p { class: if is_active { "sm:scale-150" } else { "sm:group-hover:scale-150 transition-scale duration-100" },
                            "{link.text}"
                        }
                    }}
                }}
            }
            // Theme switcher in bottom-right corner below `Navbar`.
            div { class: "absolute bottom-[-25] right-0",
                ThemeSwitcher {}
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        div { class: "relative group flex items-center justify-center h-18 sm:h-8 border-t-1 border-x-1 border-fg w-full max-w-5xl mx-auto",
            p { class: "sm:text-xs flex flex-col sm:flex-row items-center",
                span { class: "md:group-hover:translate-x-[-20px] transition-translate duration-300",
                    "Copyright © 2025-present"
                }
                img { class: "h-4 px-2 md:group-hover:scale-500 md:group-hover:translate-y-[-40px] transition-translate-y duration-300 animate-bounce",
                    src: LOGO_NO_BG
                }
                span { class: "md:group-hover:translate-x-[20px] transition-translate duration-300",
                    span { class: "text-purple-light pr-2",
                        "PlumJam"
                    }
                "<git@plumj.am>"
                }
            }
            p { class: "absolute text-xs bottom-1 sm:bottom-2 right-1",
                "v{VERSION}"
            }
        }
    }
}
