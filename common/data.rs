use core::fmt;

use dioxus::prelude::*;

pub const VERSION: &str = env!("APP_VERSION");

pub const LOGO_NO_BG: Asset = asset!("/assets/plumjam-nobg.png");

pub struct SocialLink {
    pub name: &'static str,
    pub url:  &'static str,
    pub icon: &'static str,
}

pub struct Me {
    pub name:              &'static str,
    pub image_url:         &'static str,
    pub email:             &'static str,
    pub location:          &'static str,
    pub langs:             &'static [&'static str],
    pub learning_langs:    &'static [&'static str],
    pub scripting:         &'static [&'static str],
    pub operating_systems: &'static [&'static str],
    pub frameworks:        &'static [&'static str],
    pub tools:             &'static [&'static str],
    pub socials:           &'static [SocialLink],
}

pub static ME: Me = Me {
    name:              "PlumJam",
    image_url:         "https://github.com/plumj-am.png",
    email:             "contact@plumj.am",
    location:          "Poland",
    langs:             &["Rust", "Nix", "TypeScript", "Svelte", "Go"],
    learning_langs:    &["Haskell", "fasm"],
    scripting:         &["Nushell", "Bash", "Python"],
    operating_systems: &["NixOS", "Debian", "Windows >7", "MacOS >14"],
    frameworks:        &[
        "Dioxus (Rust)",
        "Poem (Rust)",
        "Chi (Go)",
        "Astro (Typescript)",
        "SvelteKit (Svelte)",
        "Next.js (Typescript)",
    ],
    tools:             &["Docker", "GitHub Actions", "K3s"],
    socials:           &[
        SocialLink {
            name: "GitHub",
            url:  "https://github.com/plumj-am",
            icon: "fa-brands fa-github",
        },
        SocialLink {
            name: "Forgejo",
            url:  "https://git.plumj.am/plumjam",
            icon: "fa-brands fa-git-alt",
        },
        SocialLink {
            name: "Xitter",
            url:  "https://x.com/plumj_am",
            icon: "fa-brands fa-twitter",
        },
        SocialLink {
            name: "Matrix",
            url:  "https://matrix.to/#/@plumjam:plumj.am",
            icon: "fa fa-comment",
        },
    ],
};

pub enum ProjectType {
    Website,
    CliTool,
    Script,
    Plugin,
    Utility,
    Config,
    WebApp,
    Embedded,
    Automation,
}

impl ProjectType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Website => "Website",
            Self::CliTool => "CLI Tool",
            Self::Script => "Script",
            Self::Plugin => "Plugin",
            Self::Utility => "Utility",
            Self::Config => "Config",
            Self::WebApp => "Web Application",
            Self::Embedded => "Embedded",
            Self::Automation => "Automation",
        }
    }
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub struct ProjectInfo {
    pub name:         &'static str,
    pub short_desc:   &'static str,
    pub long_desc:    Option<&'static str>,
    pub project_type: ProjectType,
    pub tech_used:    &'static [&'static str],
    pub repo:         Option<&'static str>,
    pub site_url:     Option<&'static str>,
    pub npm_url:      Option<&'static str>,
    pub crate_url:    Option<&'static str>,
    pub media:        Option<&'static [&'static str]>,
}

impl ProjectInfo {
    #[must_use]
    pub fn clean_name(&self) -> String {
        self.name
            .trim()
            .to_lowercase()
            .replace(' ', "_")
            .replace('.', "")
    }
    #[must_use]
    pub fn tech_used_str(&self) -> String {
        self.tech_used.join(", ")
    }
    #[must_use]
    pub fn github_url(&self) -> Option<String> {
        self.repo
            .map(|repo| format!("https://github.com/plumj-am/{repo}"))
    }
    #[must_use]
    pub fn forgejo_url(&self) -> Option<String> {
        self.repo
            .map(|repo| format!("https://git.plumj.am/plumjam/{repo}"))
    }
    #[must_use]
    pub fn main_tech_used(&self) -> &str {
        self.tech_used.first().unwrap_or(&"Unknown")
    }
}

