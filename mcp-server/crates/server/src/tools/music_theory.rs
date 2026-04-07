//! Music theory computational tools powered by `music-comp-mt`.
//!
//! These tools provide pure computation — no file I/O or state required.
//! They accept note names as strings (e.g., "C#", "Eb", "F") and return
//! structured music theory results.

use serde::{Deserialize, Serialize};

use music_comp_mt::analysis;
use music_comp_mt::chord::Chord;
use music_comp_mt::harmony;
use music_comp_mt::interval::Interval;
use music_comp_mt::note::{Notes, Pitch};
use music_comp_mt::scale::{Direction, Mode, Scale};

use crate::error::{Error, Result};

// ============================================================================
// Pitch parsing helper
// ============================================================================

fn parse_pitch(s: &str) -> Result<Pitch> {
    Pitch::try_parse(s).ok_or_else(|| Error::parse(format!("Invalid pitch: '{}'", s)))
}

fn parse_mode(s: &str) -> Result<Mode> {
    let (mode, _) =
        Mode::from_regex(s).map_err(|e| Error::parse(format!("Invalid mode '{}': {}", s, e)))?;
    Ok(mode)
}

fn note_to_string(note: &music_comp_mt::note::Note) -> String {
    format!("{}", note.pitch)
}

// ============================================================================
// Tool 1: get_scale_notes
// ============================================================================

#[derive(Deserialize)]
pub struct GetScaleNotesArgs {
    pub tonic: String,
    pub mode: String,
    #[serde(default)]
    pub direction: Option<String>,
}

#[derive(Serialize)]
pub struct GetScaleNotesResponse {
    pub notes: Vec<String>,
    pub scale_type: String,
    pub mode: String,
}

pub fn get_scale_notes(args: GetScaleNotesArgs) -> Result<GetScaleNotesResponse> {
    let direction = match args.direction.as_deref() {
        Some("descending") | Some("down") => Direction::Descending,
        _ => Direction::Ascending,
    };
    let input = format!("{} {}", args.tonic, args.mode);
    let scale = Scale::from_regex_in_direction(&input, direction)
        .map_err(|e| Error::parse(format!("Invalid scale '{}': {}", input, e)))?;
    let notes: Vec<String> = scale.notes().iter().map(note_to_string).collect();
    Ok(GetScaleNotesResponse {
        notes,
        scale_type: format!("{:?}", scale.scale_type),
        mode: scale.mode.map(|m| format!("{:?}", m)).unwrap_or_default(),
    })
}

// ============================================================================
// Tool 2: get_chord_notes
// ============================================================================

#[derive(Deserialize)]
pub struct GetChordNotesArgs {
    pub root: String,
    pub quality: String,
    #[serde(default)]
    pub number: Option<String>,
    #[serde(default)]
    pub inversion: Option<u8>,
}

#[derive(Serialize)]
pub struct GetChordNotesResponse {
    pub notes: Vec<String>,
    pub quality: String,
    pub number: String,
}

pub fn get_chord_notes(args: GetChordNotesArgs) -> Result<GetChordNotesResponse> {
    let input = match &args.number {
        Some(num) => format!("{} {} {}", args.root, args.quality, num),
        None => format!("{} {}", args.root, args.quality),
    };
    let mut chord = Chord::from_regex(&input)
        .map_err(|e| Error::parse(format!("Invalid chord '{}': {}", input, e)))?;
    if let Some(inv) = args.inversion {
        chord.inversion = inv;
    }
    let notes: Vec<String> = chord.notes().iter().map(note_to_string).collect();
    Ok(GetChordNotesResponse {
        notes,
        quality: format!("{:?}", chord.quality),
        number: format!("{:?}", chord.number),
    })
}

// ============================================================================
// Tool 3: get_interval
// ============================================================================

#[derive(Deserialize)]
pub struct GetIntervalArgs {
    pub from: String,
    pub to: String,
}

#[derive(Serialize)]
pub struct GetIntervalResponse {
    pub semitones: u8,
    pub quality: String,
    pub number: String,
}

