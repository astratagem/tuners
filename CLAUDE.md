<!--
SPDX-FileCopyrightText: (C) 2025 chris montgomery <chmont@protonmail.com>

SPDX-License-Identifier: AGPL-3.0-or-later
-->

# Claude Code Context

This document provides context for Claude Code when working on this project.

## Project Overview

**Name**: tuners
**Type**: Music library manager (TUI application)
**Language**: Rust
**Stage**: Phase 0 - Proof of Concept (In Progress)
**Purpose**: Rewrite of Python beets with focus on performance, type safety, and reliability

## Current Status

**Completed:**
- TUI scaffold with ratatui and crossterm
- Sequential directory scanning with concurrent search processing
- Metadata extraction (MP3, M4A, FLAC)
- Album clustering logic
- State machine architecture
- Terminal lifecycle management
- MusicBrainz API client with rate limiting (1 req/sec)
- Concurrent scan/search pipeline with bounded queue (capacity 5)
- Basic match result display UI showing artist, title, date, track count

**Next Immediate Tasks:**
- Build similarity scoring algorithm (0-100%)
- Create track mapping preview screen
- Implement manual search functionality
- Add dry-run tag writing mode
- Implement actual tag writing (apply functionality)

See [ROADMAP.md](ROADMAP.md) for complete development plan.

## Architecture Principles

### Separation of Concerns

- `main.rs` - **Only** argument parsing and error setup
- `app.rs` - Application lifecycle, event loop, state management
- `terminal.rs` - Terminal initialization and cleanup
- `ui.rs` - Pure rendering functions (no business logic)
- `scanner/` - File scanning and metadata extraction
- Domain modules separate from infrastructure

### State Machine Pattern

All application states are defined in `AppState` enum in `app.rs`:
```rust
pub enum AppState {
    Scanning { /* fields */ },
    AutoTagging {
        cluster: AlbumCluster,
        results: Vec<Release>,
        selected_idx: usize,
    },
    ClusterList { /* fields */ },
    Error { /* fields */ },
    // Add new states here
}
```

State transitions happen via methods in `App`:
- Named transition methods (e.g., `transition_to_clusters()`)
- All transitions in one place for easy tracking
- States carry their own data (no shared mutable state)

### Error Handling

- Use `color_eyre::eyre::Result` everywhere
- Add context with `.context()` on fallible operations
- Never use `.unwrap()` in user-facing code
- Use `.expect()` only for logic errors that should never happen
- Rich error messages that help debugging

### Module Organization

```
src/
  lib.rs (if needed)    - Public API
  main.rs               - CLI entry point only
  app.rs                - Application controller
  terminal.rs           - Terminal abstraction
  ui.rs                 - Rendering dispatch
  models.rs             - Domain types

  scanner/
    mod.rs              - Public scanning API
    metadata.rs         - Private implementation

  musicbrainz/          ✓ implemented
    mod.rs              - Public API and SearchMessage types
    client.rs           - Rate-limited MusicBrainz API wrapper
    search.rs           - Search logic with message passing

  matching/             (to be built)
    mod.rs              - Public scoring API
    distance.rs         - String similarity
    scorer.rs           - Release scoring logic
```

**Pattern**: Each module has `mod.rs` with public API, implementation details in separate files.

## Code Conventions

### Naming

- `snake_case` for functions, variables, modules
- `PascalCase` for types, traits, enums
- Descriptive names (prefer `selected_cluster_index` over `idx`)
- No abbreviations unless standard (e.g., `mb` for MusicBrainz is OK)

### Function Size

- Aim for <50 lines per function
- Extract helper functions when logic gets complex
- Private helper functions are fine (use `fn`)

### Error Messages

- User-facing: Clear, actionable
  - Bad: "IO error"
  - Good: "Failed to read music directory: permission denied"
- Add context showing what was being attempted
- Include paths/values when relevant

### Comments