pub static PROJECTS: &[ProjectInfo] = &[
    ProjectInfo {
        name:         "Charfreq",
        short_desc:   "The fastest character counter.",
        long_desc:    None,
        project_type: ProjectType::CliTool,
        tech_used:    &["Rust"],
        repo:         Some("charfreq-rs"),
        crate_url:    Some("https://crates.io/crates/charfreq"),
        npm_url:      None,
        site_url:     None,
        media:        None,
    },
    ProjectInfo {
        name:         "fasmfetch",
        short_desc:   "Like fastfetch but a little bit faster.",
        long_desc:    None,
        project_type: ProjectType::Utility,
        tech_used:    &["Assembly", "fasm"],
        repo:         Some("fasmfetch"),
        crate_url:    None,
        npm_url:      None,
        site_url:     None,
        media:        None,
    },
    ProjectInfo {
        name:         "AVR Switchbot",
        short_desc:   "A home-made \"switchbot\".",
        long_desc:    None,
        project_type: ProjectType::Embedded,
        tech_used:    &["Rust (embedded)", "AVR-HAL"],
        repo:         Some("avr-switchbot"),
        crate_url:    None,
        npm_url:      None,
        site_url:     None,
        media:        None,
    },
    ProjectInfo {
        name:         "Gerrit Autosubmit",
        short_desc:   "An autosubmit bot for Gerrit changes.",
        long_desc:    None,
        project_type: ProjectType::Automation,
        tech_used:    &["Rust"],
        repo:         Some("gerrit-autosubmit"),
        crate_url:    Some("https://crates.io/crates/gerrit-autosubmit"),
        npm_url:      None,
        site_url:     None,
        media:        None,
    },
    ProjectInfo {
        name:         "NixOS",
        short_desc:   "PlumJam's NixOS configuration collection.",
        long_desc:    None,
        project_type: ProjectType::Config,
        tech_used:    &["Nix"],
        repo:         Some("nixos"),
        crate_url:    None,
        npm_url:      None,
        site_url:     None,
        media:        None,
    },
    ProjectInfo {
        name:         "plumj.am",
        short_desc:   "This website!",
        long_desc:    None,
        project_type: ProjectType::Website,
        tech_used:    &["Rust", "Dioxus", "Tailwind"],
        repo:         Some("plumj.am"),
        crate_url:    None,
        npm_url:      None,
        site_url:     Some("https://plumj.am"),
        media:        None,
    },
    ProjectInfo {
        name:         "Dr. Radka",
        short_desc:   "Client website.",
        long_desc:    None,
        project_type: ProjectType::Website,
        tech_used:    &["SvelteKit", "Deno", "TypeScript", "Tailwind"],
        repo:         None,
        crate_url:    None,
        npm_url:      None,
        site_url:     Some("https://dr-radka.pl"),
        media:        None,
    },
    ProjectInfo {
        name:         "Sunny",
        short_desc:   "Check the weather from your terminal!",
        long_desc:    None,
        project_type: ProjectType::CliTool,
        tech_used:    &["Rust"],
        repo:         Some("sunny-rs"),
        crate_url:    Some("https://crates.io/crates/sunny-cli"),
        npm_url:      Some("https://www.npmjs.com/package/@jamesukiyo/sunny-cli"),
        site_url:     None,
        media:        None,
    },
    ProjectInfo {
        name:         "Jash",
        short_desc:   "A basic shell.",
        long_desc:    None,
        project_type: ProjectType::CliTool,
        tech_used:    &["Rust"],
        repo:         Some("jash-rs"),
        crate_url:    None,
        npm_url:      None,
        site_url:     None,
        media:        None,
    },
    ProjectInfo {
        name:         "Windows Setup",
        short_desc:   "Up and running with NixOS-WSL in minutes.",
        long_desc:    None,
        project_type: ProjectType::Script,
        tech_used:    &["Bash"],
        repo:         Some("windows-setup"),
        crate_url:    None,
        npm_url:      None,
        site_url:     None,
        media:        None,
    },
    ProjectInfo {
        name:         "Search This",
        short_desc:   "Initiate a web search from NeoVim.",
        long_desc:    None,
        project_type: ProjectType::Plugin,
        tech_used:    &["Lua"],
        repo:         Some("search-this.nvim"),
        crate_url:    None,
        npm_url:      None,
        site_url:     None,
        media:        None,
    },
    ProjectInfo {
        name:         "Server Health Monitor",
        short_desc:   "IOT project for local monitoring.",
        long_desc:    None,
        project_type: ProjectType::Utility,
        tech_used:    &["C++", "Python", "Bash"],
        repo:         Some("server-health-monitor"),
        crate_url:    None,
        npm_url:      None,
        site_url:     None,
        media:        None,
    },
    ProjectInfo {
        name:         "Pausarr",
        short_desc:   "A companion script for Tdarr and Jellyfin.",
        long_desc:    None,
        project_type: ProjectType::Script,
        tech_used:    &["Bash"],
        repo:         Some("pausarr"),
        crate_url:    None,
        npm_url:      None,
        site_url:     None,
        media:        None,
    },
    ProjectInfo {
        name:         "NeoVim",
        short_desc:   "(Old) NeoVim configuration files.",
        long_desc:    None,
        project_type: ProjectType::Config,
        tech_used:    &["Lua"],
        repo:         Some("nvim"),
        crate_url:    None,
        npm_url:      None,
        site_url:     None,
        media:        None,
    },
    ProjectInfo {
        name:         "Nushell",
        short_desc:   "(Old) Nushell configuration files.",
        long_desc:    None,
        project_type: ProjectType::Config,
        tech_used:    &["Nu", "Nushell"],
        repo:         Some("nushell"),
        crate_url:    None,
        npm_url:      None,
        site_url:     None,
        media:        None,
    },
    ProjectInfo {
        name:         "Gocial",
        short_desc:   "The forever unfinished social platform.",
        long_desc:    None,
        project_type: ProjectType::WebApp,
        tech_used:    &["Go", "Chi"],
        repo:         Some("gocial"),
        crate_url:    None,
        npm_url:      None,
        site_url:     None,
        media:        None,
    },
    ProjectInfo {
        name:         "Prepare Release",
        short_desc:   "Automatic version bumping and tagging.",
        long_desc:    None,
        project_type: ProjectType::Script,
        tech_used:    &["Nushell", "Bash"],
        repo:         Some("prepare_release"),
        crate_url:    None,
        npm_url:      None,
        site_url:     None,
        media:        None,
    },
    ProjectInfo {
        name:         "QuickSnip",
        short_desc:   "Instantly create snippets for Vim on the fly.",
        long_desc:    None,
        project_type: ProjectType::Plugin,
        tech_used:    &["Vimscript"],
        repo:         Some("quicksnip.vim"),
        crate_url:    None,
        npm_url:      None,
        site_url:     None,
        media:        None,
    },
];
