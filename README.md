# `zettarium` – Zettelkasten in Your Terminal

A minimal, keyboard-centric Zettelkasten CLI for structured thinking and lifelong note-taking.

---

## Features

- 📄 **Create, edit, archive and remove notes** in a structured Markdown-based Zettelkasten
- 🔗 **Manual link philosophy**: No automatic backlinks. You insert contextual links consciously.
- 🔍 **Interactive search (`z find`)** powered by `fzf`, with live preview using `bat`
- 🧩 **Note types**: Fleeting, Literature, Permanent, Structure, Index
- 🗃️ **Flat structure**: No project folders — all projects are represented through Structure notes
- ⚙️ **Configurable** via `~/.config/zettarium/config.toml`

---

## Installation

You'll need to have:

- Rust (`cargo`)
- [fzf](https://github.com/junegunn/fzf): Interactive search
- [bat](https://github.com/sharkdp/bat): Markdown preview
- [sqlite3](https://www.sqlite.org/index.html): Local database engine

```bash
brew install fzf bat
cargo install zettarium
```

---

## 📁 Configuration

`zettarium` expects a config file at:

```bash
~/.config/zettarium/config.toml
```

Example:

```toml
[paths]
zettel_dir = "/Users/you/Zettels"
archive_dir = "/Users/you/Zettels/archive"

[editor]
editor = "nvim"
```

---

## Commands Overview

### Create a note

```bash
z new "Understanding Ownership" --type permanent --tags rust,concept
```

### List notes

```bash
z list                          # All active notes
z list --type structure         # Filter by type
z list --tags rust,testing      # Filter by tags
z list --archived               # Show only archived
```

### View or edit a note

```bash
z view <note_id>
z edit <note_id> --title "New title" --tags async,await
z edit <note_id>               # Opens in your editor
```

### Archive / remove a note

```bash
z archive <note_id>
z remove <note_id> --force
```

---

## 🔍 Finding Notes (and Inserting Links)

Search across your Zettelkasten with preview and clipboard support:

```bash
z find "Rust ownership"          # Interactive full-text search
z find "ownership" --title-only # Title-based search
z find "error handling" --link  # Copies [title](./note.md) link to clipboard
```

Link insertion is **manual by design** — write links contextually like:

```md
See also [Understanding Ownership](./20250605113000.md)
```

---

## Zettelkasten Philosophy

This tool embraces the core principles from *How to Take Smart Notes*:

- All notes are equal — just different in purpose (fleeting, literature, etc.)
- Projects live inside the network as **Structure notes**, not folders
- Linking is **your thinking** — the tool only assists

---

## Roadmap (Planned Features)

- `z stat`: Statistics and note growth visualization
- `z daily`: Daily log for journaling / fleeting input
- `z backlink`: Show incoming and outgoing note references

---

## License & Contribution

MIT License. PRs welcome for those who share the same Zettelkasten spirit.
Built with ❤️ and 🦀 for deep thinkers and lifelong learners.