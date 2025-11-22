<!--
SPDX-FileCopyrightText: (C) 2025 chris montgomery <chmont@protonmail.com>

SPDX-License-Identifier: AGPL-3.0-or-later
-->

# Beets Configuration Mapping

This document tracks which beets configuration options we plan to support, defer, or skip entirely.

## Import/Auto-Tagging Settings

### Matching Thresholds (HIGH PRIORITY - Phase 0/1)

| Beets Setting | Default | Phase | Notes |
|---------------|---------|-------|-------|
| `strong_rec_thresh` | 0.04 | Phase 0 | Distance threshold for auto-accept (we're using ≥98% = 0.02 distance) |
| `medium_rec_thresh` | 0.25 | Phase 1 | Mid-confidence suggestions |
| `rec_gap_thresh` | 0.25 | Phase 1 | Gap between top matches to require confirmation |
| `max_rec` | medium | Phase 3 | Per-penalty thresholds (complex) |
| `ignored` | [] | Phase 3 | Skip specific penalty types |
| `required` | [] | Phase 3 | Enforce required metadata fields |

**Phase 0 Note**: Hardcode `strong_rec_thresh` equivalent (98% = 0.02 distance). Make configurable in Phase 1.

### Import Workflow Behavior

| Beets Setting | Default | Phase | Notes |
|---------------|---------|-------|-------|
| `autotag` | yes | Phase 0 | Always enabled in Phase 0 PoC |
| `timid` | no | Phase 1 | Prompt for every match (useful for testing) |
| `quiet` | no | Phase 3 | Auto-apply without prompts |
| `quiet_fallback` | skip | Phase 3 | What to do when quiet mode can't decide |
| `none_rec_action` | ask | Phase 1 | Behavior when no matches found |
| `default_action` | apply | Phase 1 | What Enter key does at prompt |

**Phase 0**: Hardcode behavior similar to normal mode (prompt for low confidence only).

### Duplicate Handling

| Beets Setting | Default | Phase | Notes |
|---------------|---------|-------|-------|
| `duplicate_action` | ask | Phase 3+ | Requires database to detect duplicates |
| `duplicate_keys` | albumartist/album | Phase 3+ | How to identify duplicates |
| `duplicate_verbose_prompt` | no | Phase 3+ | Show track details in dup prompt |

**Note**: All duplicate detection requires Phase 1 database infrastructure.

### Data Processing

| Beets Setting | Default | Phase | Notes |
|---------------|---------|-------|-------|
| `from_scratch` | no | Phase 2 | Discard existing tags before applying |
| `languages` | [] | Phase 1 | Preferred language aliases |
| `ignored_alias_types` | [] | Phase 2 | Skip certain alias types |
| `group_albums` | no | Phase 1 | Use metadata vs. directories for clustering |

**Phase 0**: Cluster by directory + metadata (similar to `group_albums=no`).

**Important clarification on metadata inference**: Beets uses **existing tags** for MusicBrainz searches, NOT directory/filenames. Directories are only for **grouping** files into albums. When tags are missing/bad, beets searches with what exists (even "Unknown Artist") and lets the user manually refine the search. The **FromFilename** plugin (Phase 3+) provides fallback parsing for completely untagged files.

## Path & File Organization (Phase 2)

### Core Directory Structure

| Beets Setting | Default | Phase | Notes |
|---------------|---------|-------|-------|
| `directory` | ~/Music | Phase 2 | Library base path |
| `paths.default` | (template) | Phase 2 | Default album organization |
| `paths.comp` | (template) | Phase 2 | Compilations pattern |
| `paths.singleton` | (template) | Phase 2 | Non-album tracks |

**Phase 0/1**: No file moving/copying. Just metadata preview.

### Filename Management

| Beets Setting | Default | Phase | Notes |
|---------------|---------|-------|-------|
| `replace` | {...} | Phase 2 | Character replacement in filenames |
| `path_sep_replace` | _ | Phase 2 | What to use for path separators in names |
| `art_filename` | cover | Phase 4 | Cover art filename |
| `max_filename_length` | 0 (disabled) | Phase 2 | Truncate long filenames |
| `asciify_paths` | no | Phase 2 | Convert non-ASCII characters |

### Character Handling

| Beets Setting | Default | Phase | Notes |
|---------------|---------|-------|-------|
| `clutter` | [...] | Phase 2 | Files to ignore for empty-dir detection |
| `ignore_hidden` | yes | Phase 0 | Skip hidden files during scan |
| `ignore` | [...] | Phase 0 | Glob patterns to skip |

**Phase 0**: Implement basic `ignore_hidden` behavior in scanner.

## Metadata Handling

### Tag Writing (Phase 2)

| Beets Setting | Default | Phase | Notes |
|---------------|---------|-------|-------|
| `write` | yes | Phase 2 | Write tags to files (Phase 0: preview only) |
| `id3v23` | no | Phase 2 | Use ID3v2.3 vs v2.4 for MP3s |
| `artist_credit` | no | Phase 2 | Use credited vs. actual artist names |
| `original_date` | no | Phase 2 | Use original release date vs. selected version |
| `per_disc_numbering` | no | Phase 0 | Already implemented (disc/track sorting) |
| `va_name` | Various Artists | Phase 0 | Constant for compilations |

**Phase 0**: Hardcode sane defaults. `va_name` already used in scanner constants.

### Field Management

| Beets Setting | Default | Phase | Notes |
|---------------|---------|-------|-------|
| `overwrite_null` | {...} | Phase 2 | Which fields accept null overwrites |
| `set_fields` | {} | Phase 2 | Auto-set fields on import |
| `terminal_encoding` | auto | Phase 1 | Console text encoding |

### Disambiguation

| Beets Setting | Default | Phase | Notes |
|---------------|---------|-------|-------|
| `aunique` | {} | Phase 3 | Album disambiguation strategy |
| `sunique` | {} | Phase 3 | Singleton disambiguation strategy |

## UI/Interaction Settings

### Visual Customization

| Beets Setting | Default | Phase | Notes |
|---------------|---------|-------|-------|
| `color` | yes | Phase 1 | Enable terminal colors (ratatui handles) |
| `colors` | {...} | Phase 2 | Custom color definitions |
| `terminal_width` | 80 | N/A | ratatui handles terminal sizing |

### Import UI

| Beets Setting | Default | Phase | Notes |
|---------------|---------|-------|-------|
| `import.detail` | no | Phase 1 | Show all tracks vs. just changes |
| `import.length_diff_thresh` | 10.0 | Phase 1 | Highlight duration mismatches |
| `import.layout` | newline | Phase 1 | Match display layout |
| `import.indentation.*` | {...} | Phase 2 | Indentation settings |

### User Prompts

| Beets Setting | Default | Phase | Notes |
|---------------|---------|-------|-------|
| `bell` | no | Phase 2 | Ring bell for input (TUI less relevant) |
| `resume` | ask | Phase 3 | Resume interrupted imports |
| `log` | (none) | Phase 1 | Import activity log file |

## File Operations (Phase 2 - CRITICAL)

**These settings determine permanent file changes and must be carefully implemented.**

| Beets Setting | Default | Phase | Notes |
|---------------|---------|-------|-------|
| `copy` | yes | Phase 2 | Copy files into library |
| `move` | no | Phase 2 | Move instead of copy |
| `link` | no | Phase 2 | Use symbolic links |
| `hardlink` | no | Phase 2 | Use hard links |
| `reflink` | auto | Phase 3+ | Copy-on-write (Linux/macOS specific) |

**Phase 0/1**: No file operations. Preview only (dry-run mode).

**Phase 2 Implementation Strategy**:
1. Implement copy first (safest - preserves originals)
2. Add move with confirmation prompts
3. Add link/hardlink carefully (test thoroughly)
4. Defer reflink (platform-specific, lower priority)

### Incremental Processing

| Beets Setting | Default | Phase | Notes |
|---------------|---------|-------|-------|
| `incremental` | no | Phase 3 | Skip previously imported directories |
| `incremental_skip_later` | no | Phase 3 | Record skipped directories |

**Requires**: Database to track import history.

## Plugin Configuration

| Beets Setting | Default | Phase | Notes |
|---------------|---------|-------|-------|
| `plugins` | [] | Phase 5 | Space-separated plugin names |
| `pluginpath` | [] | Phase 5 | Custom plugin directories |
| `include` | [] | Phase 2 | Include additional config files |

## Configuration Priorities for Phase 0

**Must implement (hardcoded for now):**
- `strong_rec_thresh` equivalent (98% similarity = 0.02 distance)
- `ignore_hidden` (yes)
- `va_name` ("Various Artists" - already in scanner)
- `per_disc_numbering` (no - already implemented)

**Should be configurable by Phase 1:**
- `strong_rec_thresh` - matching threshold
- `timid` - prompt behavior
- `none_rec_action` - no match behavior
- `default_action` - default prompt action
- `languages` - preferred aliases
- `import.detail` - show all tracks vs changes only

**Defer to later phases:**
- All file operation settings (Phase 2)
- Path templates (Phase 2)
- Duplicate detection (Phase 3)
- Plugin system (Phase 5)

## Configuration File Format

**Phase 1 Decision Point**: TOML vs YAML

**Beets uses**: YAML
**Rust ecosystem prefers**: TOML (serde support excellent)

**Recommendation**: Use TOML for Rust idiomaticity, but document differences from beets.

Example structure:
```toml
[import]
strong_rec_thresh = 0.02  # 98% similarity
timid = false
detail = false

[paths]
default = "Music/$albumartist/$album/$track $title"

[ui]
color = true
```

## Decision: Configuration Scope by Phase

**Phase 0**: No configuration file. Hardcode sensible defaults.

**Phase 1**: Basic TOML config with import thresholds and behavior settings.

**Phase 2**: Add path templates and file operation settings.

**Phase 3**: Add duplicate detection and advanced matching.

**Phase 5**: Add plugin configuration.

## Notes on Critical Thresholds

The default `strong_rec_thresh = 0.04` in beets represents a **distance** (lower = better match).

**Beets distance scale**:
- 0.00 = perfect match
- 0.04 = strong recommendation (96% similarity)
- 0.25 = medium recommendation (75% similarity)

**Our percentage scale** (easier for users):
- 100% = perfect match
- 98% = strong (auto-apply threshold)
- 75% = medium (prompt user)

**Conversion**: `similarity = 1.0 - distance`

For Phase 1 config, expose as percentage (more intuitive) and convert internally to distance for scoring algorithm.
