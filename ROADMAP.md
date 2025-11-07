# Development Roadmap

## Project Overview

A complete rewrite of beets music library manager in Rust, focusing on:
- Performance improvements through parallelization and native compilation
- Type safety and reliability
- Modern development practices
- Core feature parity with select plugins

**Target user**: Solo music library curator with large collections (1000+ albums)

## Development Philosophy

1. **Validate before investing**: PoC phase validates the riskiest assumptions
2. **Iterative feature delivery**: Each phase delivers working functionality
3. **Learning-driven**: Project doubles as Rust learning vehicle
4. **Quality over speed**: Idiomatic code, proper error handling, maintainability

## Timeline Overview

- **Phase 0 (PoC)**: 10-12 weeks - MusicBrainz matching validation
- **Phase 1**: 12-16 weeks - Core library infrastructure
- **Phase 2**: 8-12 weeks - Tag writing and organization
- **Phase 3**: 12-16 weeks - Full import pipeline
- **Phase 4**: 8-12 weeks - Discogs integration
- **Phase 5**: 16-24 weeks - Plugin ecosystem

**Total estimated time**: 66-92 weeks (15-21 months) at spare-time pace

**Note**: Timeline assumes evenings/weekends development (10-15 hours/week)

---

## Phase 0: Proof of Concept (Current)

**Duration**: 10-12 weeks  
**Status**: In progress (scaffold complete)

### Goals

Validate the three riskiest assumptions:
1. Can we match albums accurately enough? (MusicBrainz + scoring algorithm)
2. Are Rust's tag libraries adequate for real-world files?
3. Is the learning curve manageable?

### Deliverables

- [x] TUI scaffold with ratatui
- [x] Directory scanning and clustering
- [x] Tag reading (MP3, M4A, FLAC)
- [ ] MusicBrainz integration via musicbrainz_rs
- [ ] Fuzzy matching/scoring algorithm
- [ ] Interactive match selection UI
- [ ] Track mapping review screen
- [ ] Dry-run output (what would be written)

### Success Criteria

- Match accuracy >90% on test library (20-30 albums)
- Performance noticeably better than Python beets
- Code feels maintainable (not fighting the borrow checker constantly)
- Decision: Continue to Phase 1 or stop here

### Technical Focus

- **State machine patterns**: AppState enum with explicit transitions
- **Async in sync context**: Background MusicBrainz calls via channels
- **TUI architecture**: Separation of rendering and business logic
- **Error handling**: color-eyre for debugging and user feedback

### Key Modules to Build

```
src/
  musicbrainz/
    mod.rs          - Public search API
    client.rs       - Rate limiting wrapper
    search.rs       - Search strategies
  matching/
    mod.rs          - Public scoring API
    distance.rs     - String similarity algorithms
    scorer.rs       - Release scoring logic
    types.rs        - ScoredRelease, TrackMapping
```

### Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| musicbrainz_rs doesn't cover needed API | Fallback: write own client using reqwest |
| Matching algorithm too complex | Start with simple weighted score, iterate |
| Tag libraries have format quirks | Test on diverse real-world files early |
| Async integration difficult | Use simple channel pattern from scaffold |

---

## Phase 1: Core Library

**Duration**: 12-16 weeks  
**Prerequisites**: Phase 0 successful

### Goals

Build the foundational library management system without import/write capabilities.

### Architecture Refactoring

Before starting Phase 1 features, refactor TUI to component-based architecture:
- See [RATATUI_PATTERNS.md](RATATUI_PATTERNS.md) for modern patterns
- Extract `Action` enum for state changes
- Create `Component` trait for screens
- Migrate existing screens to components
- **Estimated time**: 2-3 days
- **Benefit**: Better foundation for Phase 1 complexity

This refactoring uses modern ratatui 0.29 patterns and sets up proper separation of concerns before database complexity is added.

### Deliverables