pub fn get_interval(args: GetIntervalArgs) -> Result<GetIntervalResponse> {
    let from = parse_pitch(&args.from)?;
    let to = parse_pitch(&args.to)?;
    let interval = Interval::between(&from, &to)
        .map_err(|e| Error::parse(format!("Cannot compute interval: {}", e)))?;
    Ok(GetIntervalResponse {
        semitones: interval.semitone_count,
        quality: format!("{}", interval.quality),
        number: format!("{}", interval.number),
    })
}

// ============================================================================
// Tool 4: transpose
// ============================================================================

#[derive(Deserialize)]
pub struct TransposeArgs {
    pub notes: Vec<String>,
    pub semitones: u8,
    #[serde(default = "default_direction_up")]
    pub direction: String,
}

fn default_direction_up() -> String {
    "up".to_string()
}

#[derive(Serialize)]
pub struct TransposeResponse {
    pub original: Vec<String>,
    pub transposed: Vec<String>,
}

pub fn transpose(args: TransposeArgs) -> Result<TransposeResponse> {
    let interval = Interval::from_semitone(args.semitones)
        .map_err(|e| Error::parse(format!("Invalid semitone count: {}", e)))?;
    let original: Vec<String> = args.notes.clone();
    let transposed: Vec<String> = args
        .notes
        .iter()
        .map(|s| {
            let pitch = parse_pitch(s)?;
            let new_pitch = if args.direction == "down" {
                pitch.transpose_down(&interval)
            } else {
                pitch.transpose_up(&interval)
            };
            Ok(format!("{}", new_pitch))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(TransposeResponse {
        original,
        transposed,
    })
}

// ============================================================================
// Tool 5: get_diatonic_chords
// ============================================================================

#[derive(Deserialize)]
pub struct GetDiatonicChordsArgs {
    pub tonic: String,
    pub mode: String,
    #[serde(default)]
    pub chord_type: Option<String>,
}

#[derive(Serialize)]
pub struct DiatonicChordInfo {
    pub degree: u8,
    pub root: String,
    pub quality: String,
    pub number: String,
    pub notes: Vec<String>,
}

#[derive(Serialize)]
pub struct GetDiatonicChordsResponse {
    pub chords: Vec<DiatonicChordInfo>,
}

pub fn get_diatonic_chords(args: GetDiatonicChordsArgs) -> Result<GetDiatonicChordsResponse> {
    let tonic = parse_pitch(&args.tonic)?;
    let mode = parse_mode(&args.mode)?;
    let use_sevenths = args.chord_type.as_deref() == Some("seventh");
    let diatonic = if use_sevenths {
        harmony::diatonic_sevenths(tonic, mode)
    } else {
        harmony::diatonic_triads(tonic, mode)
    };
    let chords = diatonic
        .into_iter()
        .map(|dc| {
            let notes = dc.chord.notes().iter().map(note_to_string).collect();
            DiatonicChordInfo {
                degree: dc.degree,
                root: format!("{}", dc.root),
                quality: format!("{:?}", dc.quality),
                number: format!("{:?}", dc.number),
                notes,
            }
        })
        .collect();
    Ok(GetDiatonicChordsResponse { chords })
}

// ============================================================================
// Tool 6: identify_chord
// ============================================================================

#[derive(Deserialize)]
pub struct IdentifyChordArgs {
    pub notes: Vec<String>,
}

#[derive(Serialize)]
pub struct ChordMatch {
    pub root: String,
    pub quality: String,
    pub number: String,
    pub inversion: u8,
}

#[derive(Serialize)]
pub struct IdentifyChordResponse {
    pub matches: Vec<ChordMatch>,
}

pub fn identify_chord(args: IdentifyChordArgs) -> Result<IdentifyChordResponse> {
    let pitches: Vec<Pitch> = args
        .notes
        .iter()
        .map(|s| parse_pitch(s))
        .collect::<Result<Vec<_>>>()?;
    let chords = Chord::identify(&pitches);
    let matches = chords
        .into_iter()
        .map(|c| ChordMatch {
            root: format!("{}", c.root),
            quality: format!("{:?}", c.quality),
            number: format!("{:?}", c.number),
            inversion: c.inversion,
        })
        .collect();
    Ok(IdentifyChordResponse { matches })
}

// ============================================================================
// Tool 7: identify_scale
// ============================================================================

#[derive(Deserialize)]
pub struct IdentifyScaleArgs {
    pub notes: Vec<String>,
}

#[derive(Serialize)]
pub struct ScaleMatch {
    pub tonic: String,
    pub mode: String,
}

#[derive(Serialize)]
pub struct IdentifyScaleResponse {
    pub matches: Vec<ScaleMatch>,
}

pub fn identify_scale(args: IdentifyScaleArgs) -> Result<IdentifyScaleResponse> {
    let pitches: Vec<Pitch> = args
        .notes
        .iter()
        .map(|s| parse_pitch(s))
        .collect::<Result<Vec<_>>>()?;
    let scales = Scale::identify(&pitches);
    let matches = scales
        .into_iter()
        .map(|(tonic, mode)| ScaleMatch {
            tonic: format!("{}", tonic),
            mode: format!("{:?}", mode),
        })
        .collect();
    Ok(IdentifyScaleResponse { matches })
}

// ============================================================================
// Tool 8: check_enharmonic
// ============================================================================

#[derive(Deserialize)]
pub struct CheckEnharmonicArgs {
    pub note_a: String,
    pub note_b: String,
}

#[derive(Serialize)]
pub struct CheckEnharmonicResponse {
    pub equivalent: bool,
    pub semitone_value_a: u8,
    pub semitone_value_b: u8,
}

pub fn check_enharmonic(args: CheckEnharmonicArgs) -> Result<CheckEnharmonicResponse> {
    let a = parse_pitch(&args.note_a)?;
    let b = parse_pitch(&args.note_b)?;
    Ok(CheckEnharmonicResponse {
        equivalent: a.is_enharmonic_to(&b),
        semitone_value_a: a.as_u8(),
        semitone_value_b: b.as_u8(),
    })
}

// ============================================================================
// Tool 9: analyze_roman_numerals
// ============================================================================

#[derive(Deserialize)]
pub struct AnalyzeRomanNumeralsArgs {
    pub key_tonic: String,
    pub key_mode: String,
    pub chords: Vec<String>,
}

#[derive(Serialize)]
pub struct RomanNumeralInfo {
    pub chord: String,
    pub roman: Option<String>,
    pub degree: Option<u8>,
}

#[derive(Serialize)]
pub struct AnalyzeRomanNumeralsResponse {
    pub analysis: Vec<RomanNumeralInfo>,
}

pub fn analyze_roman_numerals(
    args: AnalyzeRomanNumeralsArgs,
) -> Result<AnalyzeRomanNumeralsResponse> {
    let key_tonic = parse_pitch(&args.key_tonic)?;
    let key_mode = parse_mode(&args.key_mode)?;
    let analysis = args
        .chords
        .iter()
        .map(|chord_str| {
            let chord = Chord::from_regex(chord_str).ok();
            let rn = chord
                .as_ref()
                .and_then(|c| analysis::roman_numeral(key_tonic, key_mode, c));
            RomanNumeralInfo {
                chord: chord_str.clone(),
                roman: rn.as_ref().map(|r| r.label.clone()),
                degree: rn.as_ref().map(|r| r.degree),
            }
        })
        .collect();
    Ok(AnalyzeRomanNumeralsResponse { analysis })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_scale_notes_c_major() {
        let args = GetScaleNotesArgs {
            tonic: "C".to_string(),
            mode: "Major".to_string(),
            direction: None,
        };
        let response = get_scale_notes(args).unwrap();
        assert_eq!(response.notes.len(), 8); // 7 notes + octave
        assert_eq!(response.notes[0], "C");
        assert_eq!(response.notes[7], "C");
    }

    #[test]
    fn test_get_chord_notes_c_major() {
        let args = GetChordNotesArgs {
            root: "C".to_string(),
            quality: "Major".to_string(),
            number: None,
            inversion: None,
        };
        let response = get_chord_notes(args).unwrap();
        assert_eq!(response.notes.len(), 3);
        assert_eq!(response.notes[0], "C");
        assert_eq!(response.notes[1], "E");
        assert_eq!(response.notes[2], "G");
    }

    #[test]
    fn test_get_interval_c_to_g() {
        let args = GetIntervalArgs {
            from: "C".to_string(),
            to: "G".to_string(),
        };
        let response = get_interval(args).unwrap();
        assert_eq!(response.semitones, 7);
        assert_eq!(response.number, "5");
        assert_eq!(response.quality, "P");
    }

    #[test]
    fn test_transpose_up() {
        let args = TransposeArgs {
            notes: vec!["C".to_string(), "E".to_string(), "G".to_string()],
            semitones: 2,
            direction: "up".to_string(),
        };
        let response = transpose(args).unwrap();
        assert_eq!(response.transposed[0], "D");
    }

    #[test]
    fn test_get_diatonic_chords_c_major() {
        let args = GetDiatonicChordsArgs {
            tonic: "C".to_string(),
            mode: "Ionian".to_string(),
            chord_type: None,
        };
        let response = get_diatonic_chords(args).unwrap();
        assert_eq!(response.chords.len(), 7);
        assert_eq!(response.chords[0].degree, 1);
        assert_eq!(response.chords[0].root, "C");
    }

    #[test]
    fn test_identify_chord() {
        let args = IdentifyChordArgs {
            notes: vec!["C".to_string(), "E".to_string(), "G".to_string()],
        };
        let response = identify_chord(args).unwrap();
        assert!(!response.matches.is_empty());
        assert_eq!(response.matches[0].root, "C");
    }

    #[test]
    fn test_identify_scale() {
        let args = IdentifyScaleArgs {
            notes: vec![
                "C".to_string(),
                "D".to_string(),
                "E".to_string(),
                "F".to_string(),
                "G".to_string(),
                "A".to_string(),
                "B".to_string(),
            ],
        };
        let response = identify_scale(args).unwrap();
        assert!(!response.matches.is_empty());
    }

    #[test]
    fn test_check_enharmonic() {
        let args = CheckEnharmonicArgs {
            note_a: "C#".to_string(),
            note_b: "Db".to_string(),
        };
        let response = check_enharmonic(args).unwrap();
        assert!(response.equivalent);
        assert_eq!(response.semitone_value_a, response.semitone_value_b);
    }

    #[test]
    fn test_check_enharmonic_not_equivalent() {
        let args = CheckEnharmonicArgs {
            note_a: "C".to_string(),
            note_b: "D".to_string(),
        };
        let response = check_enharmonic(args).unwrap();
        assert!(!response.equivalent);
    }

    #[test]
    fn test_analyze_roman_numerals() {
        let args = AnalyzeRomanNumeralsArgs {
            key_tonic: "C".to_string(),
            key_mode: "Major".to_string(),
            chords: vec![
                "C Major".to_string(),
                "G Major".to_string(),
                "A Minor".to_string(),
            ],
        };
        let response = analyze_roman_numerals(args).unwrap();
        assert_eq!(response.analysis.len(), 3);
        assert_eq!(response.analysis[0].roman.as_deref(), Some("I"));
        assert_eq!(response.analysis[1].roman.as_deref(), Some("V"));
        assert_eq!(response.analysis[2].roman.as_deref(), Some("vi"));
    }

    #[test]
    fn test_parse_pitch_invalid() {
        assert!(parse_pitch("X#").is_err());
    }

    #[test]
    fn test_get_chord_notes_with_number() {
        let args = GetChordNotesArgs {
            root: "C".to_string(),
            quality: "Major".to_string(),
            number: Some("Seventh".to_string()),
            inversion: None,
        };
        let response = get_chord_notes(args).unwrap();
        assert_eq!(response.notes.len(), 4);
    }

    #[test]
    fn test_get_scale_notes_descending() {
        let args = GetScaleNotesArgs {
            tonic: "C".to_string(),
            mode: "Major".to_string(),
            direction: Some("descending".to_string()),
        };
        let response = get_scale_notes(args).unwrap();
        assert!(!response.notes.is_empty());
    }
}
