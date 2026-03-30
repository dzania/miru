# Getting Started

A quick guide to get up and running with miru.

## Installation

Install directly from the repository using Cargo:

```bash
cargo install --path .
```

Or build a local binary without installing:

```bash
cargo build --release
./target/release/miru
```

## Opening Files

Pass one or more markdown files as arguments to open them as tabs:

```bash
miru README.md CHANGELOG.md notes.md
```

Or run miru without arguments to browse all `.md` files in the
current directory tree:

```bash
miru
```

## Navigation

miru uses vim-style keybindings throughout:

- `j` / `k` — scroll down or up one line
- `d` / `u` — jump half a page at a time
- `g` — jump to the very top of the document
- `G` — jump to the very bottom

## Searching

Press `/` to open the search bar at the bottom of the screen.
Type your query and results are highlighted live across the document.
Press `Enter` to confirm and use `n` / `N` to cycle through matches.

## Multiple Files

Open additional files at any time with `Ctrl+T`. Switch between open
tabs using `h` and `l`. Close the current tab with `w`.

## Headings Index

Press `Tab` to open the headings index — an outline of every heading
in the current document. Navigate with `j` / `k` and press `Enter`
to jump directly to any section.
