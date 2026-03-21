---
number: 14
title: "Music Theory Computational Tools"
author: "Duncan McGreggor"
component: All
tags: [change-me]
created: 2026-03-21
updated: 2026-03-21
state: Active
supersedes: null
superseded-by: null
version: 1.0
---

# Music Theory Computational Tools

## Context

The `ai-music-theory` MCP server currently serves 30 knowledge tools (concept search, source material access, graph traversal). It has zero computational music theory capability — it can look up what a "dominant seventh chord" IS, but cannot compute the notes of G7.

The `music-comp-mt` library (v0.4.0, just published to crates.io) provides all the computation. This plan wires it into the MCP server as 9 new tools, enabling Claude to compute music theory facts instead of hallucinating them.

## Files to Modify

| File | Change |
|------|--------|
| `Cargo.toml` (workspace root) | Add `music-comp-mt` to workspace dependencies |
| `crates/server/Cargo.toml` | Add `music-comp-mt` dependency |
| `crates/server/src/tools/mod.rs` | Add `pub mod music_theory;` and re-exports |
| `crates/server/src/tools/music_theory.rs` | **NEW** — 9 tool handler functions |
| `crates/server/src/server.rs` | Add `MusicTheoryToolsRegistry`, register in `build_server()` |

## Dependencies

Add to workspace `Cargo.toml`:

```toml
music-comp-mt = { version = "0.4", features = ["serde"] }
```

Add to `crates/server/Cargo.toml`:

```toml
music-comp-mt = { workspace = true }
```

## Tool Implementations

All 9 tools live in `crates/server/src/tools/music_theory.rs`. Each follows the existing pattern: deserialize args struct → call library → serialize response.

### Tool 1: `get_scale_notes`

- **Args:** `tonic: String`, `mode: String`, `direction: Option<String>`
- **Library:** `Scale::from_regex_in_direction(&format!("{} {}", tonic, mode), direction)`
- **Response:** `{ notes: [String], scale_type: String, mode: String }`

### Tool 2: `get_chord_notes`

- **Args:** `root: String`, `quality: String`, `number: Option<String>`, `inversion: Option<u8>`
- **Library:** `Chord::from_regex(&format!("{} {} {}", root, quality, number))`
- **Response:** `{ notes: [String], quality: String, number: String }`

### Tool 3: `get_interval`

- **Args:** `from: String`, `to: String`
- **Library:** `Interval::between(&from_pitch, &to_pitch)`
- **Response:** `{ semitones: u8, quality: String, number: String }`

### Tool 4: `transpose`

- **Args:** `notes: [String]`, `semitones: u8`, `direction: String` ("up"/"down")
- **Library:** `pitch.transpose_up/down(&interval)` for each note
- **Response:** `{ original: [String], transposed: [String] }`

### Tool 5: `get_diatonic_chords`

- **Args:** `tonic: String`, `mode: String`, `chord_type: Option<String>` ("triad"/"seventh")
- **Library:** `harmony::diatonic_triads/sevenths(tonic, mode)`
- **Response:** `{ chords: [{ degree: u8, roman: String, root: String, quality: String, notes: [String] }] }`

### Tool 6: `identify_chord`

- **Args:** `notes: [String]`
- **Library:** `Chord::identify(&pitches)`
- **Response:** `{ matches: [{ root: String, quality: String, number: String, inversion: u8 }] }`

### Tool 7: `identify_scale`

- **Args:** `notes: [String]`
- **Library:** `Scale::identify(&pitches)`
- **Response:** `{ matches: [{ tonic: String, mode: String }] }`

### Tool 8: `check_enharmonic`

- **Args:** `note_a: String`, `note_b: String`
- **Library:** `pitch_a.is_enharmonic_to(&pitch_b)`
- **Response:** `{ equivalent: bool, semitone_value: u8 }`

### Tool 9: `analyze_roman_numerals`

- **Args:** `key_tonic: String`, `key_mode: String`, `chords: [String]`
- **Library:** `analysis::roman_numeral()` for each chord
- **Response:** `{ analysis: [{ chord: String, roman: String, degree: u8 }] }`

## Registry (in server.rs)

```rust
struct MusicTheoryToolsRegistry;

impl ToolRegistry for MusicTheoryToolsRegistry {
    fn tools(&self) -> Vec<Tool> { /* 9 tools with JSON schemas */ }
    fn call(&self, name: &str, args: Value) -> Option<ToolResult> { /* dispatch */ }
}
```

No `AppState` needed — these tools are pure computation. The registry still follows the pattern but doesn't clone state.

Register in `build_server()`:

```rust
let music_theory_tools = MusicTheoryToolsRegistry;
let registry = CompositeRegistry::new()
    .add(concept_tools)
    // ... existing ...
    .add(health_tools)
    .add(music_theory_tools);  // NEW
```

## Pitch Parsing Helper

All tools accept note names as strings ("C#", "Eb", "F"). A shared helper parses them:

```rust
fn parse_pitch(s: &str) -> Result<music_comp_mt::note::Pitch> {
    music_comp_mt::note::Pitch::try_parse(s)
        .ok_or_else(|| Error::InvalidInput(format!("Invalid pitch: {}", s)))
}
```

## Testing

1. `cargo build` in the MCP server project — verifies compilation
2. `cargo test` — run existing tests (should still pass)
3. Manual test via curl: initialize MCP session, call `get_scale_notes` with `{"tonic":"C","mode":"major"}`, verify response
4. Manual test: call `get_interval` with `{"from":"F","to":"B"}`, verify returns "Augmented Fourth"
5. Manual test: call `analyze_roman_numerals` with `{"key_tonic":"C","key_mode":"major","chords":["C major","G major","A minor"]}`, verify returns I, V, vi

## Verification

```bash
cd /Users/oubiwann/lab/music-comp/ai-music-theory/mcp-server
cargo build --features full
cargo test
# Then start server and test via curl:
cargo run --features "fts graph http" -- serve --transport http
# In another terminal, run MCP initialization + tool calls
```
