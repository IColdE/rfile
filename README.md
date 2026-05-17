# rfile - Rust Utility Toolkit

A collection of high-performance CLI tools built with Rust to demonstrate speed, safety, and modern systems programming.

## 🛠 Included Tools

### 1. `search` (The Core Tool)
A blazing-fast file searcher and replacer.
- **Features:** Parallel processing across CPU cores, respects `.gitignore`, regex support, and colored output.
- **Search:** `cargo run --bin search -- "pattern" .`
- **Replace:** `cargo run --bin search -- "old_text" -r "new_text" .`
- **Options:** `-i` for case-insensitive, `--line-numbers false` to hide lines.

### 2. `monitor` (System Watcher)
A real-time system resource monitor.
- **Features:** Live CPU usage bars per core, Memory/RAM tracking, and auto-refreshing interface.
- **Run:** `cargo run --bin monitor`
- **Stop:** Press `Ctrl+C`.

### 3. `todo` (Task Manager)
A persistent CLI-based To-Do list.
- **Features:** JSON-backed storage (remembers your tasks), subcommands for adding/listing/completing.
- **Add:** `cargo run --bin todo -- add "Task description"`
- **List:** `cargo run --bin todo -- list`
- **Done:** `cargo run --bin todo -- done <ID>`
- **Remove:** `cargo run --bin todo -- remove <ID>`

---

## 🚀 Why these tools?

These projects were created to explore the Rust ecosystem:
- **Speed:** `rayon` for multi-threaded file scanning.
- **Intelligence:** `ignore` for smart folder skipping (like `ripgrep`).
- **Hardware:** `sysinfo` for direct OS/Hardware interaction.
- **Data:** `serde` for seamless JSON serialization.
- **UI:** `colored` for a better terminal experience.

## 📦 Building for Production

To create optimized, standalone `.exe` files:
```powershell
cargo build --release
```
The executables will be waiting for you in `target/release/`.
