Output docs/demo.gif

Set FontSize 13
Set Width 1200
Set Height 650
Set Theme "Catppuccin Mocha"
Set TypingSpeed 50ms
Set Framerate 24

# ── Setup: navigate to project root (hidden from recording) ──────────────
Hide
Type "cd /Users/arturkantorczyk/miru"
Enter
Sleep 300ms
Show

# ── Launch miru — picker appears with all .md files ──────────────────────
Type "./target/release/miru"
Enter
Sleep 2s

# ── Navigate down through the file list ──────────────────────────────────
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms

# ── Fuzzy-filter to features.md ──────────────────────────────────────────
Type "/"
Sleep 400ms
Type "feat"
Sleep 1.2s

# ── Confirm filter (list narrows, navigate mode becomes active) ───────────
Enter
Sleep 800ms

# ── Navigate to the match and open ───────────────────────────────────────
Enter
Sleep 1s

# ── Scroll through the document ──────────────────────────────────────────
Type "j"
Sleep 160ms
Type "j"
Sleep 160ms
Type "j"
Sleep 160ms
Type "j"
Sleep 160ms
Type "j"
Sleep 160ms
Type "j"
Sleep 160ms
Type "j"
Sleep 400ms

# ── Live search: highlight every "search" match ───────────────────────────
Type "/"
Sleep 400ms
Type "search"
Sleep 900ms
Enter
Sleep 700ms

# ── Cycle through matches ─────────────────────────────────────────────────
Type "n"
Sleep 700ms
Type "n"
Sleep 600ms
Type "n"
Sleep 500ms

# ── Clear search, jump back to top ────────────────────────────────────────
Type "/"
Sleep 200ms
Escape
Sleep 300ms
Type "g"
Sleep 500ms

# ── Open headings index ───────────────────────────────────────────────────
Tab
Sleep 1s

# ── Navigate the outline ─────────────────────────────────────────────────
Type "j"
Sleep 380ms
Type "j"
Sleep 380ms
Type "j"
Sleep 380ms
Type "j"
Sleep 300ms

# ── Jump to selected heading ─────────────────────────────────────────────
Enter
Sleep 1.5s

# ── Open a second tab with Ctrl+T ────────────────────────────────────────
Ctrl+T
Sleep 1s

# ── Filter to shortcuts.md ───────────────────────────────────────────────
Type "/"
Sleep 300ms
Type "short"
Sleep 1s

# ── Confirm filter, then open ─────────────────────────────────────────────
Enter
Sleep 500ms
Enter
Sleep 1.2s

# ── Switch between tabs with h / l ───────────────────────────────────────
Type "h"
Sleep 900ms
Type "l"
Sleep 900ms
Type "h"
Sleep 700ms

# ── Quit ─────────────────────────────────────────────────────────────────
Type "q"
Sleep 500ms
