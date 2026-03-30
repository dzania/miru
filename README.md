# miru

Minimal markdown reader I use daily.

![Demo](docs/demo.gif)

## Usage

```
miru [files...]
```

Open files directly or run `miru` in a directory to pick from `.md` files.

## Keys

### Reader

| Key | Action |
|-----|--------|
| `j/k` | Scroll |
| `d/u` | Page down/up |
| `g/G` | Top/bottom |
| `/` | Search |
| `n/N` | Next/prev match |
| `h/l` | Prev/next tab |
| `Tab` | Headings index |
| `Ctrl+T` | Open file |
| `w` | Close tab |
| `?` | Toggle help |
| `q` | Quit |

### Picker

| Key | Action |
|-----|--------|
| `j/k` | Navigate |
| `/` | Enter filter |
| `Enter` | Confirm filter / open file |
| `Esc` | Cancel filter / clear filter |
| `q` | Quit |

## Install

```
cargo install --path .
```