- Doc comments (`///`) for public API
- Regular comments for non-obvious logic
- No comments for obvious code
- Prefer self-documenting code over comments

### Testing

Not yet implemented, but when adding tests:
- Unit tests in same file: `#[cfg(test)] mod tests`
- Integration tests in `tests/` directory
- Test edge cases, not just happy path
- Use descriptive test names: `test_cluster_ignores_hidden_files`

## Important Files

### Configuration

- `Cargo.toml` - Dependencies and project metadata
- `.gitignore` - Standard Rust gitignore

### Documentation

- `README.md` - Current status and quick start (user-facing)
- `ROADMAP.md` - Complete development plan (all phases)
- `RATATUI_PATTERNS.md` - Modern ratatui component-based architecture (for future refactoring)
- `RATATUI_UPDATE.md` - Ratatui 0.29 update and migration strategy
- `CLAUDE.md` - This file (for Claude Code)
- `RENAME.md` - Project rename details

### Source Code

**Core:**
- `src/main.rs` - Entry point (keep minimal)
- `src/app.rs` - State machine and event loop (complex, modify carefully)
- `src/terminal.rs` - Terminal setup/teardown (rarely needs changes)
- `src/models.rs` - Domain types (add new types here)

**Scanner:**
- `src/scanner/mod.rs` - Public API: `scan_directory`, `cluster_files`
- `src/scanner/metadata.rs` - Format-specific tag extraction

**UI:**
- `src/ui.rs` - Rendering dispatch (add new screens here)

## Common Tasks

### Adding a New Application State

1. Add variant to `AppState` enum in `app.rs`
2. Add rendering function in `ui.rs`: `render_your_state()`
3. Add match arm in `ui::render()` to dispatch to your function
4. Add transition method in `App`: `transition_to_your_state()`
5. Add input handling in `App::handle_events()` match statement

### Adding Background Work (like MusicBrainz search)

Pattern already established in scanner:

1. Define message enum:
```rust
enum YourMessage {
    Progress(Data),
    Complete(Result),
    Error(String),
}
```

2. Create channel in relevant method:
```rust
let (tx, rx) = mpsc::channel();
self.your_rx = Some(rx);
```

3. Spawn thread:
```rust
thread::spawn(move || {
    match do_work() {
        Ok(result) => tx.send(YourMessage::Complete(result)),
        Err(e) => tx.send(YourMessage::Error(e.to_string())),
    }
});
```

4. Check messages in `App::handle_messages()`:
```rust
if let Some(rx) = &self.your_rx {
    if let Ok(msg) = rx.try_recv() {
        self.handle_your_message(msg);
    }
}
```

### Adding a New Audio Format

1. Add extension to `AUDIO_EXTENSIONS` in `scanner/mod.rs`
2. Add extraction function in `scanner/metadata.rs`:
```rust
fn extract_your_format(path: &Path) -> Result<(...)> {
    // Use appropriate tag library
}
```
3. Add match arm in `extract()` function
4. Test with real files of that format

### Adding a New UI Screen

1. Create rendering function in `ui.rs`:
```rust
fn render_your_screen(
    frame: &mut Frame,
    data: &YourData,
    selected: usize,
) {
    // Use ratatui widgets
}
```

2. Use Layout for screen sections:
```rust
let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),  // Header
        Constraint::Min(10),     // Main content
        Constraint::Length(3),   // Footer/help
    ])
    .split(frame.area());
```

3. Always include help text showing available keys

## Dependencies

### Core
- `ratatui = "0.29"` - TUI framework
- `crossterm = "0.27"` - Terminal backend
- `color-eyre = "0.6"` - Error handling

### Audio
- `id3 = "1"` - MP3 tags
- `mp4ameta = "0.11"` - M4A/MP4 tags
- `metaflac = "0.2"` - FLAC tags

