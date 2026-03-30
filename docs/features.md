# Features

miru is a fast, minimal markdown reader built for daily terminal use.

## Markdown Rendering

Renders standard CommonMark markdown with clean terminal formatting:

- Headings at every level with distinct visual weight
- **Bold**, *italic*, and `inline code` spans
- Fenced code blocks with line numbers and a language label
- Tables with box-drawing borders and aligned columns
- Blockquotes with a leading indicator
- Ordered and unordered lists with proper indentation
- Horizontal rules and blank-line spacing

## File Picker

When launched without arguments, miru scans the current directory
tree recursively and presents every `.md` file in a scrollable list.
Each entry shows the file path, size, and relative modification time.

Press `/` to activate fuzzy filtering — characters you type narrow
the list instantly with match highlighting. Press `Escape` to clear
the filter and restore all results.

## Multi-Tab Reading

Open several documents at once and switch between them with `h` and
`l`. Each tab is independent: scroll position, search query, and
heading index selection are all preserved per tab.

Use `Ctrl+T` to open the file picker while a document is already
open, then select a file to add it as a new tab alongside the current
one. Close any tab with `w`.

## Full-Text Search

Press `/` from the reader to open the incremental search bar.
Every line containing a match is highlighted as you type, so you
get immediate feedback without pressing Enter first.

Confirm the search query with `Enter`, then use `n` to jump to the
next match and `N` to jump to the previous one. The status indicator
shows your current position — for example `2/7` — so you always know
how many matches exist and where you are among them.

## Headings Index

Press `Tab` to open an outline of all headings in the document.
The index lists every heading in document order with indentation that
reflects heading level, giving you a quick structural overview.

Navigate the list with `j` and `k`, then press `Enter` to jump
directly to the selected section. The index closes automatically
and the reader scrolls to the chosen heading.

## Theme Awareness

miru queries your terminal's background colour using the OSC 11
escape sequence and automatically selects a dark or light palette.
There is no configuration file and no flag to set — it just works
with whatever colour scheme your terminal uses.
