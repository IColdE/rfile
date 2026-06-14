# rfile - Rust Utility Toolkit (v0.2.0)

A high-performance collection of CLI tools built with Rust, optimized for speed, safety, and modern systems programming.

## 🛠 Included Tools

### 1. `search`
A multi-threaded file searcher and replacer.
- **How it works:** 
    - Uses `ignore` to walk the file system while respecting `.gitignore` and hidden file rules.
    - Employs `rayon` to process files in parallel across all available CPU cores.
    - Utilizes `regex` for fast pattern matching and optional text replacement.
    - Implements `anyhow` for robust error propagation and context-aware failure reporting.
- **Run Search:** `cargo run --bin search -- "pattern" .`
- **Run Replace:** `cargo run --bin search -- "old_text" -r "new_text" .`
- **Options:** `-i` (case-insensitive), `--line-numbers false` (hide output line numbers).

### 2. `monitor`
A real-time system resource dashboard.
- **How it works:**
    - Leverages `sysinfo` to fetch live hardware metrics directly from the OS.
    - Displays per-core CPU usage with visual bars and tracks memory allocation (Used/Total).
    - Refreshes every second using a terminal clear-and-draw loop.
- **Run:** `cargo run --bin monitor`
- **Stop:** Press `Ctrl+C`.

### 3. `todo`
A persistent task manager.
- **How it works:**
    - Stores tasks in a local `tasks.json` file.
    - Uses `serde` and `serde_json` for efficient data serialization.
    - Manages task state (description, completion status) via ID-based subcommands.
- **Add:** `cargo run --bin todo -- add "Task description"`
- **List:** `cargo run --bin todo -- list`
- **Done:** `cargo run --bin todo -- done <ID>`
- **Remove:** `cargo run --bin todo -- remove <ID>`

---

## 🚀 Why I built this project

This toolkit was developed to explore and demonstrate the power of the Rust ecosystem in building production-grade system utilities. The primary goals were:
- **Performance:** Leveraging `rayon` and `ignore` to outperform traditional single-threaded tools.
- **Safety:** Using Rust's strict type system and ownership model to eliminate common memory bugs.
- **Modernity:** Integrating modern libraries like `anyhow` for error handling and `clap` for intuitive CLI design.
- **Practicality:** Creating a single repository for the most common developer workflows: searching, monitoring, and task management.

## 🧪 Testing & Reliability

The toolkit includes unit tests for core logic:
- **Search:** Validates regex matching and case-sensitivity.
- **Todo:** Verifies JSON serialization and task state integrity.

Run tests:
```powershell
cargo test
```

## 🚀 Building for Production

To generate optimized executables:
```powershell
cargo build --release
```
Binaries are located in `target/release/`.
