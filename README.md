# metafix

`metafix` is a Rust-based command-line utility (with optional TUI) 
for batch editing and fixing metadata in your photo and video files.  

The main focus of this project is educational: 
to explore how metadata is stored in different media formats 
and to deepen understanding of the Rust programming language.

---

## Disclaimer

> **This project is for educational purposes only.**
>
> - It is not intended for production use, but rather as a learning tool 
    and a personal experiment in low-level format parsing and manipulation
> - The code is experimental and may not handle all edge cases or file types.
> - Always back up your files before using any batch metadata tool.

---

## Project Goals

- **Learn and experiment with Rust** (2024 edition), including workspace organization, modules, and idiomatic code.
- **Understand and implement** low-level parsing and editing of photo and video metadata (EXIF for images, MP4 atoms for videos) without relying on existing high-level libraries.
- **Process supplementary JSON files** from Google Takeout to batch-restore original creation dates and geolocation data.
- **Provide a CLI and optional TUI** (based on [Ratatui](https://github.com/ratatui-org/ratatui)) for previewing, selecting, and applying metadata changes.
- **Document the process** and findings for future reference and for anyone interested in similar explorations.

---

## Features

- Scan directories for supported photo and video files.
- Parse and display existing metadata (date taken, geolocation, etc.).
- Import and match supplementary JSON metadata from Google Takeout exports.
- Preview which files and metadata fields will be updated.
- Batch-apply metadata changes to files (with a "dry run" mode).
- Interactive TUI for selection and confirmation (optional).
- Modular codebase with separate crates for EXIF, MP4, JSON parsing, and core logic.

---

## Project Structure

This repository is organized as a [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html):

---

## Getting Started

### Clone the repository:

```shell
git clone https://github.com/aOri69/metafix.git
cd metafix
```

### Build the CLI

```shell
cargo build --workspace
```

### Run the CLI

```shell
cargo run -p metafix-cli -- --help
```

### (Optional) Run the TUI

```shell
cargo run -p metafix-tui
```

---

## License

[MIT](./LICENSE)

---

## Contributions

This is a personal learning project and not actively maintained for general use.  
If you are interested in similar explorations or 
want to discuss metadata formats and Rust, 
feel free to open an issue or start a discussion!