- SQLite database schema
- Query engine (beets' flexible syntax)
- Read-only operations: `ls`, `stats`, `info`
- Configuration system (TOML or similar)
- Template system for formatting output
- Logging infrastructure

### Database Schema

Core tables:
- `items` - Individual tracks
- `albums` - Album groupings
- `album_attributes` - Flexible attributes
- `item_attributes` - Track-level attributes

Design for: fast queries, flexible metadata, future plugin extensibility

### Query Language

Support beets query syntax:
```
artist:beatles year:1965..1970
album:abbey singleton:false
bitrate:320k+ format:MP3
```

Parser implementation options:
- nom (parser combinator)
- pest (PEG parser)
- Hand-rolled recursive descent

### Commands to Implement

- `tune ls [query]` - List items
- `tune list -a [query]` - List albums  
- `tune stats [query]` - Library statistics
- `tune info [item]` - Detailed item info
- `tune version` - Version information
- `tune config` - Show/validate config

### Technical Focus

- **Database abstraction**: Clean separation between storage and domain logic
- **Query optimization**: Indexes, prepared statements
- **Configuration**: Strong typing with serde
- **Testing**: Unit tests for query parser, integration tests for DB

### Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Query parser complexity | Start with subset, expand incrementally |
| SQLite performance issues | Profile early, add indexes strategically |
| Schema inflexibility | Design for future columns/tables from start |

---

## Phase 2: Tag Writing & Organization

**Duration**: 8-12 weeks  
**Prerequisites**: Phase 1 complete

### Goals

Enable modifying metadata and organizing files on disk.

### Deliverables

- Tag writing to all supported formats
- Path formatting based on templates
- File moving/copying operations
- Filesystem transaction safety
- Commands: `modify`, `move`, `update`

### Tag Writing Strategy

**Approach**: Read-modify-write pattern with validation
1. Read current tags
2. Apply modifications
3. Validate changes
4. Write atomically
5. Update database

**Formats to support**:
- MP3 (ID3v2.3 and v2.4)
- M4A/MP4
- FLAC
- OGG Vorbis
- Opus

### Path Templates

Support flexible path formatting:
```
Music/$albumartist/$album%aunique{}/$track $title
Music/Compilations/$album/$track $artist - $title
```

Template engine options:
- tera (Jinja2-like)
- handlebars
- Custom implementation

### File Operations

**Safety requirements**:
- Atomic moves when possible
- Preserve permissions and timestamps
- Handle cross-filesystem moves
- Rollback on errors
- Dry-run mode

### Commands to Implement

- `tune modify [query] field=value` - Update metadata
- `tune move [query]` - Move files to organized structure
- `tune update [query]` - Sync metadata from files to DB
- `tune write [query]` - Write DB metadata to files

### Plugins to Implement

- `inline` - Inline field computations
- `the` - Move articles (The, A) to end
- `export` - Export library to other formats

### Technical Focus

- **Atomic operations**: File operations that can rollback
- **Template safety**: Prevent directory traversal attacks
- **Batch processing**: Efficient bulk operations
- **Progress reporting**: User feedback for long operations

### Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Data loss from bugs | Extensive testing, mandatory dry-run first |
| Cross-platform path issues | Use std::path exclusively, test on Windows |
| Tag format edge cases | Comprehensive test suite with real files |

---

## Phase 3: Import Pipeline

**Duration**: 12-16 weeks  
**Prerequisites**: Phases 0, 1, 2 complete

### Goals

Build the complete import workflow with auto-tagging.

### Deliverables

- Full import state machine
- Interactive import TUI
- Duplicate detection
- Various import modes (copy, move, link)
- Resume interrupted imports
- Import history tracking

### Import State Machine

```
Scanning → Clustering → Searching → Selection → Review → Apply → Complete
                ↓                       ↓          ↓
              Skip                   Manual    Retry
```

Each state has:
- Entry/exit hooks (for plugins)
- User interaction points
- Persistence (for resume)

### Import Modes

1. **Standard**: Copy files, apply tags
2. **Move**: Move files instead of copy
3. **Link**: Create symlinks/hardlinks
4. **As-is**: Import without retagging
5. **Timid**: No file operations, DB only

### Duplicate Detection

Strategies:
- Acoustic fingerprinting (chromaprint)
- Metadata similarity
- File hash comparison
- User-defined rules

### Interactive Workflow

```
Found 3 matches for: The Beatles - Abbey Road

1. [96%] The Beatles - Abbey Road (1969, Original UK)
   12 tracks, Apple Records
   
2. [95%] The Beatles - Abbey Road (2019, Remaster)
   12 tracks, Apple Records
   
3. [92%] The Beatles - Abbey Road (1987, CD)
   12 tracks, Parlophone

Select (1-3), Skip (s), Manual search (m), or Quit (q): 1

Track mapping:
  01 Come Together.mp3 → Come Together ✓
  02 Something.mp3 → Something ✓
  ...
  
Apply these changes? (y/n/e=edit/r=retry): y

✓ Imported The Beatles - Abbey Road (12 tracks)
```

### Commands to Implement

- `tune import [path]` - Interactive import
- `tune import -q [path]` - Quiet mode (auto-accept high matches)
- `tune import -t [path]` - Timid mode
- `tune import -s [path]` - Singleton mode (no albums)

### Plugins to Implement

- `fromfilename` - Parse tags from filenames
- `importfeeds` - Generate playlists of imported items
- `filetote` - Import auxiliary files (lyrics, artwork folders)
- `scrub` - Remove extraneous tags
- `unimported` - Find files not in library

### Technical Focus

- **State persistence**: Save import session for resume
- **Concurrent imports**: Thread safety for parallel tracks
- **Plugin hooks**: Clean interface for extending behavior
- **Undo capability**: Track all changes for rollback

### Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Complex state management | Thorough testing of state transitions |
| Long-running operations | Progress bars, interruptibility |
| Import corruption | Comprehensive rollback mechanisms |

---

## Phase 4: Discogs Integration

**Duration**: 8-12 weeks  
**Prerequisites**: Phase 3 complete

### Goals

Add Discogs as alternative/supplementary metadata source.

### Deliverables

- Discogs API client with OAuth
- Search integration in import flow
- Discogs-specific data fields
- Plugin: `discogs` for auto-tagging

### Discogs API Challenges

1. **OAuth dance**: User authentication flow
2. **Rate limiting**: Stricter than MusicBrainz
3. **Data model differences**: Releases vs Masters
4. **Image handling**: Cover art downloads

### Integration Points

- Primary source (instead of MusicBrainz)
- Secondary source (if MB fails)
- Supplementary (combine data from both)
- Manual search override

### Commands to Implement

- `tune discogs-auth` - Authenticate with Discogs
- `tune import -A discogs [path]` - Use Discogs for auto-tagging

### Plugins to Implement

- `discogs` - Discogs auto-tagger
- `fetchart` - Download album artwork (Discogs source)
- `embedart` - Embed artwork in files
- `thumbnails` - Generate thumbnails

### Technical Focus

- **OAuth flow**: Store tokens securely
- **API client**: Rate limiting, retry logic
- **Image processing**: Format conversion, resizing
- **Cache management**: Artwork cache on disk

### Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| OAuth complexity | Use existing OAuth library |
| Rate limits too restrictive | Aggressive caching, queue requests |
| Image format issues | Use image processing library (image crate) |

---

## Phase 5: Plugin Ecosystem

**Duration**: 16-24 weeks  
**Prerequisites**: Phases 1-4 complete

### Goals

Implement remaining essential plugins and establish plugin architecture.

### Core Plugins to Implement

**Metadata Enhancement**:
- `lastgenre` - Fetch genres from Last.fm (optional)
- `mbsync` - Sync metadata updates from MusicBrainz

**Querying and Selection**:
- `fuzzy` - Fuzzy matching for queries
- `missing` - Find missing tracks in albums
- `smartplaylist` - Generate dynamic playlists

**Analysis and Reporting**:
- `summarize` - Generate library reports

**Playlist Management**:
- `playlist` - Manage M3U playlists

**Editing**:
- `edit` - Interactive metadata editor (TUI or external)

### Plugin Architecture

**Hook system**:
```rust
pub trait Plugin {
    fn name(&self) -> &str;
    fn on_import(&mut self, ctx: &ImportContext) -> Result<()>;
    fn on_modify(&mut self, ctx: &ModifyContext) -> Result<()>;
    fn commands(&self) -> Vec<Command>;
}
```

**Hook points**:
- `before_import` - Before import starts
- `after_import` - After import completes
- `before_modify` - Before metadata changes
- `after_modify` - After metadata changes
- `query_transform` - Modify queries
- `path_template` - Customize path generation

### Plugin Loading

**Compile-time** (Phase 5 approach):
- Plugins as separate crates in workspace
- Feature flags to enable/disable
- Zero runtime cost

**Runtime** (future consideration):
- Dynamic library loading
- More complex, enables user plugins
- Defer unless needed

### Configuration

Per-plugin configuration:
```toml
[plugins.fetchart]
sources = ["coverart", "discogs", "fanarttv"]
max_width = 1200

[plugins.smartplaylist]
relative_to = "/Music"
playlist_dir = "/Playlists"
```

### Technical Focus

- **Trait design**: Flexible hooks without performance cost
- **Error isolation**: Plugin failures don't crash app
- **Testing**: Each plugin has integration tests
- **Documentation**: Plugin development guide

### Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Plugin API too rigid | Design with extension points from start |
| Performance overhead | Use zero-cost abstractions (traits) |
| Plugin conflicts | Clear plugin ordering, conflict detection |

---

## Beyond Phase 5

### Features Not Planned (Initially)

- Web interface plugin (complex, lower priority)
- Audio analysis (replaygain, etc) - defer
- Transcoding plugin - defer
- Incremental import - defer
- Cloud storage backends - defer

### Possible Future Directions

1. **Web UI**: Separate project building on core library
2. **Library server**: Daemon mode for remote access
3. **Plugin marketplace**: Discovery and installation
4. **Format support**: AAC, WMA, APE, etc.
5. **Advanced features**: Duplicate merging, fuzzy clustering

### Maintenance and Polish

After Phase 5:
- Comprehensive documentation
- Performance optimization pass
- Edge case handling
- Windows/macOS testing
- Package for distribution
- Consider 1.0 release

---

## Decision Points

### After Phase 0

**Go/No-Go Decision**: Continue to Phase 1?

Evaluate:
- [ ] Matching accuracy acceptable (>90%)?
- [ ] Performance gains noticeable?
- [ ] Rust learning curve manageable?
- [ ] Enjoying the project?

If NO to any: Consider pivoting or stopping.

### After Phase 1

**Architecture Validation**: Is the foundation solid?

Evaluate:
- [ ] Database queries fast enough?
- [ ] Query language expressive enough?
- [ ] Code maintainable at this scale?

If NO: Refactor before proceeding.

### After Phase 3

**Feature Completion**: Ready for daily use?

Evaluate:
- [ ] Can replace Python beets for your workflow?
- [ ] Comfortable with reliability?
- [ ] Missing any critical features?

If YES: Consider using as daily driver (dogfooding).

---

## Success Metrics

### Phase 0 (PoC)
- Matching accuracy: >90% on test library
- Scan speed: >2x faster than Python
- Lines of code: <2000 (staying focused)

### Phase 1 (Core Library)
- Query performance: <100ms for typical queries on 10k items
- Database size: <20MB for 10k tracks
- Test coverage: >80% for core modules

### Phase 3 (Import)
- Import speed: >5 albums/minute
- Error rate: <1% of imports require manual intervention
- Crash rate: Zero data loss bugs

### Phase 5 (Plugins)
- Plugin count: 15+ implemented
- Documentation: All plugins documented with examples
- User satisfaction: Would use as primary tool

---

## Technical Debt Management

### Acceptable in Early Phases
- Hardcoded values (before config system)
- Missing error handling for edge cases
- Limited format support
- No optimization

### Must Fix Before Next Phase
- Unsafe code without documentation
- Panics in user-facing code
- Missing critical tests
- Architecture violations

### Continuous Refactoring
- Extract functions >50 lines
- Remove duplicated logic
- Improve error messages
- Add doc comments

---

## Learning Goals

Since this is also a Rust learning project:

### Phase 0-1 Focus
- Ownership and borrowing
- Error handling patterns
- Module organization
- Basic trait usage

### Phase 2-3 Focus
- Concurrent programming
- Unsafe code (if needed)
- Advanced traits (trait objects)
- Macro basics

### Phase 4-5 Focus
- Async Rust
- Design patterns
- Performance optimization
- FFI (if needed)

**Approach**: Learn as needed, don't over-architect early.

---

## Resources and References

### Beets Documentation
- [Beets reference](https://beets.readthedocs.io/)
- Plugin implementations (for reference)
- User forum discussions (feature priorities)

### Rust Libraries
- ratatui examples and patterns
- musicbrainz_rs documentation
- rusqlite best practices
- id3/mp4ameta/metaflac quirks

### Community
- #rust on various platforms
- Beets user community (for feedback)
- Code review requests (after major milestones)

---

## Appendix: Plugin Priority Matrix

| Plugin | Phase | Complexity | Value | Dependencies |
|--------|-------|------------|-------|--------------|
| inline | 2 | Low | High | Core |
| the | 2 | Low | Medium | Core |
| export | 2 | Low | Medium | Core |
| fromfilename | 3 | Medium | High | Import |
| importfeeds | 3 | Low | Medium | Import |
| filetote | 3 | Medium | High | Import |
| scrub | 3 | Low | High | Import |
| unimported | 3 | Medium | Medium | Import |
| discogs | 4 | High | High | Import, OAuth |
| fetchart | 4 | Medium | High | Import, Discogs |
| embedart | 4 | Medium | High | fetchart |
| thumbnails | 4 | Low | Low | fetchart |
| lastgenre | 5 | Medium | Low | API |
| mbsync | 5 | Low | Medium | MusicBrainz |
| fuzzy | 5 | Medium | High | Query |
| missing | 5 | Low | Medium | Query |
| smartplaylist | 5 | Medium | High | Query |
| summarize | 5 | Low | Low | Core |
| playlist | 5 | Low | Medium | Core |
| edit | 5 | High | Medium | TUI |
