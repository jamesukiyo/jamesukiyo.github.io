---
title: "Vim, Helix, Zed, and Helix again."
date: "2026-04-29"
edited: "2026-04-29"
slug: "vim-helix-zed-helix-again"
brief: "The trials and tribulations of text editors."
---

## NeoVim

Before Helix I was a [Vim](https://github.com/vim/vim) and
[NeoVim](https://github.com/neovim/neovim) enjoyer. NeoVim seems amazing with
all of its plugins for everything from pomodoro timers to language extensions.
However, I had a few issues with it:

- it is slow
- requires a lot of configuration
- you need a plugin for basically everything
- plugins break somewhat frequently

Let's just say it's the perfect editor for your Vim dotfiles!

With that out of the way, if you are familiar with Helix, you might already see
why I was drawn to it.

## Helix

[Helix](https://github.com/helix-editor/helix) has the following benefits:

- excellent LSP and Treesitter integration/support OOTB
- simple TOML-based configuration
- extremely fast
- multi-cursors (which I now can't live without)
- built-in functionality for live grep, fuzzy finding files, and more
- a better modal editing experience which I will go into more detail about later

My base configuration for Helix is about 50 lines (excluding overriding Deno for
all the J*vaScript derivatives and some weird personal tweaks). I believe most
can get a perfect setup with no more than 15 lines of code in the
[`[editor]`](https://docs.helix-editor.com/editor.html) section of the config.

Just to provide a point of comparison, an average NeoVim config not only needs
to be split between files, but can easily hover around 500-1000 lines of Lua.

### A Better Modal Editing Experience

Vim works on the premise of action -> selection. You first decide what you want
to do, and then you pick what you want to perform the action on.

`keys: diw -> action: delete -> selection: inner word`

Helix does the opposite with selection -> action. You first decide what you will
be performing an action on, then you pick the action to perform.

`keys: miwd -> selection: match inner word -> action: delete`

Now, before you yap "🤓☝️ Well Vim uses less keystrokes so it's faster!!", read
the next sections first:

Not only does Helix operate with a superior model, it's also _always_ in
selection mode. In Vim, selecting text (as you would in Helix) is strictly
reserved for the visual modes, activated by `v`. This means that no matter what
you are doing, you are selecting something:

Finding the next "g" in the document with `fg`? Everything from your cursor
position to that "g" is selected.

Jumping between `w`ords? They get selected as you move.

Want to completely drop the current selection and keep only the cursor? Just
press `;`.

There are loads of other things I have not covered here. Here are a few:

- `Alt-o` and `Alt-i` to expand/shrink selections in syntax nodes
- built-in which-key
- flash.nvim/vim-sneak/leap.nvim with `gw`
- the alignment tool `%`
- multi-cursors

It's all there, with **0** configuration. Maybe I'll write another post soon
going into more detail.

## Zed

Quick disclaimer: Some of the topics here are based around my wish for Zed to
emulate Helix as suggested by their documentation. It probably won't reflect the
average experience.

[Zed](https://github.com/zed-industries/zed) appears to be the hot new thing in
the GUI editor space. It promises to be a high performance editor, with Helix
keymap support and good LSP/Treesitter integration OOTB, which is everything I
wanted.

For anyone interested, they also advertise great AI support (you can
**completely** disable it too, which is fantastic), and multiplayer editing? No
idea why though.

I was on a bit of a GUI spree, trying things like
[GG](https://github.com/gulbanana/gg) (a GUI for jj) to replace my TUI
equivalents like [jjui](https://github.com/idursun/jjui). After seeing a
[post on Xitter](https://x.com/TheGingerBill/status/1910975930756931761) by
Ginger Bill about GUIs and some of the discussions in the replies, I wanted to
give them a real shot. As the title of the article suggests, however, it didn't
go well.

With all that said, I ended up giving it a go...

The obvious place to start is by enabling Helix mode and picking a theme. This
went well and only took a minute. But, it turns out Helix mode which has
supposedly reached
[parity](https://github.com/zed-industries/zed/discussions/33580) with Helix,
isn't even close to the same experience. Maybe my idea of a Helix mode is wrong,
but after using [VSCodeVim](https://github.com/vscodevim/vim) a while ago (which
I remember being quite good?), I expected much better for what it claims to be.

The basic editor configuration experience is miserable. A JSON file with tens of
undocumented options which can only be found mentioned in GitHub issues,
Discord, or by scrolling through the settings menu. Trust me, you have to change
A LOT, to get anything close to a productive keyboard-based setup without any
garbage cluttering your screen.

A quick aside: I'm pro-TOML for most configurations. It is clear, concise, and
simple. If you're working on something, use basic key-value pairs, TOML, or even
KDL for your config file.

The keymap editing is even worse. At the time of writing there is no way to use
a minimal built-in keymap. This discussion perfectly captures most of my
complaints so I won't drone on for much longer:
[zed/discussions/38948](https://github.com/zed-industries/zed/discussions/38948)
TL;DR: The "scopes" for keymaps are abysmal. You don't set it once and forget,
you need to override some, remove overlapping ones, and even then, you might not
get it right until you've iterated and stumbled 5 times. If you want a "custom"
`keybindings.json` be prepared for it to be
[1000 LOC](https://github.com/zed-industries/zed/discussions/38948#:~:text=i%20think%20my%20keybindings.json%20is%20like%201000%20lines.).

Now, configuration aside, how was the editing experience? Actually, quite nice.
It is indeed fast, as they say, and the Helix keymaps that do work as they
should are great. I did notice some visual glitches with menus but they quickly
vanished after a minor version bump.

If you're a VSCoal user, I would recommend giving it a go because my complaints
don't really apply to you. Pick the VSCoal keymap preset and code away.

I didn't care much for the AI features like tab completion and built-in chat
widgets so I won't comment on them.

Using an IDE made me realise something I was missing when using Helix: A good
window splitting system for running processes under the editor, and even a small
pane for a slop agent like Claude Code on the side. This takes us back to Helix,
but better.

## Helix Again (but better)

After wrestling with my Zed configs for a couple of months, I gave up. I deleted
hundreds of lines of config and went back to Helix again. Now with my newfound
enjoyment of an IDE in regards to splitting windows, I decided to level it up a
little.

[Zellij](https://github.com/zellij-org/zellij) is a terminal multiplexer. I've
been using it for over a year now but never really to its full potential. In
short, it runs in your terminal of choice,
[kitty](https://github.com/kovidgoyal/kitty) for me, and lets you do the
following (non-exhaustive):

- provides session persistence so you can ssh to a machine, start a Zellij
  session and it will continue running after you disconnect (for example)
- make splits and divide your workspace in the form of panes and tabs
- create layout presets for easy IDE-like layouts or whatever else

Wonderful foreshadowing, given where this is heading... So yes, I ended up
taking what I liked from Zed and did it with Zellij instead - there's not much
more to it really.

I have a nice setup where I can run terminals alongside my editor via preset
layouts, keep my sessions and switch between them with a plugin like
[zellij-sessionizer](https://github.com/plumj-am/zellij-sessionizer) and
navigate between it all with `Alt-[h/j/k/l]`.

Fullscreen pane? `Alt-f`

New pane? `Alt-g w [r/d/n]`

Move pane? `Alt-g m [h/j/k/l]`

It turns out everything I needed was already there for me before. Still, GUIs
were a fun experiment for a while; sadly they are no match for what a good
terminal-based setup can do.

If you're still reading this, there is hope in the GUI editor space in the form
of [the-editor](https://github.com/misterclayt0n/the-editor) by
[@misterclayt0n](https://x.com/misterclayt0n), go and star it!

The best part of all: I can go back to using and evangelising my beloved Helix
now.
