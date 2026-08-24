use std::sync::OnceLock;

use dioxus::prelude::*;
use libs::data::{
    LOGO_NO_BG,
    ME,
};
use pulldown_cmark::{
    Parser,
    html,
};

use super::Route;

static POSTS: &[&str] = &[
    include_str!("posts/is-this-thing-on.md"),
    include_str!("posts/maybe-nix-does-fix-everything.md"),
    include_str!("posts/vim-helix-zed-helix-again.md"),
];

#[derive(Clone)]
pub struct Post {
    pub title:   String,
    pub date:    String,
    pub edited:  String,
    pub slug:    String,
    pub brief:   String,
    pub author:  String,
    pub content: String,
}

impl Post {
    fn from_markdown(markdown: &str) -> Option<Self> {
        let (frontmatter, content) = parse_frontmatter(markdown)?;
        Some(Self {
            title:   frontmatter.get("title")?.as_str()?.to_owned(),
            date:    frontmatter.get("date")?.as_str()?.to_owned(),
            edited:  frontmatter.get("edited")?.as_str()?.to_owned(),
            slug:    frontmatter.get("slug")?.as_str()?.to_owned(),
            brief:   frontmatter.get("brief")?.as_str()?.to_owned(),
            author:  String::from(ME.name),
            content: content.to_owned(),
        })
    }
}

// TODO: Handle missing details. Currently it causes all blogs to disappear.
/// Extract blog details from frontmatter in `yaml` format.
fn parse_frontmatter(markdown: &str) -> Option<(serde_yaml_bw::Value, &str)> {
    let markdown = markdown.strip_prefix("---")?;
    let (frontmatter, content) = markdown.split_once("\n---\n")?;
    let frontmatter = serde_yaml_bw::from_str(frontmatter).ok()?;
    Some((frontmatter, content.trim()))
}

#[component]
fn MarkdownContent(content: String) -> Element {
    let parser = Parser::new(&content);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Apply custom styling to HTML elements.
    let styled_html = html_output
        // Style paragraphs.
        .replace("<p>", "<p class=\"text-lg text-fg mb-4\">")
        // Style headings with # prefix.
        .replace("<h1>", "<h1 class=\"text-xl font-semibold text-fg mb-4 mt-6\"># ")
        .replace("<h2>", "<h2 class=\"text-xl font-semibold text-fg mb-4 mt-6\">## ")
        .replace("<h3>", "<h3 class=\"text-xl font-semibold text-fg mb-4 mt-6\">### ")
        .replace("<h4>", "<h4 class=\"text-xl font-semibold text-fg mb-4 mt-6\">#### ")
        .replace("<h5>", "<h5 class=\"text-xl font-semibold text-fg mb-4 mt-6\">##### ")
        .replace("<h6>", "<h6 class=\"text-xl font-semibold text-fg mb-4 mt-6\">###### ")
        // Style links.
        .replace("<a ", "<a class=\"text-lg p-0.5 bg-gray-400/30 rounded-md hover:text-purple-light underline decoration-1 opacity-90 hover:opacity-100\" ")
        // Style inline code.
        .replace("<code>", "<code class=\"text-md bg-gray-400/30 rounded-md text-purple-light px-1 py-0.5 font-mono\">")
        // Style code blocks.
        .replace("<pre><code>", "<pre class=\"text-md whitespace-pre-wrap bg-bg text-purple-light p-4 overflow-x-auto border border-purple font-mono m-0 mb-2\"><code>")
        // Style images.
        .replace("<img ", "<div class=\"mb-2 py-2\"><img class=\"max-w-3/4 max-h-96 h-auto\" ")
        // Style lists.
        .replace("<ul>", "<ul class=\"text-lg list-disc list-outside mb-6 text-fg ml-4\">")
        .replace("<ol>", "<ol class=\"text-lg list-decimal list-outside mb-6 text-fg ml-4\">")
        .replace("<li>", "<li class=\"text-lg mb-2\">")
        .replace("<hr />", "<hr class=\"opacity-20 mb-6\"/>")
        .replace(" />", " /></div>");

    rsx! {
        div {
            dangerous_inner_html: "{styled_html}"
        }
    }
}

fn posts() -> &'static [Post] {
    static PARSED: OnceLock<Vec<Post>> = OnceLock::new();
    PARSED.get_or_init(|| {
        let mut posts: Vec<_> = POSTS
            .iter()
            .filter_map(|&md| Post::from_markdown(md))
            .collect();
        posts.sort_unstable_by(|a, b| b.date.cmp(&a.date));
        posts
    })
}

