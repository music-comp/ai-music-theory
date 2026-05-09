//! Music Theory MCP Server — registry-based implementation.
//!
//! Replaces the monolithic `#[tool_router]` approach with composable
//! [`ToolRegistry`] implementations grouped by domain, served by
//! [`FabrykMcpServer`].

use std::sync::Arc;

use serde_json::{json, Value};

use fabryk_mcp::{
    make_tool,
    model::{ErrorCode, ErrorData, Tool},
    serialize_response, FabrykMcpServer, McpErrorContextExt, ToolRegistry, ToolResult,
};

use crate::state::AppState;
use crate::tools;

// ============================================================================
// MusicTheoryToolsRegistry (9 tools — pure computation, no state needed)
// ============================================================================

struct MusicTheoryToolsRegistry;

impl ToolRegistry for MusicTheoryToolsRegistry {
    fn tools(&self) -> Vec<Tool> {
        vec![
            make_tool(
                "get_scale_notes",
                "Compute the notes of a musical scale given a tonic and mode",
                json!({
                    "type": "object",
                    "properties": {
                        "tonic": {
                            "type": "string",
                            "description": "Root note (e.g. 'C', 'F#', 'Bb')"
                        },
                        "mode": {
                            "type": "string",
                            "description": "Scale mode (e.g. 'Major', 'Dorian', 'Harmonic Minor', 'Pentatonic Major')"
                        },
                        "direction": {
                            "type": "string",
                            "enum": ["ascending", "descending"],
                            "description": "Scale direction (default: ascending)"
                        }
                    },
                    "required": ["tonic", "mode"]
                }),
            ),
            make_tool(
                "get_chord_notes",
                "Compute the notes of a chord given root, quality, and optional \
                 number/inversion. Note: the inversion parameter uses traditional \
                 octave-transfer (bass-note rotation), correct for close-voiced \
                 tertian chords. For quintal/quartal chord inversions via Tymoczko \
                 interscalar transposition, use get_oth_chord_scale.",
                json!({
                    "type": "object",
                    "properties": {
                        "root": {
                            "type": "string",
                            "description": "Root note (e.g. 'C', 'F#', 'Bb')"
                        },
                        "quality": {
                            "type": "string",
                            "description": "Chord quality (e.g. 'Major', 'Minor', 'Diminished', 'Dominant')"
                        },
                        "number": {
                            "type": "string",
                            "description": "Chord number (e.g. 'Seventh', 'Ninth', 'Triad')"
                        },
                        "inversion": {
                            "type": "integer",
                            "description": "Traditional bass-note inversion (0 = root position, \
                                            1 = first inversion, etc.). Rotates which chord \
                                            tone is in the bass — correct for close-voiced \
                                            tertian chords (triads, seventh chords). For \
                                            Tymoczko interscalar transposition of spread \
                                            voicings (quintal/quartal chords), use \
                                            get_oth_chord_scale instead."
                        }
                    },
                    "required": ["root", "quality"]
                }),
            ),
            make_tool(
                "get_interval",
                "Calculate the interval between two notes",
                json!({
                    "type": "object",
                    "properties": {
                        "from": {
                            "type": "string",
                            "description": "Starting note (e.g. 'C', 'F#')"
                        },
                        "to": {
                            "type": "string",
                            "description": "Ending note (e.g. 'G', 'Bb')"
                        }
                    },
                    "required": ["from", "to"]
                }),
            ),
            make_tool(
                "transpose",
                "Transpose a list of notes up or down by a number of semitones",
                json!({
                    "type": "object",
                    "properties": {
                        "notes": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Notes to transpose (e.g. ['C', 'E', 'G'])"
                        },
                        "semitones": {
                            "type": "integer",
                            "description": "Number of semitones to transpose"
                        },
                        "direction": {
                            "type": "string",
                            "enum": ["up", "down"],
                            "description": "Transpose direction (default: up)"
                        }
                    },
                    "required": ["notes", "semitones"]
                }),
            ),
            make_tool(
                "get_diatonic_chords",
                "Get all diatonic chords (triads or sevenths) for a key",
                json!({
                    "type": "object",
                    "properties": {
                        "tonic": {
                            "type": "string",
                            "description": "Key tonic (e.g. 'C', 'G', 'Bb')"
                        },
                        "mode": {
                            "type": "string",
                            "description": "Scale mode (e.g. 'Ionian', 'Dorian', 'Aeolian')"
                        },
                        "chord_type": {
                            "type": "string",
                            "enum": ["triad", "seventh"],
                            "description": "Whether to build triads or seventh chords (default: triad)"
                        }
                    },
                    "required": ["tonic", "mode"]
                }),
            ),
            make_tool(
                "identify_chord",
                "Identify possible chords from a set of notes",
                json!({
                    "type": "object",
                    "properties": {
                        "notes": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Notes to identify (e.g. ['C', 'E', 'G'])"
                        }
                    },
                    "required": ["notes"]
                }),
            ),
            make_tool(
                "identify_scale",
                "Identify possible scales/modes from a set of notes",
                json!({
                    "type": "object",
                    "properties": {
                        "notes": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Notes to identify (e.g. ['C', 'D', 'E', 'F', 'G', 'A', 'B'])"
                        }
                    },
                    "required": ["notes"]
                }),
            ),
            make_tool(
                "check_enharmonic",
                "Check if two notes are enharmonically equivalent",
                json!({
                    "type": "object",
                    "properties": {
                        "note_a": {
                            "type": "string",
                            "description": "First note (e.g. 'C#')"
                        },
                        "note_b": {
                            "type": "string",
                            "description": "Second note (e.g. 'Db')"
                        }
                    },
                    "required": ["note_a", "note_b"]
                }),
            ),
            make_tool(
                "analyze_roman_numerals",
                "Analyze chords in a key context, returning Roman numeral labels",
                json!({
                    "type": "object",
                    "properties": {
                        "key_tonic": {
                            "type": "string",
                            "description": "Key tonic (e.g. 'C', 'G')"
                        },
                        "key_mode": {
                            "type": "string",
                            "description": "Key mode (e.g. 'Major', 'Ionian', 'Aeolian')"
                        },
                        "chords": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Chords to analyze (e.g. ['C Major', 'G Major', 'A Minor'])"
                        }
                    },
                    "required": ["key_tonic", "key_mode", "chords"]
                }),
            ),
        ]
    }

    fn call(&self, name: &str, args: Value) -> Option<ToolResult> {
        match name {
            "get_scale_notes" => Some(Box::pin(async move {
                let args: tools::music_theory::GetScaleNotesArgs = serde_json::from_value(args)
                    .map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::music_theory::get_scale_notes(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error computing scale"))?;
                serialize_response(&response)
            })),
            "get_chord_notes" => Some(Box::pin(async move {
                let args: tools::music_theory::GetChordNotesArgs = serde_json::from_value(args)
                    .map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::music_theory::get_chord_notes(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error computing chord"))?;
                serialize_response(&response)
            })),
            "get_interval" => Some(Box::pin(async move {
                let args: tools::music_theory::GetIntervalArgs = serde_json::from_value(args)
                    .map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::music_theory::get_interval(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error computing interval"))?;
                serialize_response(&response)
            })),
            "transpose" => Some(Box::pin(async move {
                let args: tools::music_theory::TransposeArgs = serde_json::from_value(args)
                    .map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::music_theory::transpose(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error transposing"))?;
                serialize_response(&response)
            })),
            "get_diatonic_chords" => Some(Box::pin(async move {
                let args: tools::music_theory::GetDiatonicChordsArgs = serde_json::from_value(args)
                    .map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::music_theory::get_diatonic_chords(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error computing diatonic chords"))?;
                serialize_response(&response)
            })),
            "identify_chord" => Some(Box::pin(async move {
                let args: tools::music_theory::IdentifyChordArgs = serde_json::from_value(args)
                    .map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::music_theory::identify_chord(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error identifying chord"))?;
                serialize_response(&response)
            })),
            "identify_scale" => Some(Box::pin(async move {
                let args: tools::music_theory::IdentifyScaleArgs = serde_json::from_value(args)
                    .map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::music_theory::identify_scale(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error identifying scale"))?;
                serialize_response(&response)
            })),
            "check_enharmonic" => Some(Box::pin(async move {
                let args: tools::music_theory::CheckEnharmonicArgs = serde_json::from_value(args)
                    .map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let response = tools::music_theory::check_enharmonic(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error checking enharmonic"))?;
                serialize_response(&response)
            })),
            "analyze_roman_numerals" => Some(Box::pin(async move {
                let args: tools::music_theory::AnalyzeRomanNumeralsArgs =
                    serde_json::from_value(args).map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::music_theory::analyze_roman_numerals(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error analyzing roman numerals"))?;
                serialize_response(&response)
            })),
            _ => None,
        }
    }
}

// ============================================================================
// OthToolsRegistry (12 tools — Open Tone Harmony, quintal/quartal analysis)
// ============================================================================

struct OthToolsRegistry;

impl ToolRegistry for OthToolsRegistry {
    fn tools(&self) -> Vec<Tool> {
        vec![
            // ---- Tier 1: Core (5 tools) ----
            make_tool(
                "get_oth_orbit_info",
                "Get complete information about an OTH orbit: modes, parent scales, \
                 step sequence, Forte number, prime form, degree, orbit size, fiber class, \
                 and quartal label",
                json!({
                    "type": "object",
                    "properties": {
                        "orbit": {
                            "type": "string",
                            "description": "Orbit identifier: quintal label (e.g. 'Q777'), \
                                            quartal label (e.g. 'Q555'), or interval structure \
                                            (e.g. '7,7,7')"
                        }
                    },
                    "required": ["orbit"]
                }),
            ),
            make_tool(
                "list_oth_orbits",
                "List all 14 OTH orbits with full data, optionally filtered by \
                 step-vocabulary cluster",
                json!({
                    "type": "object",
                    "properties": {
                        "cluster": {
                            "type": "string",
                            "enum": [
                                "NoSemitoneNoTritone",
                                "ContainsSemitone",
                                "EvenStepsOnly",
                                "ContainsTritoneStep"
                            ],
                            "description": "Optional step-vocabulary cluster filter"
                        }
                    }
                }),
            ),
            make_tool(
                "get_oth_parent_scales",
                "Find traditional parent scales containing an OTH chord's pitch classes",
                json!({
                    "type": "object",
                    "properties": {
                        "orbit": {
                            "type": "string",
                            "description": "Orbit identifier (e.g. 'Q777'). \
                                            Mutually exclusive with pcs; orbit takes precedence."
                        },
                        "pcs": {
                            "type": "array",
                            "items": { "type": "integer", "minimum": 0, "maximum": 11 },
                            "minItems": 4,
                            "maxItems": 4,
                            "description": "Four pitch classes (0-11). \
                                            Mutually exclusive with orbit."
                        }
                    }
                }),
            ),
            make_tool(
                "get_oth_chord_info",
                "Identify which OTH orbit a 4-note chord belongs to, or explain why \
                 it is outside the [6,8] space",
                json!({
                    "type": "object",
                    "properties": {
                        "pcs": {
                            "type": "array",
                            "items": { "type": "integer", "minimum": 0, "maximum": 11 },
                            "minItems": 4,
                            "maxItems": 4,
                            "description": "Four pitch classes (0-11)"
                        },
                        "notes": {
                            "type": "array",
                            "items": { "type": "string" },
                            "minItems": 4,
                            "maxItems": 4,
                            "description": "Four note names (e.g. ['C', 'D', 'G', 'A'])"
                        }
                    }
                }),
            ),
            make_tool(
                "get_oth_chord_scale",
                "Compute the Tymoczko chord scale and full interscalar transposition \
                 inversion cycle for a voiced chord. This is the geometrically correct \
                 inversion method for spread voicings (quintal/quartal stacks). Each \
                 inversion step moves every voice to the next chord-scale degree in its \
                 local register. NOT the same as traditional bass-note inversion — for \
                 that, use get_chord_notes with the inversion parameter.",
                json!({
                    "type": "object",
                    "properties": {
                        "pitches": {
                            "type": "array",
                            "items": { "type": "integer", "minimum": 0, "maximum": 127 },
                            "minItems": 4,
                            "maxItems": 4,
                            "description": "Four MIDI pitches in ascending order \
                                            (e.g. [48, 55, 62, 69] for C3-G3-D4-A4)"
                        }
                    },
                    "required": ["pitches"]
                }),
            ),
            // ---- Tier 2: Analytical (4 tools) ----
            make_tool(
                "list_oth_modes",
                "List all 52 distinct OTH modes with optional filtering by opening \
                 interval, step-vocabulary cluster, or orbit",
                json!({
                    "type": "object",
                    "properties": {
                        "opening_interval": {
                            "type": "integer",
                            "minimum": 1,
                            "maximum": 6,
                            "description": "Filter by opening interval size (1-6)"
                        },
                        "cluster": {
                            "type": "string",
                            "enum": [
                                "NoSemitoneNoTritone",
                                "ContainsSemitone",
                                "EvenStepsOnly",
                                "ContainsTritoneStep"
                            ],
                            "description": "Filter by step-vocabulary cluster"
                        },
                        "orbit": {
                            "type": "string",
                            "description": "Filter to modes of a specific orbit \
                                            (e.g. 'Q777', 'Q555', '7,7,7')"
                        }
                    }
                }),
            ),
            make_tool(
                "get_oth_distance",
                "Compute geodesic distance between two chords in the [6,8] base \
                 space B (228 chords)",
                json!({
                    "type": "object",
                    "properties": {
                        "from": {
                            "type": "object",
                            "description": "Source chord: {\"pcs\": [0,2,7,9]} or \
                                            {\"notes\": [\"C\",\"D\",\"G\",\"A\"]}",
                            "properties": {
                                "pcs": {
                                    "type": "array",
                                    "items": { "type": "integer", "minimum": 0, "maximum": 11 }
                                },
                                "notes": {
                                    "type": "array",
                                    "items": { "type": "string" }
                                }
                            }
                        },
                        "to": {
                            "type": "object",
                            "description": "Target chord, same format as 'from'",
                            "properties": {
                                "pcs": {
                                    "type": "array",
                                    "items": { "type": "integer", "minimum": 0, "maximum": 11 }
                                },
                                "notes": {
                                    "type": "array",
                                    "items": { "type": "string" }
                                }
                            }
                        }
                    },
                    "required": ["from", "to"]
                }),
            ),
            make_tool(
                "get_oth_neighbors",
                "Get all adjacent chords (distance 1) for a chord in the [6,8] base \
                 space, with voice movement details and orbit distribution",
                json!({
                    "type": "object",
                    "properties": {
                        "pcs": {
                            "type": "array",
                            "items": { "type": "integer", "minimum": 0, "maximum": 11 },
                            "minItems": 4,
                            "maxItems": 4,
                            "description": "Four pitch classes (0-11)"
                        },
                        "notes": {
                            "type": "array",
                            "items": { "type": "string" },
                            "minItems": 4,
                            "maxItems": 4,
                            "description": "Four note names"
                        },
                        "orbit": {
                            "type": "string",
                            "description": "Optional: filter neighbors to this orbit"
                        }
                    }
                }),
            ),
            make_tool(
                "verify_oth_properties",
                "Run mathematical verification checks on OTH properties: fiber-mode \
                 connection, multiset uniqueness, Universal L1 Law, quartal/quintal duality",
                json!({
                    "type": "object",
                    "properties": {
                        "check": {
                            "type": "string",
                            "enum": ["all", "fiber_modes", "multisets", "l1_law", "duality"],
                            "description": "Which check to run (default: 'all')"
                        }
                    }
                }),
            ),
            // ---- Tier 3: Exploratory (3 tools) ----
            make_tool(
                "get_oth_geodesics",
                "Find all shortest paths between two chords in the [6,8] space \
                 with step-by-step voice movement analysis",
                json!({
                    "type": "object",
                    "properties": {
                        "from": {
                            "type": "object",
                            "description": "Source chord: {\"pcs\": [...]} or {\"notes\": [...]}",
                            "properties": {
                                "pcs": {
                                    "type": "array",
                                    "items": { "type": "integer", "minimum": 0, "maximum": 11 }
                                },
                                "notes": {
                                    "type": "array",
                                    "items": { "type": "string" }
                                }
                            }
                        },
                        "to": {
                            "type": "object",
                            "description": "Target chord, same format as 'from'",
                            "properties": {
                                "pcs": {
                                    "type": "array",
                                    "items": { "type": "integer", "minimum": 0, "maximum": 11 }
                                },
                                "notes": {
                                    "type": "array",
                                    "items": { "type": "string" }
                                }
                            }
                        },
                        "max_paths": {
                            "type": "integer",
                            "minimum": 1,
                            "description": "Maximum geodesics to return (default: 10)"
                        }
                    },
                    "required": ["from", "to"]
                }),
            ),
            make_tool(
                "get_oth_crossroads",
                "Return the 6 crossroads chords (Q686 orbit) with betweenness \
                 centrality, T6 partners, and structural properties",
                json!({
                    "type": "object",
                    "properties": {}
                }),
            ),
            make_tool(
                "find_oth_modes_by_opening",
                "Find OTH modes grouped by opening interval for exploratory analysis",
                json!({
                    "type": "object",
                    "properties": {
                        "interval": {
                            "type": "integer",
                            "minimum": 1,
                            "maximum": 6,
                            "description": "Specific opening interval (1-6). \
                                            If omitted, returns all grouped."
                        }
                    }
                }),
            ),
        ]
    }

    fn call(&self, name: &str, args: Value) -> Option<ToolResult> {
        match name {
            // ---- Tier 1 ----
            "get_oth_orbit_info" => Some(Box::pin(async move {
                let args: tools::oth::GetOthOrbitInfoArgs =
                    serde_json::from_value(args).map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::oth::get_oth_orbit_info(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error getting orbit info"))?;
                serialize_response(&response)
            })),
            "list_oth_orbits" => Some(Box::pin(async move {
                let args: tools::oth::ListOthOrbitsArgs =
                    serde_json::from_value(args).map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::oth::list_oth_orbits(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error listing orbits"))?;
                serialize_response(&response)
            })),
            "get_oth_parent_scales" => Some(Box::pin(async move {
                let args: tools::oth::GetOthParentScalesArgs = serde_json::from_value(args)
                    .map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::oth::get_oth_parent_scales(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error getting parent scales"))?;
                serialize_response(&response)
            })),
            "get_oth_chord_info" => Some(Box::pin(async move {
                let args: tools::oth::GetOthChordInfoArgs =
                    serde_json::from_value(args).map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::oth::get_oth_chord_info(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error identifying chord"))?;
                serialize_response(&response)
            })),
            "get_oth_chord_scale" => Some(Box::pin(async move {
                let args: tools::oth::GetOthChordScaleArgs =
                    serde_json::from_value(args).map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::oth::get_oth_chord_scale(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error computing chord scale"))?;
                serialize_response(&response)
            })),
            // ---- Tier 2 ----
            "list_oth_modes" => Some(Box::pin(async move {
                let args: tools::oth::ListOthModesArgs =
                    serde_json::from_value(args).map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::oth::list_oth_modes(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error listing modes"))?;
                serialize_response(&response)
            })),
            "get_oth_distance" => Some(Box::pin(async move {
                let args: tools::oth::GetOthDistanceArgs =
                    serde_json::from_value(args).map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::oth::get_oth_distance(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error computing distance"))?;
                serialize_response(&response)
            })),
            "get_oth_neighbors" => Some(Box::pin(async move {
                let args: tools::oth::GetOthNeighborsArgs =
                    serde_json::from_value(args).map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::oth::get_oth_neighbors(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error getting neighbors"))?;
                serialize_response(&response)
            })),
            "verify_oth_properties" => Some(Box::pin(async move {
                let args: tools::oth::VerifyOthPropertiesArgs = serde_json::from_value(args)
                    .map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::oth::verify_oth_properties(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error verifying properties"))?;
                serialize_response(&response)
            })),
            // ---- Tier 3 ----
            "get_oth_geodesics" => Some(Box::pin(async move {
                let args: tools::oth::GetOthGeodesicsArgs =
                    serde_json::from_value(args).map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::oth::get_oth_geodesics(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error computing geodesics"))?;
                serialize_response(&response)
            })),
            "get_oth_crossroads" => Some(Box::pin(async move {
                let args: tools::oth::GetOthCrossroadsArgs =
                    serde_json::from_value(args).map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::oth::get_oth_crossroads(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error getting crossroads"))?;
                serialize_response(&response)
            })),
            "find_oth_modes_by_opening" => Some(Box::pin(async move {
                let args: tools::oth::FindOthModesByOpeningArgs = serde_json::from_value(args)
                    .map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::oth::find_oth_modes_by_opening(args)
                    .map_err(|e| e.to_mcp_error_with_context("Error finding modes by opening"))?;
                serialize_response(&response)
            })),
            _ => None,
        }
    }
}

// ============================================================================
// Server builder
// ============================================================================

#[cfg(feature = "graph")]
use fabryk_mcp::tier_confidence_schema;

/// Collect service handles from AppState for health reporting.
#[allow(clippy::vec_init_then_push, unused_mut)]
fn collect_service_handles(_state: &AppState) -> Vec<fabryk::core::service::ServiceHandle> {
    let mut svcs = Vec::new();
    #[cfg(feature = "fts")]
    svcs.push(_state.fts.service().clone());
    #[cfg(feature = "graph")]
    svcs.push(_state.graph.service().clone());
    #[cfg(feature = "vector")]
    svcs.push(_state.vector.service().clone());
    svcs
}

/// Build the Music Theory MCP server from application state.
///
/// Composes domain-specific tool registries and resources into a single
/// [`FabrykMcpServer`] ready for stdio or HTTP transport.
///
/// # Graph tools
///
/// Graph tools are provided by fabryk's [`GraphTools`](fabryk_mcp::graph::GraphTools)
/// with custom tool names (via `.with_names()`), a domain-specific node filter
/// (via `.with_node_filter()`), and extra schema properties for tier/confidence
/// filtering (via `.with_extra_schema()`).
///
/// The 16 original tool names are preserved:
/// `graph_status`, `graph_stats`, `graph_validate`, `get_node`, `get_node_edges`,
/// `get_related_concepts`, `find_concept_path`, `get_prerequisites`,
/// `get_concept_neighborhood`, `get_dependents`, `get_central_concepts`,
/// `get_concept_sources`, `get_concept_variants`, `find_bridge_concepts`,
/// `get_source_coverage`, `get_learning_path`.
///
/// Fabryk's `graph_bridges` tool is also exposed (an additional 17th tool that
/// finds bridge nodes by betweenness centrality).
pub fn build_server(state: AppState) -> FabrykMcpServer {
    use fabryk_mcp::content::{
        ContentTools, FsContentItemProvider, FsGuideProvider, FsQuestionSearchProvider,
        FsSourceProvider, GuideTools, SourceTools,
    };
    use fabryk_mcp::fts::FtsTools;
    #[cfg(feature = "graph")]
    use fabryk_mcp::graph::GraphTools;
    use std::collections::HashMap;

    let concept_cards_path = state.config.paths.concept_cards_path().unwrap_or_default();
    let content_provider = Arc::new(
        FsContentItemProvider::new(&concept_cards_path)
            .with_content_type_name("concept", "concepts"),
    );
    let concept_tools = ContentTools::with_shared(content_provider)
        .with_names(HashMap::from([
            ("list".to_string(), "list_concepts".to_string()),
            ("get".to_string(), "get_concept".to_string()),
            ("categories".to_string(), "list_categories".to_string()),
        ]))
        .with_descriptions(HashMap::from([
            (
                "list".to_string(),
                "List concept cards with optional filtering by category, tier, subcategory, or source".to_string(),
            ),
            (
                "get".to_string(),
                "Retrieve a specific concept card".to_string(),
            ),
            (
                "categories".to_string(),
                "List all distinct concept categories with counts".to_string(),
            ),
        ]))
        .with_get_id_field("concept_id")
        .with_extra_list_schema(serde_json::json!({
            "tier": {
                "type": "string",
                "enum": ["foundational", "intermediate", "advanced"],
                "description": "Filter by prerequisite depth tier"
            },
            "subcategory": {
                "type": "string",
                "description": "Filter by subcategory"
            },
            "source": {
                "type": "string",
                "description": "Filter by source text"
            }
        }));

    let sources_md_path = state.config.paths.sources_md_path().unwrap_or_default();
    let source_provider = Arc::new(FsSourceProvider::new(&sources_md_path));
    let source_tools = SourceTools::with_shared(source_provider)
        .with_names(HashMap::from([
            ("sources_list".to_string(), "list_sources".to_string()),
            (
                "sources_chapters".to_string(),
                "list_source_chapters".to_string(),
            ),
            (
                "sources_get_chapter".to_string(),
                "get_source_chapter".to_string(),
            ),
            (
                "sources_check_availability".to_string(),
                "check_source_availability".to_string(),
            ),
            (
                "sources_get_path".to_string(),
                "get_source_pdf_path".to_string(),
            ),
        ]))
        .with_descriptions(HashMap::from([
            (
                "sources_list".to_string(),
                "List all available source materials with metadata".to_string(),
            ),
            (
                "sources_check_availability".to_string(),
                "Check availability status of a source (indexed/converted/exists)".to_string(),
            ),
            (
                "sources_chapters".to_string(),
                "List all chapters for a source material".to_string(),
            ),
            (
                "sources_get_chapter".to_string(),
                "Retrieve a specific chapter from a source material".to_string(),
            ),
            (
                "sources_get_path".to_string(),
                "Get filesystem path to original PDF/EPUB for a source".to_string(),
            ),
        ]));

    let guides_path = state.config.paths.guides_path().unwrap_or_default();
    let guide_provider = FsGuideProvider::new(&guides_path);
    let guide_tools = GuideTools::new(guide_provider);

    let search_backend = state.search_backend();
    let search_tools = FtsTools::with_shared(search_backend)
        .with_names(HashMap::from([(
            "search".to_string(),
            "search_concepts".to_string(),
        )]))
        .with_descriptions(HashMap::from([(
            "search".to_string(),
            "Search concept cards with full-text search and relevance ranking".to_string(),
        )]))
        .with_extra_search_schema(serde_json::json!({
            "tier": {
                "type": "string",
                "enum": ["foundational", "intermediate", "advanced"],
                "description": "Filter by prerequisite depth tier"
            },
            "subcategory": {
                "type": "string",
                "description": "Filter by subcategory"
            },
            "min_confidence": {
                "type": "string",
                "enum": ["high", "medium", "low"],
                "description": "Minimum extraction confidence (high includes only high; medium includes high+medium; low includes all)"
            },
            "content_types": {
                "type": "array",
                "items": {"type": "string"},
                "description": "Optional content type filter (concept_card, source_chapter, unified_concept, guide)"
            }
        }));

    // Fabryk's SemanticSearchTools handles keyword/vector/hybrid dispatch with
    // RRF fusion. The vector_slot is a tokio::sync::RwLock that starts empty and
    // is populated when the background vector build completes.
    #[cfg(feature = "vector")]
    let semantic_search = {
        use fabryk_mcp::semantic::SemanticSearchTools;
        SemanticSearchTools::with_vector_slot(
            state.search_backend(),
            Arc::clone(&state.vector_slot),
        )
    };
    #[cfg(not(feature = "vector"))]
    let semantic_search = {
        use fabryk_mcp::semantic::SemanticSearchTools;
        SemanticSearchTools::new(state.search_backend(), None)
    };
    let question_provider = FsQuestionSearchProvider::new(&concept_cards_path);
    let question_search = fabryk_mcp::content::QuestionSearchTools::new(question_provider);
    let music_theory_tools = MusicTheoryToolsRegistry;
    let oth_tools = OthToolsRegistry;

    // Build backend probes for health diagnostics
    let search_backend_for_probe = state.search_backend();
    #[allow(unused_mut)]
    let mut probes: Vec<Arc<dyn fabryk::core::BackendProbe>> =
        vec![fabryk::fts::search_probe(search_backend_for_probe)];

    #[cfg(feature = "vector")]
    {
        if let Ok(guard) = state.vector.inner().read() {
            if let Some(ref backend) = *guard {
                probes.push(fabryk::vector::vector_probe(Arc::clone(backend)));
            }
        }
    }

    let health_tools = fabryk_mcp::HealthTools::new(
        &state.config.server.name,
        &state.config.server.version,
        0, // legacy field; CompositeRegistry handles actual count
    )
    .with_backends(probes)
    .with_search_config(fabryk_mcp::SearchConfigInfo {
        query_mode: format!("{:?}", state.config.search.query_mode).to_lowercase(),
        stopwords_enabled: state.config.search.stopwords_enabled,
        fuzzy_search: state.config.search.fuzzy_enabled,
        field_boosts: Some(fabryk_mcp::FieldBoosts {
            title: state.config.search.field_boost_title,
            description: state.config.search.field_boost_description,
            content: state.config.search.field_boost_content,
        }),
    });

    // ---- Graph tools (17 tools) ----
    //
    // The shared_graph field provides an Arc<tokio::sync::RwLock<GraphData>> that
    // starts empty and is populated when the graph finishes loading asynchronously.
    // GraphTools reads from this lock on every tool call, so it transparently
    // picks up the graph data once available.
    #[cfg(feature = "graph")]
    let graph_tools = {
        let node_filter = Arc::new(
            fabryk_mcp::graph::MetadataNodeFilter::new()
                .with_exact("tier", "tier")
                .with_ordered(
                    "min_confidence",
                    "extraction_confidence",
                    &["low", "medium", "high"],
                ),
        );
        let extra = tier_confidence_schema();

        GraphTools::with_shared(Arc::clone(&state.shared_graph))
            .with_node_filter(node_filter)
            .with_names(HashMap::from([
                // Slot key -> exposed tool name
                (GraphTools::SLOT_GET_NODE.into(), "get_node".into()),
                (GraphTools::SLOT_GET_NODE_EDGES.into(), "get_node_edges".into()),
                (GraphTools::SLOT_RELATED.into(), "get_related_concepts".into()),
                (GraphTools::SLOT_PATH.into(), "find_concept_path".into()),
                (GraphTools::SLOT_PREREQUISITES.into(), "get_prerequisites".into()),
                (GraphTools::SLOT_NEIGHBORHOOD.into(), "get_concept_neighborhood".into()),
                (GraphTools::SLOT_DEPENDENTS.into(), "get_dependents".into()),
                (GraphTools::SLOT_CENTRALITY.into(), "get_central_concepts".into()),
                (GraphTools::SLOT_CONCEPT_SOURCES.into(), "get_concept_sources".into()),
                (GraphTools::SLOT_CONCEPT_VARIANTS.into(), "get_concept_variants".into()),
                (GraphTools::SLOT_BRIDGE_CATEGORIES.into(), "find_bridge_concepts".into()),
                (GraphTools::SLOT_SOURCE_COVERAGE.into(), "get_source_coverage".into()),
                (GraphTools::SLOT_LEARNING_PATH.into(), "get_learning_path".into()),
                (GraphTools::SLOT_STATUS.into(), "graph_status".into()),
                (GraphTools::SLOT_INFO.into(), "graph_stats".into()),
                (GraphTools::SLOT_VALIDATE.into(), "graph_validate".into()),
                // graph_bridges keeps its default name (additive 17th tool)
            ]))
            .with_descriptions(HashMap::from([
                (
                    GraphTools::SLOT_STATUS.into(),
                    "Get concept graph status and basic statistics".into(),
                ),
                (
                    GraphTools::SLOT_INFO.into(),
                    "Get detailed concept graph statistics (categories, relationships)".into(),
                ),
                (
                    GraphTools::SLOT_VALIDATE.into(),
                    "Validate concept graph integrity (orphans, self-loops)".into(),
                ),
                (
                    GraphTools::SLOT_GET_NODE.into(),
                    "Get node information by ID with in/out degree counts".into(),
                ),
                (
                    GraphTools::SLOT_GET_NODE_EDGES.into(),
                    "Get all edges for a node with optional direction filter".into(),
                ),
                (
                    GraphTools::SLOT_RELATED.into(),
                    "Find related concepts with optional relationship and direction filtering".into(),
                ),
                (
                    GraphTools::SLOT_PATH.into(),
                    "Find shortest path between two concepts in the graph".into(),
                ),
                (
                    GraphTools::SLOT_PREREQUISITES.into(),
                    "Get prerequisites for a concept in topological learning order".into(),
                ),
                (
                    GraphTools::SLOT_NEIGHBORHOOD.into(),
                    "Get local neighborhood subgraph around a concept".into(),
                ),
                (
                    GraphTools::SLOT_DEPENDENTS.into(),
                    "Get concepts that depend on this concept as a prerequisite".into(),
                ),
                (
                    GraphTools::SLOT_CENTRALITY.into(),
                    "Get most connected concepts by degree centrality".into(),
                ),
                (
                    GraphTools::SLOT_CONCEPT_SOURCES.into(),
                    "Get all sources that introduce or cover a concept".into(),
                ),
                (
                    GraphTools::SLOT_CONCEPT_VARIANTS.into(),
                    "Get all source-specific variants of a canonical concept".into(),
                ),
                (
                    GraphTools::SLOT_BRIDGE_CATEGORIES.into(),
                    "Find concepts that bridge two categories by connecting to both".into(),
                ),
                (
                    GraphTools::SLOT_SOURCE_COVERAGE.into(),
                    "Get all concepts introduced or covered by a source material".into(),
                ),
                (
                    GraphTools::SLOT_LEARNING_PATH.into(),
                    "Get topologically sorted learning path of prerequisites for a target concept, with tier annotations".into(),
                ),
            ]))
            // Add tier/confidence filter schema to tools that support it
            .with_extra_schema(GraphTools::SLOT_RELATED, extra.clone())
            .with_extra_schema(GraphTools::SLOT_PREREQUISITES, extra.clone())
            .with_extra_schema(GraphTools::SLOT_NEIGHBORHOOD, extra.clone())
            .with_extra_schema(GraphTools::SLOT_CENTRALITY, extra.clone())
            .with_extra_schema(GraphTools::SLOT_LEARNING_PATH, extra)
    };

    let skill_docs_path = state
        .config
        .paths
        .skill_docs_path()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));

    #[allow(unused_mut)]
    let mut builder = fabryk_mcp::ServerBuilder::new()
        .name(&state.config.server.name)
        .version(&state.config.server.version)
        .description(
            "Music Theory AI Skill — comprehensive music theory materials and computation.\n\n\
             QUERY STRATEGY:\n\
             1. Start with semantic_search (mode: hybrid) for open-ended questions\n\
             2. Use search_concepts for keyword-specific lookups\n\
             3. Use get_concept to read full content of a result\n\
             4. Use get_related_concepts or get_concept_neighborhood to explore connections\n\
             5. Use get_prerequisites or get_learning_path for learning order\n\
             6. Use find_concept_path to find how two concepts connect\n\n\
             COMPUTATION: get_scale_notes, get_chord_notes, get_interval, transpose, \
             get_diatonic_chords, identify_chord, identify_scale, check_enharmonic, \
             analyze_roman_numerals for standard music theory.\n\n\
             OPEN TONE HARMONY: get_oth_* tools for quintal/quartal chord analysis \
             in the [6,8] metric space.",
        )
        .resources_path(skill_docs_path)
        .with_services(collect_service_handles(&state))
        .add(concept_tools)
        .add(guide_tools)
        .add(source_tools)
        .add(search_tools)
        .add(semantic_search)
        .add(question_search)
        .add(health_tools)
        .add(music_theory_tools)
        .add(oth_tools);

    #[cfg(feature = "graph")]
    {
        let gated_graph =
            fabryk_mcp::ServiceAwareRegistry::new(graph_tools, vec![state.graph.service().clone()]);
        builder = builder.add(gated_graph);
    }

    let builder = builder
        .with_resource(fabryk_mcp::StaticResourceDef {
            uri: "skill://conventions".into(),
            name: "Music Theory Conventions".into(),
            description: "Notation conventions and terminology used in this skill".into(),
            mime_type: "text/markdown".into(),
            filename: "CONVENTIONS.md".into(),
            fallback: Some(crate::resources::default_conventions()),
        })
        .with_resource(fabryk_mcp::StaticResourceDef {
            uri: "skill://scope".into(),
            name: "Skill Scope".into(),
            description: "Topics covered and learning objectives of this skill".into(),
            mime_type: "text/markdown".into(),
            filename: "SCOPE.md".into(),
            fallback: Some(crate::resources::default_scope()),
        })
        .with_resource(fabryk_mcp::StaticResourceDef {
            uri: "skill://sources".into(),
            name: "Source Materials".into(),
            description: "Bibliography and source attribution".into(),
            mime_type: "text/markdown".into(),
            filename: "SOURCES.md".into(),
            fallback: Some(crate::resources::default_sources()),
        })
        .with_resource(fabryk_mcp::StaticResourceDef {
            uri: "skill://index".into(),
            name: "Skill Index".into(),
            description: "Complete index of concepts, topics, and materials".into(),
            mime_type: "text/markdown".into(),
            filename: "INDEX.md".into(),
            fallback: Some(crate::resources::default_index()),
        });

    // Wrap registry in DiscoverableRegistry for tool metadata + directory tool
    let (registry, parts) = builder.into_parts();

    use fabryk_mcp::{DiscoverableRegistry, ToolMeta};

    let discoverable = DiscoverableRegistry::new(registry, "mt")
        // ---- Search tools ----
        .with_tool_meta(
            "semantic_search",
            ToolMeta {
                summary: "Search by meaning using keyword, vector, or hybrid mode".into(),
                when_to_use: "When the user asks a question or searches for a topic".into(),
                returns: "Ranked results with relevance scores and snippets".into(),
                next: Some(
                    "get_concept for full content, get_related_concepts for connections".into(),
                ),
                category: Some("search".into()),
            },
        )
        .with_tool_meta(
            "search_concepts",
            ToolMeta {
                summary: "Full-text keyword search across concept cards and sources".into(),
                when_to_use: "When searching for specific terms or phrases".into(),
                returns: "Ranked results with relevance scores".into(),
                next: Some("get_concept".into()),
                category: Some("search".into()),
            },
        )
        .with_tool_meta(
            "search_by_question",
            ToolMeta {
                summary: "Find concepts by natural language question matching".into(),
                when_to_use: "When the user asks a specific question about music theory".into(),
                returns: "Concepts whose questions/descriptions match the query".into(),
                next: Some("get_concept".into()),
                category: Some("search".into()),
            },
        )
        // ---- Content tools ----
        .with_tool_meta(
            "list_concepts",
            ToolMeta {
                summary: "List concept cards with optional filtering".into(),
                when_to_use: "When browsing or exploring available topics".into(),
                returns: "Array of concept summaries with title, category, tier".into(),
                next: Some("get_concept for details".into()),
                category: Some("content".into()),
            },
        )
        .with_tool_meta(
            "get_concept",
            ToolMeta {
                summary: "Retrieve a specific concept card by ID".into(),
                when_to_use: "After finding a concept via search or graph exploration".into(),
                returns: "Full concept card content with metadata".into(),
                next: Some("get_related_concepts, get_prerequisites".into()),
                category: Some("content".into()),
            },
        )
        .with_tool_meta(
            "list_sources",
            ToolMeta {
                summary: "List all source materials with metadata".into(),
                when_to_use: "When looking for source texts or references".into(),
                returns: "Source list with author, title, availability status".into(),
                next: Some("list_source_chapters, get_source_chapter".into()),
                category: Some("content".into()),
            },
        )
        .with_tool_meta(
            "get_source_chapter",
            ToolMeta {
                summary: "Read a specific chapter from a source text".into(),
                when_to_use: "When reading original source material".into(),
                returns: "Chapter content in markdown".into(),
                next: Some("search_concepts for related concept cards".into()),
                category: Some("content".into()),
            },
        )
        .with_tool_meta(
            "get_guide",
            ToolMeta {
                summary: "Read a topic guide".into(),
                when_to_use: "For in-depth exploration of a topic".into(),
                returns: "Guide content in markdown".into(),
                next: Some("search_concepts, get_related_concepts".into()),
                category: Some("content".into()),
            },
        )
        // ---- Graph tools ----
        .with_tool_meta(
            "get_related_concepts",
            ToolMeta {
                summary: "Find concepts related to a given concept".into(),
                when_to_use: "When exploring connections from a concept".into(),
                returns: "Related concepts with relationship types and directions".into(),
                next: Some("get_concept, get_concept_neighborhood".into()),
                category: Some("graph".into()),
            },
        )
        .with_tool_meta(
            "get_prerequisites",
            ToolMeta {
                summary: "Get prerequisites for a concept in topological order".into(),
                when_to_use: "When understanding what must be learned first".into(),
                returns: "Ordered list of prerequisite concepts".into(),
                next: Some("get_concept, get_learning_path".into()),
                category: Some("graph".into()),
            },
        )
        .with_tool_meta(
            "get_learning_path",
            ToolMeta {
                summary: "Get topologically sorted learning path with tier annotations".into(),
                when_to_use: "When planning a study sequence for a target concept".into(),
                returns: "Ordered learning steps with tier (foundational/intermediate/advanced)"
                    .into(),
                next: Some("get_concept for each step".into()),
                category: Some("graph".into()),
            },
        )
        .with_tool_meta(
            "find_concept_path",
            ToolMeta {
                summary: "Find shortest path between two concepts in the graph".into(),
                when_to_use: "When exploring how two concepts connect".into(),
                returns: "Path with intermediate concepts and relationship types".into(),
                next: Some("get_concept for any node along the path".into()),
                category: Some("graph".into()),
            },
        )
        .with_tool_meta(
            "get_concept_neighborhood",
            ToolMeta {
                summary: "Get local subgraph around a concept".into(),
                when_to_use: "When exploring the immediate context of a concept".into(),
                returns: "Nodes and edges within N hops".into(),
                next: Some("get_concept, get_related_concepts".into()),
                category: Some("graph".into()),
            },
        )
        // ---- Computation tools ----
        .with_tool_meta(
            "get_scale_notes",
            ToolMeta {
                summary: "Compute notes of a musical scale given tonic and mode".into(),
                when_to_use: "When asked about scale notes, modes, or scale construction".into(),
                returns: "List of note names in the scale".into(),
                next: Some("get_diatonic_chords, identify_scale".into()),
                category: Some("computation".into()),
            },
        )
        .with_tool_meta(
            "get_chord_notes",
            ToolMeta {
                summary: "Compute notes of a chord given root and quality".into(),
                when_to_use: "When asked about chord spelling or voicing".into(),
                returns: "List of note names in the chord".into(),
                next: Some("identify_chord, analyze_roman_numerals".into()),
                category: Some("computation".into()),
            },
        )
        .with_tool_meta(
            "get_interval",
            ToolMeta {
                summary: "Calculate the interval between two notes".into(),
                when_to_use: "When asked about the distance between notes".into(),
                returns: "Semitone count, quality, and number".into(),
                next: Some("transpose".into()),
                category: Some("computation".into()),
            },
        )
        .with_tool_meta(
            "analyze_roman_numerals",
            ToolMeta {
                summary: "Analyze chords in a key context with Roman numeral labels".into(),
                when_to_use: "When analyzing a chord progression or harmonic function".into(),
                returns: "Roman numeral analysis for each chord".into(),
                next: Some("get_diatonic_chords".into()),
                category: Some("computation".into()),
            },
        )
        // ---- OTH tools ----
        .with_tool_meta(
            "get_oth_orbit_info",
            ToolMeta {
                summary: "Get complete info about an OTH orbit (modes, scales, structure)".into(),
                when_to_use: "When exploring quintal/quartal chord families".into(),
                returns: "Orbit data: modes, parent scales, fiber class, Forte number".into(),
                next: Some("get_oth_chord_scale, list_oth_modes".into()),
                category: Some("oth".into()),
            },
        )
        .with_tool_meta(
            "get_oth_chord_scale",
            ToolMeta {
                summary: "Compute Tymoczko chord scale and interscalar transposition cycle".into(),
                when_to_use: "When exploring inversions of spread voicings (quintal/quartal)"
                    .into(),
                returns: "Chord scale degrees and full inversion cycle with voice movements".into(),
                next: Some("get_oth_orbit_info".into()),
                category: Some("oth".into()),
            },
        )
        .with_tool_meta(
            "get_oth_distance",
            ToolMeta {
                summary: "Compute geodesic distance between two chords in [6,8] space".into(),
                when_to_use: "When measuring harmonic distance between quintal chords".into(),
                returns: "Distance, L1 distance, orbit membership of both chords".into(),
                next: Some("get_oth_geodesics, get_oth_neighbors".into()),
                category: Some("oth".into()),
            },
        )
        // ---- Meta ----
        .with_tool_meta(
            "health",
            ToolMeta {
                summary: "Server health status and backend availability".into(),
                when_to_use: "When checking if search/graph/vector backends are ready".into(),
                returns: "Service status, tool count, backend health".into(),
                next: None,
                category: Some("meta".into()),
            },
        );

    fabryk_mcp::ServerBuilder::build_with_registry(discoverable, parts)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use fabryk::core::ConfigManager;

    /// Load Config from this crate's `config/default.toml`, regardless of CWD or
    /// host filesystem layout. Keeps tests independent of `path_resolver()`'s
    /// fallbacks (which assume a developer-machine layout that doesn't exist on CI).
    fn load_test_config() -> Config {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/config/default.toml");
        <Config as ConfigManager>::load(Some(path)).unwrap()
    }

    // --- Build server ---

    #[tokio::test]
    async fn test_build_server() {
        let config = load_test_config();
        let state = AppState::new(config.clone()).await.unwrap();
        let server = build_server(state);

        assert_eq!(server.config().name, config.server.name);
        assert_eq!(server.config().version, config.server.version);
        assert!(server
            .config()
            .description
            .as_ref()
            .unwrap()
            .contains("Music Theory AI Skill"));
    }

    #[tokio::test]
    async fn test_build_server_total_tool_count() {
        let config = load_test_config();
        let state = AppState::new(config).await.unwrap();
        let server = build_server(state);

        // Without graph feature:
        //   3 concept + 2 guide + 5 source + 2 fts + 1 semantic + 1 question + 1 health
        //   + 9 music theory + 12 OTH + 1 mt_directory = 37
        // With graph feature:
        //   37 + 17 graph = 54
        #[cfg(feature = "graph")]
        assert_eq!(server.registry().tool_count(), 54);
        #[cfg(not(feature = "graph"))]
        assert_eq!(server.registry().tool_count(), 37);
    }

    #[tokio::test]
    async fn test_build_server_has_all_tools() {
        let config = load_test_config();
        let state = AppState::new(config).await.unwrap();
        let server = build_server(state);
        let registry = server.registry();

        let expected_tools = [
            "list_concepts",
            "get_concept",
            "list_categories",
            "list_guides",
            "get_guide",
            "list_sources",
            "check_source_availability",
            "list_source_chapters",
            "get_source_chapter",
            "get_source_pdf_path",
            "search_concepts",
            "search_status",
            "semantic_search",
            "search_by_question",
            "health",
        ];

        for tool_name in &expected_tools {
            assert!(registry.has_tool(tool_name), "Missing tool: {}", tool_name);
        }
    }

    #[cfg(feature = "graph")]
    #[tokio::test]
    async fn test_build_server_has_graph_tools() {
        let config = load_test_config();
        let state = AppState::new(config).await.unwrap();
        let server = build_server(state);
        let registry = server.registry();

        let expected_graph_tools = [
            "graph_status",
            "graph_stats",
            "graph_validate",
            "get_node",
            "get_node_edges",
            "get_related_concepts",
            "find_concept_path",
            "get_prerequisites",
            "get_concept_neighborhood",
            "get_dependents",
            "get_central_concepts",
            "get_concept_sources",
            "get_concept_variants",
            "find_bridge_concepts",
            "get_source_coverage",
            "get_learning_path",
            "graph_bridges", // fabryk's bridge-node-by-centrality tool
            // music theory computation tools
            "get_scale_notes",
            "get_chord_notes",
            "get_interval",
            "transpose",
            "get_diatonic_chords",
            "identify_chord",
            "identify_scale",
            "check_enharmonic",
            "analyze_roman_numerals",
        ];

        for tool_name in &expected_graph_tools {
            assert!(
                registry.has_tool(tool_name),
                "Missing graph tool: {}",
                tool_name
            );
        }
    }

    // --- Schema validation ---

    #[tokio::test]
    async fn test_all_tools_have_valid_schemas() {
        let config = load_test_config();
        let state = AppState::new(config).await.unwrap();
        let server = build_server(state);
        fabryk_mcp::assert_tools_valid(server.registry());
    }

    #[tokio::test]
    async fn test_server_description_contains_query_strategy() {
        let config = load_test_config();
        let state = AppState::new(config).await.unwrap();
        let server = build_server(state);
        let desc = server.config().description.as_ref().unwrap();
        assert!(desc.contains("QUERY STRATEGY"));
        assert!(desc.contains("semantic_search"));
        assert!(desc.contains("COMPUTATION"));
        assert!(desc.contains("OPEN TONE HARMONY"));
    }
}
