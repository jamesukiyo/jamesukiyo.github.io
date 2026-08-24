use dioxus::prelude::*;
use libs::{
    clipboard::toast_script,
    data::ME,
};

#[component]
pub fn Contact() -> Element {
    rsx! {
        div { class: "h-full w-full py-6 gap-2 flex flex-col px-4 lg:px-0",
            p { class: "mb-2",
                "Matrix is my preferred way of communicating but feel free to reach out via Email or Xitter too!"
            }
            p { class: "mb-2",
                "Happy to discuss projects, work and anything else. My CV is available upon request."
            }
            p {
                span { i { class: "fa fa-envelope w-4 mr-2" } }
                "Email: \u{00A0}", // Whitespace for alignment.
                Link {
                    to: "mailto:{ME.email}",
                    span { class: "underline ml-2 opacity-70 hover:text-purple-light hover:opacity-100 transition-all duration-100",
                        {ME.email}
                    }
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
            p {
                span { i { class: "fa fa-comment w-4 mr-2" } }
                "Matrix: "
                Link {
                    to: "https://matrix.to/#/@plumjam:plumj.am",
                    new_tab: true,
                    span { class: "underline ml-2 opacity-70 hover:text-purple-light hover:opacity-100 transition-all duration-100",
                        "@plumjam:plumj.am"
                    }
                }
            }
            p {
                span { i { class: "fa-brands fa-twitter w-4 mr-2" } }
                "Xitter: "
                Link {
                    to: "https://x.com/plumj_am",
                    new_tab: true,
                    span { class: "underline ml-2 opacity-70 hover:text-purple-light hover:opacity-100 transition-all duration-100",
                        "@plumj_am"
                    }
                }
            }
            p { class: "mt-8 mb-2",
                "Other links:"
            }
            p {
                span { i { class: "fa-brands fa-github w-4 mr-2" } }
                "GitHub: \u{00A0}" // Whitespace for alignment.
                Link {
                    to: "https://github.com/plumj-am",
                    new_tab: true,
                    span { class: "underline opacity-70 hover:text-purple-light hover:opacity-100 transition-all duration-100",
                        "plumj-am"
                    }
                }
            }
            p {
                span { i { class: "fa-brands fa-git-alt w-4 mr-2" } }
                "Forgejo: "
                Link {
                    to: "https://git.plumj.am/plumjam",
                    new_tab: true,
                    span { class: "underline opacity-70 hover:text-purple-light hover:opacity-100 transition-all duration-100",
                        "plumjam"
                    }
                }
            }
            p {
                span { i { class: "fa-brands fa-npm w-4 mr-2" } }
                "NPM: \u{00A0}\u{00A0}\u{00A0}\u{00A0}" // Whitespace for alignment.
                Link {
                    to: "https://www.npmjs.com/~jamesukiyo",
                    new_tab: true,
                    span { class: "underline opacity-70 hover:text-purple-light hover:opacity-100 transition-all duration-100",
                        "jamesukiyo"
                    }
                }
            }
            p {
                span { i { class: "fa-brands fa-rust w-4 mr-2" } }
                "Crates: \u{00A0}" // Whitespace for alignment.
                Link {
                    to: "https://crates.io/users/plumj-am",
                    new_tab: true,
                    span { class: "underline opacity-70 hover:text-purple-light hover:opacity-100 transition-all duration-100",
                        "plumj-am"
                    }
                }
            }



        }
    }
}