### Utilities
- `walkdir = "2"` - Directory traversal
- `rayon = "1.10"` - Data parallelism for concurrent scanning
- `strsim = "0.11"` - String similarity (for matching)
- `mp3-duration = "0.1"` - Calculate MP3 duration from audio stream

### Async/Future
- `musicbrainz_rs = "0.12"` - MusicBrainz API (not yet used)
- `tokio = "1.40"` - Async runtime (for MB API)

**Adding dependencies:**
- Check if widely used and maintained
- Prefer well-established crates
- Avoid dependencies with C bindings unless necessary
- Update `CLAUDE.md` when adding major dependencies

## Patterns to Follow

### RAII for Resources

Terminal cleanup example in `app.rs`:
```rust
pub fn run(&mut self) -> Result<()> {
    let mut terminal = terminal::init()?;
    // Use terminal
    terminal::restore()?;  // Always called
    Ok(())
}
```

### Builder Pattern for Complex Construction

For search queries (future):
```rust
SearchBuilder::new()
    .artist("The Beatles")
    .album("Abbey Road")
    .build()?
```

### NewType Pattern for Clarity

Instead of:
```rust
HashMap<(PathBuf, String, String), Vec<AudioFile>>
```

Use:
```rust
struct ClusterKey {
    base_path: PathBuf,
    artist: String,
    album: String,
}
HashMap<ClusterKey, Vec<AudioFile>>
```

### Composition Over Inheritance

No inheritance in Rust, use:
- Traits for shared behavior
- Composition for shared data
- Enums for variants

## Anti-Patterns to Avoid

### Don't: Unnecessary Clones

```rust
// Bad
let data = expensive_data.clone();
process(data);
drop(expensive_data);

// Good - consume if you're done with it
process(expensive_data);
```

### Don't: Stringly-Typed Code

```rust
// Bad
if state == "scanning" { ... }

// Good
match app_state {
    AppState::Scanning { ... } => { ... }
}
```

### Don't: Panic in User-Facing Code

```rust
// Bad
let config = read_config().unwrap();

// Good
let config = read_config()
    .context("Failed to read configuration file")?;
```

### Don't: God Structs

Keep structs focused:
```rust
// Bad
struct App {
    state: State,
    terminal: Terminal,
    database: Database,
    config: Config,
    scanner: Scanner,
    mb_client: MBClient,
    // ... 20 more fields
}

// Good - compose as needed
struct App {
    state: AppState,
    // Other components accessed via modules
}
```

## Working with the Event Loop

The event loop in `app.rs` follows this pattern:

```rust
loop {
    terminal.draw(|f| ui::render(f, &self.state))?;
    self.handle_messages();  // Check background tasks
    if event::poll(TICK_RATE)? {
        self.handle_events()?;  // Handle keyboard input
    }
    if self.should_quit { break; }
}
```

**Key points:**
- `draw()` called every iteration (renders current state)
- `handle_messages()` checks for background task results
- `handle_events()` processes keyboard input
- `TICK_RATE` (100ms) balances responsiveness and CPU usage

**When adding new features:**
- State changes → update `self.state`
- Long operations → spawn thread, use channel
- User input → add handler in `handle_events()`
- UI updates → add function in `ui.rs`

## Testing Approach

Not yet implemented, but planned approach:

### Unit Tests
- Test pure functions in isolation
- Mock external dependencies
- Focus on edge cases

### Integration Tests
- Test full workflows
- Use temporary directories for file operations
- Real tag libraries (with test fixtures)

### Manual Testing
- Test on real music collection
- Various file formats and edge cases
- Different terminal sizes

## Performance Considerations

Current phase (PoC) prioritizes correctness over performance, but:

**Do:**
- Use iterators instead of collecting unnecessarily
- Avoid cloning large data structures
- Use references where possible

**Don't (yet):**
- Premature optimization
- Unsafe code for performance
- Complex caching schemes

**Implemented:**
- ✅ Parallel scanning with rayon (see "Parallel Processing" section below)

**Later phases** will focus on:
- Database query optimization
- Caching metadata lookups

