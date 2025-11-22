<!--
SPDX-FileCopyrightText: (C) 2025 chris montgomery <chmont@protonmail.com>

SPDX-License-Identifier: AGPL-3.0-or-later
-->

# Tuners

A proof-of-concept music library manager inspired by beets.

This project is in its very early stages.  And I am still learning
Rust.

## Current Status

**Phase 0: Proof of Concept** (In Progress)

This is an early-stage scaffold demonstrating idiomatic Rust project structure for a TUI application. The scaffold is complete and working - MusicBrainz integration is the next major task.

### What Works Now

- [x] TUI interface with ratatui
- [x] Parallel directory scanning with live progress (using rayon)
- [x] Metadata extraction from MP3, M4A, and FLAC files
- [x] Duration calculation from audio streams (MP3, FLAC, M4A)
- [x] Album clustering based on directory and tags
- [x] Multi-disc album support with proper track sorting
- [x] Interactive cluster detail view with track listings
- [x] Keyboard navigation (j/k, arrows, space, Enter)

### What's Next

See [ROADMAP.md](ROADMAP.md) for the complete development plan.

**Immediate next steps:**
- Auto-tagging workflow (automatic MusicBrainz search after scan)
- Similarity scoring algorithm (0-100% match confidence)
- Interactive prompts for ambiguous matches
- Track mapping preview before applying changes

The goal is to replicate beets' core auto-tagging experience: scan → auto-search → prompt only when needed → preview changes.

## Quick Start

### Building

```bash
cargo build
```

### Running

Scan current directory:
```bash
cargo run
# or after building: tune
```

Scan specific directory:
```bash
cargo run /path/to/music
# or after building: tune /path/to/music
```

### Controls

**Scanning screen:**
- `Enter` - Continue to cluster list (when scan complete)
- `q` or `Ctrl-C` - Quit

**Cluster list screen:**
- `↑/↓` or `j/k` - Navigate clusters
- `Space` or `Enter` - View cluster details
- `q` or `Ctrl-C` - Quit

**Cluster detail screen:**
- `↑/↓` or `j/k` - Navigate tracks
- `Esc` or `h` - Back to cluster list
- `q` or `Ctrl-C` - Quit

## Architecture

### Module Structure

```
src/
  main.rs          - Entry point (argument parsing only)
  app.rs           - Application state machine and event loop
  ui.rs            - TUI rendering for each state
  models.rs        - Domain types (AudioFile, AlbumCluster)
  codecs.rs        - Audio codec enumeration
  scanner.rs       - Directory scanning and clustering (with rayon parallelism)
  scanner/
    metadata.rs    - Tag extraction by format (MP3, M4A, FLAC)
```

### Design Principles

**Separation of Concerns:**
- `main.rs` handles only argument parsing and error reporting
- `app.rs` owns the application lifecycle and event loop
- `terminal.rs` encapsulates crossterm setup/teardown
- UI rendering is isolated from business logic

**Error Handling:**
- `color_eyre::eyre::Result` for application-level errors
- Beautiful error reports with backtraces and suggestions
- Proper cleanup via RAII (terminal restoration happens even on panic)
- Context added to errors for debugging

**State Machine:**
- Explicit `AppState` enum prevents invalid states
- State transitions are centralized in `app.rs`
- Each state has well-defined entry/exit behavior

**Modularity:**
- Scanner functionality is split into public API (mod.rs) and implementation details (metadata.rs)
- Future expansion: add `matching/`, `musicbrainz/` modules without changing existing code

## Project Documentation

- **[ROADMAP.md](ROADMAP.md)** - Complete development plan through all phases
- **[REFACTORING.md](REFACTORING.md)** - Explanation of architectural decisions
- **[PATTERNS.md](PATTERNS.md)** - Rust patterns used in the project
- **[COLOR_EYRE.md](COLOR_EYRE.md)** - Error handling with color-eyre
- **[CLAUDE.md](CLAUDE.md)** - Project context for Claude Code

## Dependencies

- **ratatui** - TUI framework
- **crossterm** - Terminal handling (backend for ratatui)
- **rayon** - Data parallelism for concurrent directory scanning
- **musicbrainz_rs** - MusicBrainz API wrapper (not yet used)
- **tokio** - Async runtime (for future MusicBrainz calls)
- **id3, mp4ameta, metaflac** - Tag reading libraries
- **mp3-duration** - Calculate MP3 duration from audio stream
- **walkdir** - Directory traversal
- **color-eyre** - Error handling with beautiful reports
- **strsim** - String similarity (for matching algorithm)

## Development

### Running with Better Errors

Enable backtraces for debugging:
```bash
RUST_BACKTRACE=1 cargo run /path/to/music
# or after building: RUST_BACKTRACE=1 tune /path/to/music
```

### Code Organization

The project follows idiomatic Rust practices:
- Pure functions where possible
- Clear ownership boundaries
- Explicit error handling
- Minimal `unsafe` code (currently none)

See [PATTERNS.md](PATTERNS.md) for detailed explanations of patterns used.

## Contributing

This is a personal learning project, but observations and suggestions are welcome.

When adding features:
1. Read the relevant section in [ROADMAP.md](ROADMAP.md)
2. Follow patterns from existing code
3. Add tests for new functionality
4. Update documentation

## License

To be determined (project is in early stages)

## Acknowledgments

- [beets](https://beets.io/) - The original and inspiration
- [ratatui](https://ratatui.rs/) - Excellent TUI framework
- Rust community for helpful libraries and documentation