fn find_post_with_index(slug: &str) -> Option<(&'static Post, usize)> {
    posts()
        .iter()
        .enumerate()
        .find(|&(_, p)| p.slug == slug)
        .map(|(i, p)| (p, i))
}

#[component]
pub fn BlogList() -> Element {
    rsx! {
        div { class: "w-full py-6",
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 px-4 lg:px-0",
                for (i, post) in posts().iter().enumerate() {
                    Link {
                        to: Route::BlogPost { slug: post.slug.clone() },
                        div { class: "border border-fg p-4 hover:bg-purple-light transition-scale hover:scale-105 lg:hover:scale-110 duration-100 hover:z-10",
                            div { class: "mb-1",
                                div { class: "flex justify-between",
                                    h1 { class: "text-xl font-semibold text-fg mb-3",
                                        "{post.title}"
                                    }
                                    p { class: "text-fg opacity-60",
                                        "#{(posts().len() - i).to_string()}"
                                    }
                                }
                                p { class: "text-fg opacity-70 mb-3",
                                    "{post.brief}"
                                }
                                p { class: "text-fg opacity-60",
                                    "{post.date}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn BlogPost(slug: String) -> Element {
    match find_post_with_index(&slug) {
        Some((post, index)) => {
            rsx! {
                div { class: "flex flex-col py-6",
                    div { class: "border border-fg p-4 relative",
                        div { class: "flex justify-between items-start",
                            h1 { class: "text-3xl font-bold text-fg mb-3",
                                "{post.title}"
                            }
                            p { class: "text-lg text-fg opacity-60",
                                "#{posts().len() - index}"
                            }
                        }
                        div { class: "text-md text-fg flex items-end justify-between",
                            p { class: "opacity-60 flex flex-col sm:flex-row w-full sm:items-center",
                                "{post.date} "
                                span { class: "text-xs opacity-40 sm:ml-2",
                                    "(last edit: {post.edited})"
                                }
                            }
                            span { class: "group flex items-center",
                                img { class: "w-4 mr-2 group-hover:scale-400 group-hover:translate-x-[-30px] transition-all duration-300 animate-bounce",
                                    src: LOGO_NO_BG,
                                    alt: "{ME.name}"
                                }
                                span { class: "text-purple-light text-sm sm:text-base mr-4",
                                    "{post.author}"
                                }
                            }
                        }
                    }
                    article { class: "border-x border-fg px-4 py-6",
                        MarkdownContent { content: &post.content }
                        Credits {}
                    }
                    div { class: "group border border-fg p-4 hover:bg-purple-light hover:scale-105 transition-all duration-100",
                        Link { class: "text-fg",
                            to: Route::Blog {},
                            "← Back to the blog list."
                        }
                    }
                }
            }
        },
        None => {
            rsx! {
                div { class: "flex flex-col py-6",
                    div { class: "border border-fg p-4 relative",
                        div { class: "flex justify-between items-start",
                            h1 { class: "text-3xl font-bold text-fg mb-3",
                                "Post Not Found :("
                            }
                        }
                    }
                    article { class: "border-x border-fg px-4 py-6",
                        "This post doesn't exist..."
                    }
                    div { class: "group border border-fg p-4 hover:bg-purple-light hover:scale-105 transition-all duration-100",
                        Link { class: "text-fg",
                            to: Route::Blog {},
                            "← Back to the blog list."
                        }
                    }
                }
            }
        },
    }
}

#[component]
fn Credits() -> Element {
    rsx! {
        div {class: "mt-8 w-full",
        hr { class: "opacity-20" }
            p { class: "mt-8 mb-4 flex flex-col",
                "You can find me on..."
                // TODO: I really need to stop repeating this everywhere.
                p { class: "mt-2",
                    "Xitter: "
                    Link { class: "ml-2 hover:text-purple-light opacity-80 hover:opacity-100",
                        to: "https://x.com/plumj_am",
                        "@plumj_am"
                    }
                }
                p { "GitHub: "
                    Link { class: "ml-2 hover:text-purple-light opacity-80 hover:opacity-100",
                        to: "https://github.com/plumj-am",
                        "plumj-am"
                    }
                }
                p { "Forgejo:"
                    Link { class: "ml-2 hover:text-purple-light opacity-80 hover:opacity-100",
                        to: "https://git.plumj.am/plumjam",
                        "plumjam"
                    }
                }
                p { "Matrix: "
                    Link { class: "ml-2 hover:text-purple-light opacity-80 hover:opacity-100",
                        to: "https://matrix.to/#/@plumjam:plumj.am",
                        "@plumjam:plumj.am"
                    }
                }
            }
        }
    }
}