## Parallel Processing with Rayon

The scanner uses rayon for data parallelism across multiple CPU cores.

### Pattern: Progress Updates from Parallel Iterators

```rust
use std::sync::{mpsc::Sender, Arc, atomic::{AtomicUsize, Ordering}};

pub fn scan_directory(
    path: &Path,
    progress_tx: Option<Sender<(usize, String)>>,
) -> Result<Vec<AudioFile>> {
    let tx = progress_tx.map(Arc::new);  // Wrap in Arc for thread-safety
    let count = Arc::new(AtomicUsize::new(0));  // Thread-safe counter

    let files: Vec<AudioFile> = WalkDir::new(path)
        .into_iter()
        .par_bridge()  // Convert to parallel iterator
        .filter_map(|entry| {
            // Process entry...
            let audio_file = metadata::extract(path).ok()?;

            // Send progress update from parallel context
            if let Some(ref tx) = tx {
                let n = count.fetch_add(1, Ordering::Relaxed);
                let _ = tx.send((n + 1, path.display().to_string()));
            }

            Some(audio_file)
        })
        .collect();

    Ok(files)
}
```

**Key techniques:**
- `Arc<Sender>` - Share sender across parallel threads
- `AtomicUsize` - Thread-safe counter without locks
- `.par_bridge()` - Convert sequential iterator to parallel
- `Ordering::Relaxed` - Good enough for progress counts (don't need strict ordering)

### Why Rayon?

**Decision**: Use rayon instead of manual thread management for parallelism.

**Reasoning**:
- Zero-cost abstraction over thread pools
- Standard in Rust ecosystem (used by ripgrep, rustc)
- Encourages data parallelism (vs shared mutable state)
- Simpler than manual thread spawning/joining
- Learning goal is building the music manager, not becoming threading expert

**Trade-off**: Less low-level threading knowledge, but better project focus.

## MusicBrainz Integration (In Progress)

### Auto-Tagging Workflow (Beets-Inspired)

**Important**: The workflow is **automatic**, not manual. Searching begins during scan, processing happens concurrently.

**Implementation Status**:

✅ **Completed**:
1. Concurrent scan/search pipeline with bounded queue (capacity 5)
2. Search MusicBrainz using cluster's **existing tags** (album_artist + album)
   - Beets infers tags from existing metadata, not from directory/filenames
   - Directories are only used for **grouping** files into albums
   - If tags are missing/bad → poor search results → user uses manual search
3. Basic match result display showing artist, title, date, track count
4. Queue management system for processing clusters one at a time
5. j/k navigation through results

⏳ **To Do**:
1. Similarity scoring algorithm (0-100%)
2. Track mapping preview (show proposed tag changes)
3. Manual search functionality (enter custom artist/album query)
4. Dry-run mode (show what would be written without modifying files)
5. Actual tag writing functionality (apply selected match to files)

**Note on metadata inference**: Beets expects music to have *some* existing tags, even if imperfect. Directory names and filenames are NOT used for searching - only for organization. The **FromFilename** plugin (Phase 3+) handles filename parsing as a fallback for completely untagged files.

**Phase 3 additions** (deferred):
- Auto-apply high confidence matches (≥98%)
- More user options ([U]se as-is, [T]racks mode, etc.)
- Duplicate detection

### Module Structure
```
src/musicbrainz/
  mod.rs      - Public API and message types
  client.rs   - Rate-limited wrapper around musicbrainz_rs
  search.rs   - Search logic (artist+album query)

src/matching/   (or integrated into musicbrainz for PoC)
  mod.rs      - Similarity scoring (0-100%)
```

### Implementation Details

1. ✅ **Rate Limiting**: MusicBrainz requires 1 request/sec
   - Implemented via `tokio::time::sleep` with throttle tracking
   - Enforced in `Client::throttle()` method

2. ✅ **Search Strategy**:
   - Primary: Artist + Album search from cluster metadata
   - Uses ReleaseSearchQuery builder with artistname + release fields
   - ⏳ Manual search option (not yet implemented)

3. ✅ **Threading Pattern**:
   - Scanner thread emits clusters via bounded sync_channel (capacity 5)
   - Search worker thread with tokio runtime consumes cluster queue
   - Sends results back via mpsc::channel (SearchMessage enum)
   - UI thread remains synchronous

4. ⏳ **Error Handling**:
   - Network errors → currently just logged to stderr
   - No results found → displays "No matches found" message
   - ⏳ Retry functionality not implemented

5. ✅ **State Integration**:
   ```rust
   pub enum AppState {
       // ... existing
       AutoTagging {
           cluster: AlbumCluster,
           results: Vec<Release>,
           selected_idx: usize,
       },
   }
   ```
   Note: Simplified from initial design - no separate TaggingStatus enum for PoC.

## Similarity Scoring (Integrated with MusicBrainz)

The scoring algorithm calculates match confidence (0-100%) for MusicBrainz results.

### Scoring Factors (Initial Weights)
- Artist name similarity (30% weight) - using `strsim` crate
- Album title similarity (30% weight) - using `strsim` crate
- Track count match (10% weight) - exact match bonus
- Track name matching (20% weight) - average similarity across tracks
- Duration similarity (10% weight) - if available

**Note**: These weights will need tuning with real data.

### Implementation Location (PoC)

For PoC, integrate scoring into musicbrainz module to keep simple:
```
src/musicbrainz/
  mod.rs      - Public API and ScoredRelease type
  client.rs   - Rate-limited API wrapper
  search.rs   - Search + scoring
```

Later (Phase 1+), extract to separate `matching/` module.

### Key Types

```rust
pub struct ScoredRelease {
    pub release: Release,           // from musicbrainz_rs
    pub score: u8,                   // 0-100 (percentage)
    pub track_mappings: Vec<TrackMapping>,
}

pub struct TrackMapping {
    pub cluster_track: AudioFile,
    pub mb_track: Track,
    pub confidence: u8,              // 0-100
}
```

### String Similarity

Use `strsim::jaro_winkler()` for artist/album/track names:
- Returns 0.0-1.0
- Good for fuzzy matching with typos
- Weights beginning of strings more heavily

## Questions to Ask Before Big Changes

1. Does this fit the current phase? (Check ROADMAP.md)
2. Does this follow existing patterns? (Check PATTERNS.md)
3. Are errors handled properly? (Check COLOR_EYRE.md)
4. Is the public API clean? (Hide implementation details)
5. Will this need refactoring later? (Is that acceptable?)

## When Stuck

1. Check existing similar code (scanner is a good reference)
2. Read PATTERNS.md for Rust idioms
3. Check ROADMAP.md to see if feature is planned later
4. Look at ratatui examples for UI patterns
5. Check musicbrainz_rs docs for API usage

## References

- **Project docs**: README.md, ROADMAP.md, PATTERNS.md
- **Rust**: https://doc.rust-lang.org/book/
- **ratatui**: https://ratatui.rs/
- **MusicBrainz API**: https://musicbrainz.org/doc/MusicBrainz_API
- **musicbrainz_rs**: https://docs.rs/musicbrainz_rs/
- **color-eyre**: https://docs.rs/color-eyre/

## Summary

This is a well-structured Rust project following idiomatic patterns. When adding features:
- Maintain separation of concerns
- Follow existing patterns
- Add proper error handling
- Keep modules focused
- Test with real data

The immediate next task is MusicBrainz integration. See the "MusicBrainz Integration" section above for guidance.

**Note on Architecture:** The current functional TUI pattern works well for Phase 0. See [RATATUI_PATTERNS.md](RATATUI_PATTERNS.md) for modern component-based patterns to consider before Phase 1. The refactoring is straightforward but should wait until after MusicBrainz matching is validated.
