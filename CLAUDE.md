# CLAUDE.md

`lim` is a lightweight Rust CLI for logging events to JSONL files.

## Development Commands

### Build & Run
```bash
cargo build
cargo run -- add <path> <message> [attributes...]
cargo run -- query <path>  # not yet implemented
```

### Testing
```bash
cargo test              # run all tests
cargo test <test_name>  # run single test
```

### Install
```bash
mise run install
# or: cargo install --path .
```

### Completions
```bash
# After adding new commands, regenerate the shell completion file:
lim completions zsh > completions/_lim
```

## Code Conventions

### Path Handling
- **IMPORTANT:** Always use `camino::Utf8PathBuf` instead of `std::path::PathBuf`
- These are imported via prelude (`src/prelude.rs`) for guaranteed UTF-8 paths
- Never use `OsStr` or `OsString` - this project assumes UTF-8 paths throughout

### Error Handling
- Use `anyhow::Result` and `anyhow::Context` (imported in prelude)
- Add context to errors with `.context()` or `.with_context()`

## Architecture

### Adding New Commands
1. Create `src/command/<name>.rs` with struct implementing `Run` trait
2. Add variant to `LimCommand` enum in `src/command.rs`
3. Add pattern match in `Lim::run()` to dispatch to `cmd.run(&config)`

### Adding New Log Backends
1. Implement `Log` trait (see `src/log/file.rs` for reference)
2. Update `File` instantiation in command handlers to use new backend

### Configuration System
Uses `Figment` for layered config: defaults → TOML file → env vars
- Config file: `~/.config/lim/lim.toml` (OS-dependent)
- Log data: `~/.local/share/lim/` (default, configurable via `[log_dir]` in config)
