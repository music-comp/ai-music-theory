//! Open Tone Harmony (OTH) tools powered by `music-comp-mt::quintal`.
//!
//! These tools expose the [6,8] metric space B — 228 quintal chords across
//! 14 T/I orbits — via the MCP protocol. They provide orbit analysis, mode
//! enumeration, parent-scale lookups, chord identification, and fiber-bundle
//! (chord scale / inversion cycle) computations.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use music_comp_mt::note::Pitch;
use music_comp_mt::quartal::{QuartalIntervalStructure, QuartalOrbit};
use music_comp_mt::quintal::{
    all_modes, betweenness_centrality, chord_scale, classify_orbit, crossroads_chords, diameter,
    distance, fiber_class, inversion_cycle, l1_distance, modes_by_opening_interval,
    orbit_modes, parent_scales, step_vocabulary_cluster,
    verify_all_orbits_self_dual, verify_universal_l1_law, BaseSpace, FiberClass,
    IntervalStructure, Orbit, OthMode, ParentScale, PcChord,
    StepVocabularyCluster, VoicedChord, transpose,
};
use music_comp_mt::set_class::PitchClassSet;

use crate::error::{Error, Result};

// ============================================================================
// Module-level caches
// ============================================================================

static SPACE: OnceLock<BaseSpace> = OnceLock::new();
static DIAMETER_CACHE: OnceLock<u8> = OnceLock::new();
static CENTRALITY_CACHE: OnceLock<std::collections::HashMap<PcChord, f64>> = OnceLock::new();

fn get_space() -> &'static BaseSpace {
    SPACE.get_or_init(BaseSpace::new)
}

fn get_diameter() -> u8 {
    *DIAMETER_CACHE.get_or_init(|| diameter(get_space()))
}

fn get_centrality() -> &'static std::collections::HashMap<PcChord, f64> {
    CENTRALITY_CACHE.get_or_init(|| betweenness_centrality(get_space()))
}

// ============================================================================
// Helper functions
// ============================================================================

const NOTE_NAMES: [&str; 12] = [
    "C", "Db", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
];

fn pc_to_note_name(pc: u8) -> &'static str {
    NOTE_NAMES[(pc % 12) as usize]
}

fn midi_to_note_name(midi: u8) -> String {
    let octave = (midi / 12) as i8 - 1;
    format!("{}{}", pc_to_note_name(midi % 12), octave)
}

fn interval_name(semitones: u8) -> &'static str {
    match semitones {
        1 => "minor second",
        2 => "major second",
        3 => "minor third",
        4 => "major third",
        5 => "perfect fourth",
        6 => "tritone",
        7 => "perfect fifth",
        8 => "minor sixth",
        _ => "unknown",
    }
}

fn format_fiber_class(fc: FiberClass) -> &'static str {
    match fc {
        FiberClass::ClassA => "A",
        FiberClass::ClassB => "B",
    }
}

fn orbit_to_label(orbit: &Orbit) -> String {
    format!("{:?}", orbit)
}

fn parse_orbit_identifier(s: &str) -> Result<Orbit> {
    let s = s.trim();

    // Try quintal label: "Q777", "q777"
    let upper = s.to_uppercase();
    for orbit in Orbit::all() {
        if upper == format!("{:?}", orbit).to_uppercase() {
            return Ok(*orbit);
        }
    }

    // Try quartal label: "Q555", "q555"
    for qorbit in QuartalOrbit::all() {
        if upper == format!("{:?}", qorbit).to_uppercase() {
            return Ok(qorbit.to_quintal());
        }
    }

    // Try interval structure: "7,7,7" or "[7,7,7]"
    let cleaned = s
        .trim_start_matches('[')
        .trim_end_matches(']')
        .replace(' ', "");
    let parts: Vec<&str> = cleaned.split(',').collect();
    if parts.len() == 3 {
        if let (Ok(a), Ok(b), Ok(c)) = (
            parts[0].parse::<u8>(),
            parts[1].parse::<u8>(),
            parts[2].parse::<u8>(),
        ) {
            // Try quintal intervals ({6,7,8} range)
            let target = IntervalStructure::new(a, b, c);
            for orbit in Orbit::all() {
                if orbit.representative() == target {
                    return Ok(*orbit);
                }
            }
            // Try quartal intervals ({4,5,6} range)
            for qorbit in QuartalOrbit::all() {
                if qorbit.representative() == QuartalIntervalStructure::new(a, b, c) {
                    return Ok(qorbit.to_quintal());
                }
            }
        }
    }

    let quintal_labels: Vec<String> = Orbit::all().iter().map(|o| format!("{:?}", o)).collect();
    let quartal_labels: Vec<String> = QuartalOrbit::all()
        .iter()
        .map(|o| format!("{:?}", o))
        .collect();
    Err(Error::parse(format!(
        "Unrecognized orbit identifier: '{}'. Accepted formats:\n  \
         Quintal labels: {}\n  \
         Quartal labels: {}\n  \
         Interval structure: e.g. '7,7,7' or '[7,7,7]'",
        s,
        quintal_labels.join(", "),
        quartal_labels.join(", "),
    )))
}

fn parse_chord_pcs(pcs: Vec<u8>) -> Result<PcChord> {
    if pcs.len() != 4 {
        return Err(Error::parse(format!(
            "Expected exactly 4 pitch classes, got {}",
            pcs.len()
        )));
    }
    let arr: [u8; 4] = [pcs[0], pcs[1], pcs[2], pcs[3]];
    PcChord::new(arr).map_err(|e| Error::parse(format!("Invalid pitch classes: {}", e)))
}

fn parse_chord_input(
    pcs: Option<Vec<u8>>,
    notes: Option<Vec<String>>,
) -> Result<PcChord> {
    match (pcs, notes) {
        (Some(pcs), _) => parse_chord_pcs(pcs),
        (None, Some(notes)) => {
            if notes.len() != 4 {
                return Err(Error::parse(format!(
                    "Expected exactly 4 notes, got {}",
                    notes.len()
                )));
            }
            let pcs: Vec<u8> = notes
                .iter()
                .map(|n| {
                    Pitch::try_parse(n)
                        .ok_or_else(|| Error::parse(format!("Invalid note: '{}'", n)))
                        .map(|p| p.as_u8())
                })
                .collect::<Result<Vec<_>>>()?;
            parse_chord_pcs(pcs)
        }
        (None, None) => Err(Error::parse(
            "Either 'pcs' or 'notes' must be provided".to_string(),
        )),
    }
}

fn parse_cluster(s: &str) -> Result<StepVocabularyCluster> {
    match s.to_lowercase().replace(' ', "").as_str() {
        "nosemitonenotritone" => Ok(StepVocabularyCluster::NoSemitoneNoTritone),
        "containssemitone" => Ok(StepVocabularyCluster::ContainsSemitone),
        "evenstepsonly" => Ok(StepVocabularyCluster::EvenStepsOnly),
        "containstritonestep" => Ok(StepVocabularyCluster::ContainsTritoneStep),
        _ => Err(Error::parse(format!(
            "Unknown cluster '{}'. Valid values: NoSemitoneNoTritone, \
             ContainsSemitone, EvenStepsOnly, ContainsTritoneStep",
            s
        ))),
    }
}

// ============================================================================
// Shared response structs
// ============================================================================

#[derive(Serialize, Clone)]
pub struct OrbitObject {
    pub quintal_label: String,
    pub quartal_label: String,
    pub interval_structure: [u8; 3],
    pub forte: Option<String>,
    pub prime_form: Vec<u8>,
    pub degree: usize,
    pub orbit_size: usize,
    pub fiber_class: String,
    pub step_cluster: String,
}

#[derive(Serialize, Clone)]
pub struct ModeInfo {
    pub rotation: u8,
    pub steps: Vec<u8>,
    pub pcs_from_c: Vec<u8>,
    pub spelled_from_c: Vec<String>,
    pub opening_interval: u8,
    pub opening_interval_name: String,
}

#[derive(Serialize, Clone)]
pub struct ParentScaleInfo {
    pub scale_type: String,
    pub root_pc: u8,
    pub root_name: String,
    pub pcs: Vec<u8>,
    pub cardinality: u8,
    pub coverage: f32,
    pub coverage_ratio: [u8; 2],
}

#[derive(Serialize, Clone)]
pub struct ChordSummary {
    pub pcs: Vec<u8>,
    pub notes: Vec<String>,
    pub orbit: String,
}

struct VoiceMovement {
    voice: u8,
    from_pc: u8,
    to_pc: u8,
    direction: String,
}

// ============================================================================
// Shared builders
// ============================================================================

fn build_orbit_object(orbit: &Orbit) -> OrbitObject {
    let is = orbit.representative();
    let quartal = QuartalOrbit::from_quintal(orbit);
    let om = orbit_modes(orbit);
    let repr_pcs = om.modes().first().map(|m| m.pcs_from_c()).unwrap_or([0; 4]);
    let pcs_set = PitchClassSet::new(&repr_pcs);
    let fc = fiber_class(&PcChord::new(repr_pcs).unwrap_or_else(|_| {
        // Fallback: use sorted form
        let mut sorted = repr_pcs;
        sorted.sort();
        PcChord::new(sorted).expect("representative PCs should be valid")
    }));

    OrbitObject {
        quintal_label: orbit_to_label(orbit),
        quartal_label: format!("{:?}", quartal),
        interval_structure: is.intervals(),
        forte: om.forte_number(),
        prime_form: pcs_set.prime_form(),
        degree: orbit.degree(),
        orbit_size: orbit.size(),
        fiber_class: fc.map(format_fiber_class).unwrap_or("unknown").to_string(),
        step_cluster: format!("{:?}", step_vocabulary_cluster(orbit)),
    }
}

fn build_mode_info(mode: &OthMode) -> ModeInfo {
    let pcs = mode.pcs_from_c();
    ModeInfo {
        rotation: mode.rotation(),
        steps: mode.steps().to_vec(),
        pcs_from_c: pcs.to_vec(),
        spelled_from_c: pcs.iter().map(|&pc| pc_to_note_name(pc).to_string()).collect(),
        opening_interval: mode.opening_interval(),
        opening_interval_name: interval_name(mode.opening_interval()).to_string(),
    }
}

fn build_parent_scale_info(ps: &ParentScale) -> ParentScaleInfo {
    let (num, den) = ps.coverage_ratio();
    ParentScaleInfo {
        scale_type: format!("{}", ps.scale_type()),
        root_pc: ps.root(),
        root_name: pc_to_note_name(ps.root()).to_string(),
        pcs: ps.pcs().to_vec(),
        cardinality: ps.cardinality(),
        coverage: ps.coverage(),
        coverage_ratio: [num, den],
    }
}

fn build_chord_summary(chord: &PcChord) -> ChordSummary {
    let orbit = classify_orbit(chord)
        .map(|o| orbit_to_label(&o))
        .unwrap_or_else(|| "none".to_string());
    ChordSummary {
        pcs: chord.pcs().to_vec(),
        notes: chord.pcs().iter().map(|&pc| pc_to_note_name(pc).to_string()).collect(),
        orbit,
    }
}

fn compute_voice_movement(from: &PcChord, to: &PcChord) -> Result<VoiceMovement> {
    let from_pcs = from.pcs();
    let to_pcs = to.pcs();

    let mut old_pc = None;
    let mut new_pc = None;

    for &pc in &from_pcs {
        if !to_pcs.contains(&pc) {
            old_pc = Some(pc);
        }
    }
    for &pc in &to_pcs {
        if !from_pcs.contains(&pc) {
            new_pc = Some(pc);
        }
    }

    let old = old_pc.ok_or_else(|| Error::parse("Chords are identical".to_string()))?;
    let new = new_pc.ok_or_else(|| Error::parse("Chords are identical".to_string()))?;
    let voice = from_pcs.iter().position(|&p| p == old).unwrap() as u8;
    let direction = if (old + 1) % 12 == new {
        "up"
    } else {
        "down"
    };

    Ok(VoiceMovement {
        voice,
        from_pc: old,
        to_pc: new,
        direction: direction.to_string(),
    })
}

// ============================================================================
// Full orbit info builder (shared by get_oth_orbit_info and list_oth_orbits)
// ============================================================================

fn build_full_orbit_info(orbit: &Orbit) -> FullOrbitInfo {
    let om = orbit_modes(orbit);
    let repr_pcs = om.modes().first().map(|m| m.pcs_from_c()).unwrap_or([0; 4]);

    let parent_scale_list = parent_scales(&repr_pcs);

    FullOrbitInfo {
        orbit: build_orbit_object(orbit),
        step_sequence: om.modes().first().map(|m| m.steps().to_vec()).unwrap_or_default(),
        step_size_multiset: om.step_size_multiset().to_vec(),
        modes: om.modes().iter().map(build_mode_info).collect(),
        distinct_mode_count: om.distinct_count(),
        parent_scales: parent_scale_list.iter().map(build_parent_scale_info).collect(),
    }
}

// ============================================================================
// Tier 1 Tool 1: get_oth_orbit_info
// ============================================================================

#[derive(Deserialize)]
pub struct GetOthOrbitInfoArgs {
    pub orbit: String,
}

#[derive(Serialize)]
pub struct FullOrbitInfo {
    pub orbit: OrbitObject,
    pub step_sequence: Vec<u8>,
    pub step_size_multiset: Vec<u8>,
    pub modes: Vec<ModeInfo>,
    pub distinct_mode_count: u8,
    pub parent_scales: Vec<ParentScaleInfo>,
}

pub fn get_oth_orbit_info(args: GetOthOrbitInfoArgs) -> Result<FullOrbitInfo> {
    let orbit = parse_orbit_identifier(&args.orbit)?;
    Ok(build_full_orbit_info(&orbit))
}

// ============================================================================
// Tier 1 Tool 2: list_oth_orbits
// ============================================================================

#[derive(Deserialize)]
pub struct ListOthOrbitsArgs {
    #[serde(default)]
    pub cluster: Option<String>,
}

#[derive(Serialize)]
pub struct ListOthOrbitsResponse {
    pub total_orbits: usize,
    pub filter_applied: Option<String>,
    pub orbits: Vec<FullOrbitInfo>,
}

pub fn list_oth_orbits(args: ListOthOrbitsArgs) -> Result<ListOthOrbitsResponse> {
    let orbits: Vec<Orbit> = if let Some(ref cluster_str) = args.cluster {
        let cluster = parse_cluster(cluster_str)?;
        Orbit::all()
            .iter()
            .filter(|o| step_vocabulary_cluster(o) == cluster)
            .copied()
            .collect()
    } else {
        Orbit::all().to_vec()
    };

    let orbit_infos: Vec<FullOrbitInfo> = orbits.iter().map(build_full_orbit_info).collect();

    Ok(ListOthOrbitsResponse {
        total_orbits: orbit_infos.len(),
        filter_applied: args.cluster,
        orbits: orbit_infos,
    })
}

// ============================================================================
// Tier 1 Tool 3: get_oth_parent_scales
// ============================================================================

#[derive(Deserialize)]
pub struct GetOthParentScalesArgs {
    #[serde(default)]
    pub orbit: Option<String>,
    #[serde(default)]
    pub pcs: Option<Vec<u8>>,
}

#[derive(Serialize)]
pub struct GetOthParentScalesResponse {
    pub query_pcs: Vec<u8>,
    pub query_orbit: Option<String>,
    pub parent_scales: Vec<ParentScaleInfo>,
    pub total_matches: usize,
    pub scale_types_matched: Vec<String>,
}

pub fn get_oth_parent_scales(args: GetOthParentScalesArgs) -> Result<GetOthParentScalesResponse> {
    let (pcs_arr, orbit_label) = if let Some(ref orbit_str) = args.orbit {
        let orbit = parse_orbit_identifier(orbit_str)?;
        let om = orbit_modes(&orbit);
        let repr = om.modes().first().map(|m| m.pcs_from_c()).unwrap_or([0; 4]);
        (repr, Some(orbit_to_label(&orbit)))
    } else if let Some(pcs) = args.pcs {
        let chord = parse_chord_pcs(pcs)?;
        let label = classify_orbit(&chord).map(|o| orbit_to_label(&o));
        (chord.pcs(), label)
    } else {
        return Err(Error::parse(
            "Either 'orbit' or 'pcs' must be provided".to_string(),
        ));
    };

    let scales = parent_scales(&pcs_arr);
    let scale_infos: Vec<ParentScaleInfo> = scales.iter().map(build_parent_scale_info).collect();
    let mut types: Vec<String> = scale_infos.iter().map(|s| s.scale_type.clone()).collect();
    types.sort();
    types.dedup();
    let total = scale_infos.len();

    Ok(GetOthParentScalesResponse {
        query_pcs: pcs_arr.to_vec(),
        query_orbit: orbit_label,
        parent_scales: scale_infos,
        total_matches: total,
        scale_types_matched: types,
    })
}

// ============================================================================
// Tier 1 Tool 4: get_oth_chord_info
// ============================================================================

#[derive(Deserialize)]
pub struct GetOthChordInfoArgs {
    #[serde(default)]
    pub pcs: Option<Vec<u8>>,
    #[serde(default)]
    pub notes: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct GetOthChordInfoResponse {
    pub is_in_space: bool,
    pub chord_pcs: Vec<u8>,
    pub chord_notes: Vec<String>,
    pub interval_structure: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orbit: Option<FullOrbitInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

pub fn get_oth_chord_info(args: GetOthChordInfoArgs) -> Result<GetOthChordInfoResponse> {
    let chord = parse_chord_input(args.pcs, args.notes)?;
    let pcs = chord.pcs();
    let notes: Vec<String> = pcs.iter().map(|&pc| pc_to_note_name(pc).to_string()).collect();
    // interval_structure() returns Option — None for degenerate chords
    let intervals = chord
        .interval_structure()
        .map(|is| is.intervals().to_vec())
        .unwrap_or_default();

    match classify_orbit(&chord) {
        Some(orbit) => Ok(GetOthChordInfoResponse {
            is_in_space: true,
            chord_pcs: pcs.to_vec(),
            chord_notes: notes,
            interval_structure: intervals,
            orbit: Some(build_full_orbit_info(&orbit)),
            reason: None,
        }),
        None => {
            let reason = if let Some(is) = chord.interval_structure() {
                let ivals = is.intervals();
                let outside: Vec<u8> =
                    ivals.iter().filter(|&&i| !(6..=8).contains(&i)).copied().collect();
                format!(
                    "Interval structure {:?} contains intervals outside {{6,7,8}}: {:?}",
                    ivals, outside
                )
            } else {
                "Could not compute interval structure".to_string()
            };
            Ok(GetOthChordInfoResponse {
                is_in_space: false,
                chord_pcs: pcs.to_vec(),
                chord_notes: notes,
                interval_structure: intervals,
                orbit: None,
                reason: Some(reason),
            })
        }
    }
}

// ============================================================================
// Tier 1 Tool 5: get_oth_chord_scale
// ============================================================================

#[derive(Deserialize)]
pub struct GetOthChordScaleArgs {
    pub pitches: Vec<u8>,
}

#[derive(Serialize)]
pub struct VoicedChordInfo {
    pub pitches: Vec<u8>,
    pub notes: Vec<String>,
    pub interval_structure: [u8; 3],
    pub span: u8,
}

#[derive(Serialize)]
pub struct ChordScaleInfo {
    pub pcs: Vec<u8>,
    pub notes: Vec<String>,
    pub steps: Vec<u8>,
}

#[derive(Serialize)]
pub struct InversionInfo {
    pub inversion: u8,
    pub label: String,
    pub pitches: Vec<u8>,
    pub notes: Vec<String>,
    pub intervals: [u8; 3],
    pub span: u8,
    pub in_6_8: bool,
    pub l1_from_previous: Option<u32>,
}

#[derive(Serialize)]
pub struct GetOthChordScaleResponse {
    pub voiced_chord: VoicedChordInfo,
    pub chord_scale: ChordScaleInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orbit: Option<OrbitObject>,
    pub inversion_cycle: Vec<InversionInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiber_class: Option<String>,
    pub inversions_in_6_8: usize,
    pub total_cycle_cost: u32,
}

pub fn get_oth_chord_scale(args: GetOthChordScaleArgs) -> Result<GetOthChordScaleResponse> {
    if args.pitches.len() != 4 {
        return Err(Error::parse(format!(
            "Expected exactly 4 MIDI pitches, got {}",
            args.pitches.len()
        )));
    }
    let pitches: [u8; 4] = [args.pitches[0], args.pitches[1], args.pitches[2], args.pitches[3]];
    let vc = VoicedChord::new(pitches)
        .map_err(|e| Error::parse(format!("Invalid voiced chord: {}", e)))?;

    let cs = chord_scale(&vc);
    let cycle = inversion_cycle(&vc);

    // Build voiced chord info
    let input_is = vc.interval_structure();
    let voiced_info = VoicedChordInfo {
        pitches: vc.pitches.to_vec(),
        notes: vc.pitches.iter().map(|&m| midi_to_note_name(m)).collect(),
        interval_structure: input_is.intervals(),
        span: vc.pitches[3] - vc.pitches[0],
    };

    // Build chord scale info
    let cs_info = ChordScaleInfo {
        pcs: cs.pcs.to_vec(),
        notes: cs.pcs.iter().map(|&pc| pc_to_note_name(pc).to_string()).collect(),
        steps: cs.steps.to_vec(),
    };

    // Classify orbit and fiber class from the input chord
    let pc_chord = vc
        .to_pc_chord()
        .map_err(|e| Error::parse(format!("Could not project to PC chord: {}", e)))?;
    let orbit_obj = classify_orbit(&pc_chord).map(|o| build_orbit_object(&o));
    let fc = fiber_class(&pc_chord).map(|f| format_fiber_class(f).to_string());

    // Build inversion cycle
    let labels = ["root", "1st", "2nd", "3rd"];
    let mut inversions = Vec::with_capacity(4);
    let mut in_6_8_count = 0usize;
    let mut total_cost = 0u32;

    for (i, inv) in cycle.iter().enumerate() {
        let inv_is = inv.interval_structure();
        let legal = inv_is.is_legal();
        if legal {
            in_6_8_count += 1;
        }
        let l1 = if i == 0 {
            None
        } else {
            let d = l1_distance(&cycle[i - 1], inv);
            total_cost += d;
            Some(d)
        };
        // Add the wrap-around distance (last -> first) to total cost
        if i == 3 {
            total_cost += l1_distance(inv, &cycle[0]);
        }

        inversions.push(InversionInfo {
            inversion: i as u8,
            label: labels[i].to_string(),
            pitches: inv.pitches.to_vec(),
            notes: inv.pitches.iter().map(|&m| midi_to_note_name(m)).collect(),
            intervals: inv_is.intervals(),
            span: inv.pitches[3] - inv.pitches[0],
            in_6_8: legal,
            l1_from_previous: l1,
        });
    }

    Ok(GetOthChordScaleResponse {
        voiced_chord: voiced_info,
        chord_scale: cs_info,
        orbit: orbit_obj,
        inversion_cycle: inversions,
        fiber_class: fc,
        inversions_in_6_8: in_6_8_count,
        total_cycle_cost: total_cost,
    })
}

// ============================================================================
// Tier 2 Tool 1: list_oth_modes
// ============================================================================

#[derive(Serialize, Clone)]
pub struct ModeObjectWithOrbit {
    pub orbit_quintal_label: String,
    pub rotation: u8,
    pub steps: Vec<u8>,
    pub pcs_from_c: Vec<u8>,
    pub spelled_from_c: Vec<String>,
    pub opening_interval: u8,
    pub opening_interval_name: String,
    pub step_cluster: String,
}

#[derive(Deserialize)]
pub struct ListOthModesArgs {
    #[serde(default)]
    pub opening_interval: Option<u8>,
    #[serde(default)]
    pub cluster: Option<String>,
    #[serde(default)]
    pub orbit: Option<String>,
}

#[derive(Serialize)]
pub struct ListOthModesResponse {
    pub total_modes: usize,
    pub filter_applied: serde_json::Value,
    pub filtered_count: usize,
    pub modes: Vec<ModeObjectWithOrbit>,
}

fn build_mode_with_orbit(mode: &OthMode) -> ModeObjectWithOrbit {
    let pcs = mode.pcs_from_c();
    let cluster = step_vocabulary_cluster(&mode.orbit());
    ModeObjectWithOrbit {
        orbit_quintal_label: orbit_to_label(&mode.orbit()),
        rotation: mode.rotation(),
        steps: mode.steps().to_vec(),
        pcs_from_c: pcs.to_vec(),
        spelled_from_c: pcs.iter().map(|&pc| pc_to_note_name(pc).to_string()).collect(),
        opening_interval: mode.opening_interval(),
        opening_interval_name: interval_name(mode.opening_interval()).to_string(),
        step_cluster: format!("{:?}", cluster),
    }
}

pub fn list_oth_modes(args: ListOthModesArgs) -> Result<ListOthModesResponse> {
    // Collect all modes flattened
    let all = all_modes();
    let mut modes: Vec<&OthMode> = all.iter().flat_map(|om| om.modes().iter()).collect();
    let total = modes.len();

    // Build filter description
    let mut filter = serde_json::Map::new();

    // Apply orbit filter
    if let Some(ref orbit_str) = args.orbit {
        let target = parse_orbit_identifier(orbit_str)?;
        modes.retain(|m| m.orbit() == target);
        filter.insert(
            "orbit".to_string(),
            serde_json::Value::String(orbit_to_label(&target)),
        );
    }

    // Apply opening interval filter
    if let Some(oi) = args.opening_interval {
        modes.retain(|m| m.opening_interval() == oi);
        filter.insert(
            "opening_interval".to_string(),
            serde_json::Value::Number(serde_json::Number::from(oi)),
        );
    }

    // Apply cluster filter
    if let Some(ref cluster_str) = args.cluster {
        let cluster = parse_cluster(cluster_str)?;
        modes.retain(|m| step_vocabulary_cluster(&m.orbit()) == cluster);
        filter.insert(
            "cluster".to_string(),
            serde_json::Value::String(cluster_str.clone()),
        );
    }

    let filtered_count = modes.len();
    let mode_objs: Vec<ModeObjectWithOrbit> = modes.iter().map(|m| build_mode_with_orbit(m)).collect();

    let filter_val = if filter.is_empty() {
        serde_json::Value::Null
    } else {
        serde_json::Value::Object(filter)
    };

    Ok(ListOthModesResponse {
        total_modes: total,
        filter_applied: filter_val,
        filtered_count,
        modes: mode_objs,
    })
}

// ============================================================================
// Tier 2 Tool 2: get_oth_distance
// ============================================================================

#[derive(Deserialize)]
pub struct ChordInput {
    #[serde(default)]
    pub pcs: Option<Vec<u8>>,
    #[serde(default)]
    pub notes: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct GetOthDistanceArgs {
    pub from: ChordInput,
    pub to: ChordInput,
}

#[derive(Serialize)]
pub struct GetOthDistanceResponse {
    pub from: ChordSummary,
    pub to: ChordSummary,
    pub distance: u8,
    pub diameter: u8,
    pub geodesic_count: usize,
}

pub fn get_oth_distance(args: GetOthDistanceArgs) -> Result<GetOthDistanceResponse> {
    let from_chord = parse_chord_input(args.from.pcs, args.from.notes)?;
    let to_chord = parse_chord_input(args.to.pcs, args.to.notes)?;
    let space = get_space();

    let dist = distance(space, &from_chord, &to_chord).ok_or_else(|| {
        Error::parse("One or both chords are not in the [6,8] base space".to_string())
    })?;

    let gc = music_comp_mt::quintal::count_geodesics(space, &from_chord, &to_chord);

    Ok(GetOthDistanceResponse {
        from: build_chord_summary(&from_chord),
        to: build_chord_summary(&to_chord),
        distance: dist,
        diameter: get_diameter(),
        geodesic_count: gc,
    })
}

// ============================================================================
// Tier 2 Tool 3: get_oth_neighbors
// ============================================================================

#[derive(Deserialize)]
pub struct GetOthNeighborsArgs {
    #[serde(default)]
    pub pcs: Option<Vec<u8>>,
    #[serde(default)]
    pub notes: Option<Vec<String>>,
    #[serde(default)]
    pub orbit: Option<String>,
}

#[derive(Serialize)]
pub struct NeighborInfo {
    pub pcs: Vec<u8>,
    pub notes: Vec<String>,
    pub orbit: String,
    pub voice_moved: u8,
    pub direction: String,
    pub common_tones: u8,
}

#[derive(Serialize)]
pub struct GetOthNeighborsResponse {
    pub chord: ChordSummary,
    pub degree: usize,
    pub neighbors: Vec<NeighborInfo>,
    pub neighbor_orbit_distribution: BTreeMap<String, usize>,
}

pub fn get_oth_neighbors(args: GetOthNeighborsArgs) -> Result<GetOthNeighborsResponse> {
    let chord = parse_chord_input(args.pcs, args.notes)?;
    let space = get_space();

    let neighbor_indices = space.neighbors(&chord).ok_or_else(|| {
        Error::parse("Chord is not in the [6,8] base space".to_string())
    })?;

    let degree = neighbor_indices.len();
    let chords = space.chords();

    // Build all neighbors and orbit distribution (pre-filter)
    let mut distribution: BTreeMap<String, usize> = BTreeMap::new();
    let mut all_neighbors: Vec<NeighborInfo> = Vec::with_capacity(degree);

    for &idx in neighbor_indices {
        let neighbor = &chords[idx];
        let orbit_label = classify_orbit(neighbor)
            .map(|o| orbit_to_label(&o))
            .unwrap_or_else(|| "unknown".to_string());

        *distribution.entry(orbit_label.clone()).or_insert(0) += 1;

        let movement = compute_voice_movement(&chord, neighbor)?;
        all_neighbors.push(NeighborInfo {
            pcs: neighbor.pcs().to_vec(),
            notes: neighbor.pcs().iter().map(|&pc| pc_to_note_name(pc).to_string()).collect(),
            orbit: orbit_label,
            voice_moved: movement.voice,
            direction: movement.direction,
            common_tones: 3,
        });
    }

    // Apply orbit filter (distribution remains pre-filter)
    let neighbors = if let Some(ref orbit_str) = args.orbit {
        let target = parse_orbit_identifier(orbit_str)?;
        let target_label = orbit_to_label(&target);
        all_neighbors
            .into_iter()
            .filter(|n| n.orbit == target_label)
            .collect()
    } else {
        all_neighbors
    };

    Ok(GetOthNeighborsResponse {
        chord: build_chord_summary(&chord),
        degree,
        neighbors,
        neighbor_orbit_distribution: distribution,
    })
}

// ============================================================================
// Tier 2 Tool 4: verify_oth_properties
// ============================================================================

#[derive(Deserialize)]
pub struct VerifyOthPropertiesArgs {
    #[serde(default = "default_check_all")]
    pub check: String,
}

fn default_check_all() -> String {
    "all".to_string()
}

#[derive(Serialize)]
pub struct CheckResult {
    pub passed: bool,
    pub detail: String,
}

#[derive(Serialize)]
pub struct VerifyOthPropertiesResponse {
    pub checks_run: Vec<String>,
    pub results: BTreeMap<String, CheckResult>,
    pub summary: String,
}

pub fn verify_oth_properties(args: VerifyOthPropertiesArgs) -> Result<VerifyOthPropertiesResponse> {
    let check = args.check.to_lowercase();
    let run_all = check == "all";

    let mut results = BTreeMap::new();
    let mut checks_run = Vec::new();

    // fiber_modes
    if run_all || check == "fiber_modes" {
        checks_run.push("fiber_modes".to_string());
        let result = music_comp_mt::quintal::verify_fiber_mode_connection();
        results.insert(
            "fiber_modes".to_string(),
            CheckResult {
                passed: result.is_ok(),
                detail: match result {
                    Ok(()) => "All 14 orbits x 4 rotations: mode rotation matches fiber projection"
                        .to_string(),
                    Err(e) => format!("Fiber-mode mismatch: {:?}", e),
                },
            },
        );
    }

    // multisets
    if run_all || check == "multisets" {
        checks_run.push("multisets".to_string());
        let result = music_comp_mt::quintal::verify_multiset_uniqueness();
        results.insert(
            "multisets".to_string(),
            CheckResult {
                passed: result.is_ok(),
                detail: match result {
                    Ok(()) => "All 14 orbits have unique step-size multisets".to_string(),
                    Err(e) => format!("Multiset collision: {:?}", e),
                },
            },
        );
    }

    // l1_law
    if run_all || check == "l1_law" {
        checks_run.push("l1_law".to_string());
        let space = get_space();
        let result = verify_universal_l1_law(space);
        results.insert(
            "l1_law".to_string(),
            CheckResult {
                passed: result.is_ok(),
                detail: match result {
                    Ok(()) => {
                        "Universal L1 Law verified: [12,12,12,36] for all 228 chords".to_string()
                    }
                    Err(failures) => format!(
                        "L1 Law violations in {} chords: {:?}",
                        failures.len(),
                        &failures[..failures.len().min(5)]
                    ),
                },
            },
        );
    }

    // duality
    if run_all || check == "duality" {
        checks_run.push("duality".to_string());
        let space = get_space();
        let passed = verify_all_orbits_self_dual(space);
        results.insert(
            "duality".to_string(),
            CheckResult {
                passed,
                detail: if passed {
                    "All 14 orbits are self-dual under quartal/quintal duality".to_string()
                } else {
                    "Some orbits are not self-dual".to_string()
                },
            },
        );
    }

    let passed_count = results.values().filter(|r| r.passed).count();
    let total = results.len();
    let summary = if passed_count == total {
        format!("All {} checks passed.", total)
    } else {
        let failed: Vec<String> = results
            .iter()
            .filter(|(_, r)| !r.passed)
            .map(|(k, _)| k.clone())
            .collect();
        format!(
            "{} of {} checks passed. Failed: {}. \
             Note: multiset collision is a known theoretical finding, not a bug.",
            passed_count,
            total,
            failed.join(", ")
        )
    };

    Ok(VerifyOthPropertiesResponse {
        checks_run,
        results,
        summary,
    })
}

// ============================================================================
// Tier 3 Tool 1: get_oth_geodesics
// ============================================================================

#[derive(Deserialize)]
pub struct GetOthGeodesicsArgs {
    pub from: ChordInput,
    pub to: ChordInput,
    #[serde(default = "default_max_paths")]
    pub max_paths: usize,
}

fn default_max_paths() -> usize {
    10
}

#[derive(Serialize)]
pub struct StepMovement {
    pub step: usize,
    pub voice: u8,
    pub from_pc: u8,
    pub to_pc: u8,
    pub direction: String,
}

#[derive(Serialize)]
pub struct GeodesicPath {
    pub chords: Vec<ChordSummary>,
    pub voice_movements: Vec<StepMovement>,
}

#[derive(Serialize)]
pub struct GetOthGeodesicsResponse {
    pub from_pcs: Vec<u8>,
    pub to_pcs: Vec<u8>,
    pub distance: u8,
    pub total_geodesics: usize,
    pub paths_returned: usize,
    pub paths: Vec<GeodesicPath>,
}

pub fn get_oth_geodesics(args: GetOthGeodesicsArgs) -> Result<GetOthGeodesicsResponse> {
    let from_chord = parse_chord_input(args.from.pcs, args.from.notes)?;
    let to_chord = parse_chord_input(args.to.pcs, args.to.notes)?;
    let space = get_space();

    let dist = distance(space, &from_chord, &to_chord).ok_or_else(|| {
        Error::parse("One or both chords are not in the [6,8] base space".to_string())
    })?;

    let total = music_comp_mt::quintal::count_geodesics(space, &from_chord, &to_chord);
    let all_paths = music_comp_mt::quintal::geodesics(space, &from_chord, &to_chord);

    let paths: Vec<GeodesicPath> = all_paths
        .into_iter()
        .take(args.max_paths)
        .map(|path| {
            let mut voice_movements = Vec::new();
            for (i, window) in path.windows(2).enumerate() {
                if let Ok(mv) = compute_voice_movement(&window[0], &window[1]) {
                    voice_movements.push(StepMovement {
                        step: i + 1,
                        voice: mv.voice,
                        from_pc: mv.from_pc,
                        to_pc: mv.to_pc,
                        direction: mv.direction,
                    });
                }
            }
            let chords = path.iter().map(build_chord_summary).collect();
            GeodesicPath {
                chords,
                voice_movements,
            }
        })
        .collect();

    let paths_returned = paths.len();

    Ok(GetOthGeodesicsResponse {
        from_pcs: from_chord.pcs().to_vec(),
        to_pcs: to_chord.pcs().to_vec(),
        distance: dist,
        total_geodesics: total,
        paths_returned,
        paths,
    })
}

// ============================================================================
// Tier 3 Tool 2: get_oth_crossroads
// ============================================================================

#[derive(Deserialize)]
pub struct GetOthCrossroadsArgs {}

#[derive(Serialize, Clone)]
pub struct CrossroadsChordInfo {
    pub pcs: Vec<u8>,
    pub notes: Vec<String>,
    pub degree: usize,
    pub betweenness_centrality: f64,
    pub t6_partner_pcs: Vec<u8>,
    pub t6_partner_notes: Vec<String>,
    pub is_self_t6: bool,
}

#[derive(Serialize)]
pub struct CrossroadsProperties {
    pub total_chords: usize,
    pub fiber_class: String,
}

#[derive(Serialize)]
pub struct GetOthCrossroadsResponse {
    pub orbit: OrbitObject,
    pub crossroads_chords: Vec<CrossroadsChordInfo>,
    pub structural_properties: CrossroadsProperties,
}

pub fn get_oth_crossroads(_args: GetOthCrossroadsArgs) -> Result<GetOthCrossroadsResponse> {
    let space = get_space();
    let centrality = get_centrality();
    let chords = crossroads_chords(space);

    let orbit_obj = build_orbit_object(&Orbit::Q686);

    let chord_infos: Vec<CrossroadsChordInfo> = chords
        .iter()
        .map(|chord| {
            let t6 = transpose(chord, 6);
            let is_self_t6 = *chord == t6;
            let bc = centrality.get(chord).copied().unwrap_or(0.0);
            let deg = space.degree(chord).unwrap_or(0);

            CrossroadsChordInfo {
                pcs: chord.pcs().to_vec(),
                notes: chord.pcs().iter().map(|&pc| pc_to_note_name(pc).to_string()).collect(),
                degree: deg,
                betweenness_centrality: bc,
                t6_partner_pcs: t6.pcs().to_vec(),
                t6_partner_notes: t6.pcs().iter().map(|&pc| pc_to_note_name(pc).to_string()).collect(),
                is_self_t6,
            }
        })
        .collect();

    let fc = fiber_class(&chords[0])
        .map(format_fiber_class)
        .unwrap_or("unknown");

    Ok(GetOthCrossroadsResponse {
        orbit: orbit_obj,
        crossroads_chords: chord_infos.clone(),
        structural_properties: CrossroadsProperties {
            total_chords: chord_infos.len(),
            fiber_class: fc.to_string(),
        },
    })
}

// ============================================================================
// Tier 3 Tool 3: find_oth_modes_by_opening
// ============================================================================

#[derive(Deserialize)]
pub struct FindOthModesByOpeningArgs {
    #[serde(default)]
    pub interval: Option<u8>,
}

#[derive(Serialize)]
pub struct OpeningIntervalGroup {
    pub name: String,
    pub count: usize,
    pub modes: Vec<ModeObjectWithOrbit>,
}

#[derive(Serialize)]
pub struct FindOthModesByOpeningResponse {
    pub opening_intervals: BTreeMap<String, OpeningIntervalGroup>,
    pub total_modes: usize,
}

pub fn find_oth_modes_by_opening(
    args: FindOthModesByOpeningArgs,
) -> Result<FindOthModesByOpeningResponse> {
    let intervals: Vec<u8> = match args.interval {
        Some(i) => {
            if !(1..=6).contains(&i) {
                return Err(Error::parse(format!(
                    "Opening interval must be 1-6, got {}",
                    i
                )));
            }
            vec![i]
        }
        None => (1..=6).collect(),
    };

    let mut groups = BTreeMap::new();
    let mut total = 0usize;

    for i in intervals {
        let modes = modes_by_opening_interval(i);
        let mode_objs: Vec<ModeObjectWithOrbit> =
            modes.iter().map(build_mode_with_orbit).collect();
        let count = mode_objs.len();
        total += count;
        groups.insert(
            i.to_string(),
            OpeningIntervalGroup {
                name: interval_name(i).to_string(),
                count,
                modes: mode_objs,
            },
        );
    }

    Ok(FindOthModesByOpeningResponse {
        opening_intervals: groups,
        total_modes: total,
    })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Helper tests ---

    #[test]
    fn test_pc_to_note_name() {
        assert_eq!(pc_to_note_name(0), "C");
        assert_eq!(pc_to_note_name(1), "Db");
        assert_eq!(pc_to_note_name(3), "Eb");
        assert_eq!(pc_to_note_name(6), "F#");
        assert_eq!(pc_to_note_name(8), "Ab");
        assert_eq!(pc_to_note_name(10), "Bb");
        assert_eq!(pc_to_note_name(11), "B");
    }

    #[test]
    fn test_midi_to_note_name() {
        assert_eq!(midi_to_note_name(60), "C4");
        assert_eq!(midi_to_note_name(55), "G3");
        assert_eq!(midi_to_note_name(48), "C3");
        assert_eq!(midi_to_note_name(69), "A4");
    }

    #[test]
    fn test_parse_orbit_identifier_quintal_label() {
        let orbit = parse_orbit_identifier("Q777").unwrap();
        assert_eq!(orbit, Orbit::Q777);
    }

    #[test]
    fn test_parse_orbit_identifier_quartal_label() {
        let orbit = parse_orbit_identifier("Q555").unwrap();
        assert_eq!(orbit, Orbit::Q777);
    }

    #[test]
    fn test_parse_orbit_identifier_interval_structure() {
        let orbit = parse_orbit_identifier("7,7,7").unwrap();
        assert_eq!(orbit, Orbit::Q777);
    }

    #[test]
    fn test_parse_orbit_identifier_bracketed() {
        let orbit = parse_orbit_identifier("[7,7,7]").unwrap();
        assert_eq!(orbit, Orbit::Q777);
    }

    #[test]
    fn test_parse_orbit_identifier_invalid() {
        let err = parse_orbit_identifier("Q999");
        assert!(err.is_err());
        let msg = format!("{}", err.unwrap_err());
        assert!(msg.contains("Q777"));
    }

    #[test]
    fn test_parse_chord_pcs_valid() {
        let chord = parse_chord_pcs(vec![0, 2, 7, 9]).unwrap();
        assert_eq!(chord.pcs().len(), 4);
    }

    #[test]
    fn test_parse_chord_pcs_out_of_range() {
        assert!(parse_chord_pcs(vec![0, 2, 7, 13]).is_err());
    }

    #[test]
    fn test_parse_chord_pcs_wrong_count() {
        assert!(parse_chord_pcs(vec![0, 2, 7]).is_err());
    }

    #[test]
    fn test_format_fiber_class() {
        assert_eq!(format_fiber_class(FiberClass::ClassA), "A");
        assert_eq!(format_fiber_class(FiberClass::ClassB), "B");
    }

    #[test]
    fn test_parse_cluster() {
        assert_eq!(
            parse_cluster("NoSemitoneNoTritone").unwrap(),
            StepVocabularyCluster::NoSemitoneNoTritone
        );
        assert_eq!(
            parse_cluster("evenstepsonly").unwrap(),
            StepVocabularyCluster::EvenStepsOnly
        );
        assert!(parse_cluster("invalid").is_err());
    }

    // --- Tool 1: get_oth_orbit_info ---

    #[test]
    fn test_get_oth_orbit_info_q777() {
        let result = get_oth_orbit_info(GetOthOrbitInfoArgs {
            orbit: "Q777".to_string(),
        })
        .unwrap();
        assert_eq!(result.orbit.quintal_label, "Q777");
        assert_eq!(result.orbit.quartal_label, "Q555");
        assert_eq!(result.orbit.interval_structure, [7, 7, 7]);
        assert_eq!(result.orbit.degree, 8);
        assert_eq!(result.orbit.orbit_size, 12);
        assert_eq!(result.distinct_mode_count, 4);
        assert!(!result.parent_scales.is_empty());
    }

    #[test]
    fn test_get_oth_orbit_info_q686() {
        let result = get_oth_orbit_info(GetOthOrbitInfoArgs {
            orbit: "Q686".to_string(),
        })
        .unwrap();
        assert_eq!(result.orbit.quintal_label, "Q686");
        assert_eq!(result.orbit.degree, 8);
        assert_eq!(result.orbit.orbit_size, 6);
        assert_eq!(result.distinct_mode_count, 2);
    }

    #[test]
    fn test_get_oth_orbit_info_quartal_label() {
        let result = get_oth_orbit_info(GetOthOrbitInfoArgs {
            orbit: "Q555".to_string(),
        })
        .unwrap();
        assert_eq!(result.orbit.quintal_label, "Q777");
    }

    #[test]
    fn test_get_oth_orbit_info_invalid() {
        assert!(get_oth_orbit_info(GetOthOrbitInfoArgs {
            orbit: "invalid".to_string(),
        })
        .is_err());
    }

    // --- Tool 2: list_oth_orbits ---

    #[test]
    fn test_list_oth_orbits_all() {
        let result = list_oth_orbits(ListOthOrbitsArgs { cluster: None }).unwrap();
        assert_eq!(result.total_orbits, 14);
    }

    #[test]
    fn test_list_oth_orbits_cluster_filter() {
        let result = list_oth_orbits(ListOthOrbitsArgs {
            cluster: Some("EvenStepsOnly".to_string()),
        })
        .unwrap();
        assert!(result.total_orbits > 0);
        assert!(result.total_orbits < 14);
    }

    #[test]
    fn test_list_oth_orbits_invalid_cluster() {
        assert!(list_oth_orbits(ListOthOrbitsArgs {
            cluster: Some("invalid".to_string()),
        })
        .is_err());
    }

    // --- Tool 3: get_oth_parent_scales ---

    #[test]
    fn test_get_oth_parent_scales_by_orbit() {
        let result = get_oth_parent_scales(GetOthParentScalesArgs {
            orbit: Some("Q777".to_string()),
            pcs: None,
        })
        .unwrap();
        assert!(!result.parent_scales.is_empty());
        assert_eq!(result.query_orbit, Some("Q777".to_string()));
    }

    #[test]
    fn test_get_oth_parent_scales_by_pcs() {
        let result = get_oth_parent_scales(GetOthParentScalesArgs {
            orbit: None,
            pcs: Some(vec![0, 2, 7, 9]),
        })
        .unwrap();
        assert!(!result.parent_scales.is_empty());
    }

    #[test]
    fn test_get_oth_parent_scales_neither() {
        assert!(get_oth_parent_scales(GetOthParentScalesArgs {
            orbit: None,
            pcs: None,
        })
        .is_err());
    }

    // --- Tool 4: get_oth_chord_info ---

    #[test]
    fn test_get_oth_chord_info_legal() {
        let result = get_oth_chord_info(GetOthChordInfoArgs {
            pcs: Some(vec![0, 2, 7, 9]),
            notes: None,
        })
        .unwrap();
        assert!(result.is_in_space);
        assert!(result.orbit.is_some());
        assert_eq!(result.orbit.unwrap().orbit.quintal_label, "Q777");
    }

    #[test]
    fn test_get_oth_chord_info_illegal() {
        let result = get_oth_chord_info(GetOthChordInfoArgs {
            pcs: Some(vec![0, 4, 7, 11]),
            notes: None,
        })
        .unwrap();
        assert!(!result.is_in_space);
        assert!(result.orbit.is_none());
        assert!(result.reason.is_some());
    }

    #[test]
    fn test_get_oth_chord_info_by_notes() {
        let result = get_oth_chord_info(GetOthChordInfoArgs {
            pcs: None,
            notes: Some(vec![
                "C".to_string(),
                "D".to_string(),
                "G".to_string(),
                "A".to_string(),
            ]),
        })
        .unwrap();
        assert!(result.is_in_space);
    }

    // --- Tool 5: get_oth_chord_scale ---

    #[test]
    fn test_get_oth_chord_scale_q777() {
        let result = get_oth_chord_scale(GetOthChordScaleArgs {
            pitches: vec![48, 55, 62, 69],
        })
        .unwrap();
        assert_eq!(result.voiced_chord.pitches, vec![48, 55, 62, 69]);
        assert_eq!(result.voiced_chord.interval_structure, [7, 7, 7]);
        assert_eq!(result.chord_scale.steps.iter().sum::<u8>(), 12);
        assert_eq!(result.inversion_cycle.len(), 4);
        assert!(result.inversion_cycle[0].l1_from_previous.is_none());
        assert!(result.orbit.is_some());
    }

    #[test]
    fn test_get_oth_chord_scale_l1_distances() {
        let result = get_oth_chord_scale(GetOthChordScaleArgs {
            pitches: vec![48, 55, 62, 69],
        })
        .unwrap();
        // Universal L1 Law: consecutive inversions are L1 distance 12
        for inv in &result.inversion_cycle[1..] {
            assert_eq!(inv.l1_from_previous, Some(12));
        }
    }

    #[test]
    fn test_get_oth_chord_scale_invalid_pitches() {
        // Non-ascending pitches
        assert!(get_oth_chord_scale(GetOthChordScaleArgs {
            pitches: vec![69, 62, 55, 48],
        })
        .is_err());
    }

    #[test]
    fn test_get_oth_chord_scale_wrong_count() {
        assert!(get_oth_chord_scale(GetOthChordScaleArgs {
            pitches: vec![48, 55, 62],
        })
        .is_err());
    }

    // --- Tier 2 Tool 1: list_oth_modes ---

    #[test]
    fn test_list_oth_modes_all() {
        let result = list_oth_modes(ListOthModesArgs {
            opening_interval: None,
            cluster: None,
            orbit: None,
        })
        .unwrap();
        assert_eq!(result.total_modes, 52);
        assert_eq!(result.filtered_count, 52);
    }

    #[test]
    fn test_list_oth_modes_by_opening_interval() {
        let result = list_oth_modes(ListOthModesArgs {
            opening_interval: Some(1),
            cluster: None,
            orbit: None,
        })
        .unwrap();
        assert!(result.filtered_count > 0);
        assert!(result.filtered_count < 52);
        for mode in &result.modes {
            assert_eq!(mode.opening_interval, 1);
        }
    }

    #[test]
    fn test_list_oth_modes_by_orbit() {
        let result = list_oth_modes(ListOthModesArgs {
            opening_interval: None,
            cluster: None,
            orbit: Some("Q777".to_string()),
        })
        .unwrap();
        assert_eq!(result.filtered_count, 4);
        for mode in &result.modes {
            assert_eq!(mode.orbit_quintal_label, "Q777");
        }
    }

    // --- Tier 2 Tool 2: get_oth_distance ---

    #[test]
    fn test_get_oth_distance_same_chord() {
        let result = get_oth_distance(GetOthDistanceArgs {
            from: ChordInput {
                pcs: Some(vec![0, 2, 7, 9]),
                notes: None,
            },
            to: ChordInput {
                pcs: Some(vec![0, 2, 7, 9]),
                notes: None,
            },
        })
        .unwrap();
        assert_eq!(result.distance, 0);
    }

    #[test]
    fn test_get_oth_distance_known_pair() {
        let result = get_oth_distance(GetOthDistanceArgs {
            from: ChordInput {
                pcs: Some(vec![0, 2, 7, 9]),
                notes: None,
            },
            to: ChordInput {
                pcs: Some(vec![0, 2, 6, 8]),
                notes: None,
            },
        })
        .unwrap();
        assert!(result.distance > 0);
        assert!(result.diameter > 0);
        assert!(result.geodesic_count >= 1);
    }

    // --- Tier 2 Tool 3: get_oth_neighbors ---

    #[test]
    fn test_get_oth_neighbors_q777() {
        let result = get_oth_neighbors(GetOthNeighborsArgs {
            pcs: Some(vec![0, 2, 7, 9]),
            notes: None,
            orbit: None,
        })
        .unwrap();
        assert_eq!(result.degree, 8);
        assert_eq!(result.neighbors.len(), 8);
        for n in &result.neighbors {
            assert_eq!(n.common_tones, 3);
        }
        let dist_sum: usize = result.neighbor_orbit_distribution.values().sum();
        assert_eq!(dist_sum, 8);
    }

    #[test]
    fn test_get_oth_neighbors_with_orbit_filter() {
        let result = get_oth_neighbors(GetOthNeighborsArgs {
            pcs: Some(vec![0, 2, 7, 9]),
            notes: None,
            orbit: Some("Q778".to_string()),
        })
        .unwrap();
        // Degree is still the full degree
        assert_eq!(result.degree, 8);
        // But filtered neighbors should be a subset
        for n in &result.neighbors {
            assert_eq!(n.orbit, "Q778");
        }
    }

    // --- Tier 2 Tool 4: verify_oth_properties ---

    #[test]
    fn test_verify_fiber_modes() {
        let result = verify_oth_properties(VerifyOthPropertiesArgs {
            check: "fiber_modes".to_string(),
        })
        .unwrap();
        assert_eq!(result.checks_run.len(), 1);
        assert!(result.results["fiber_modes"].passed);
    }

    #[test]
    fn test_verify_multisets() {
        let result = verify_oth_properties(VerifyOthPropertiesArgs {
            check: "multisets".to_string(),
        })
        .unwrap();
        // This is expected to fail (known collision)
        assert!(!result.results["multisets"].passed);
    }

    #[test]
    fn test_verify_all() {
        let result = verify_oth_properties(VerifyOthPropertiesArgs {
            check: "all".to_string(),
        })
        .unwrap();
        assert_eq!(result.checks_run.len(), 4);
        assert!(result.results["fiber_modes"].passed);
        assert!(result.results["l1_law"].passed);
        assert!(result.results["duality"].passed);
    }

    // --- Tier 3 Tool 1: get_oth_geodesics ---

    #[test]
    fn test_get_oth_geodesics_adjacent() {
        // Find two adjacent chords: [0,2,7,9] and one of its neighbors
        let neighbors = get_oth_neighbors(GetOthNeighborsArgs {
            pcs: Some(vec![0, 2, 7, 9]),
            notes: None,
            orbit: None,
        })
        .unwrap();
        let neighbor_pcs = neighbors.neighbors[0].pcs.clone();

        let result = get_oth_geodesics(GetOthGeodesicsArgs {
            from: ChordInput {
                pcs: Some(vec![0, 2, 7, 9]),
                notes: None,
            },
            to: ChordInput {
                pcs: Some(neighbor_pcs),
                notes: None,
            },
            max_paths: 10,
        })
        .unwrap();
        assert_eq!(result.distance, 1);
        assert!(result.total_geodesics >= 1);
        assert_eq!(result.paths[0].voice_movements.len(), 1);
    }

    #[test]
    fn test_get_oth_geodesics_max_paths() {
        let result = get_oth_geodesics(GetOthGeodesicsArgs {
            from: ChordInput {
                pcs: Some(vec![0, 2, 7, 9]),
                notes: None,
            },
            to: ChordInput {
                pcs: Some(vec![0, 2, 6, 8]),
                notes: None,
            },
            max_paths: 1,
        })
        .unwrap();
        assert_eq!(result.paths_returned, 1);
    }

    // --- Tier 3 Tool 2: get_oth_crossroads ---

    #[test]
    fn test_get_oth_crossroads() {
        let result = get_oth_crossroads(GetOthCrossroadsArgs {}).unwrap();
        assert_eq!(result.crossroads_chords.len(), 6);
        assert_eq!(result.orbit.quintal_label, "Q686");
        for chord in &result.crossroads_chords {
            assert!(chord.betweenness_centrality > 0.0);
            assert!(chord.degree > 0);
        }
        assert_eq!(result.structural_properties.total_chords, 6);
    }

    // --- Tier 3 Tool 3: find_oth_modes_by_opening ---

    #[test]
    fn test_find_modes_by_opening_all() {
        let result = find_oth_modes_by_opening(FindOthModesByOpeningArgs {
            interval: None,
        })
        .unwrap();
        assert_eq!(result.opening_intervals.len(), 6);
        assert_eq!(result.total_modes, 52);
    }

    #[test]
    fn test_find_modes_by_opening_specific() {
        let result = find_oth_modes_by_opening(FindOthModesByOpeningArgs {
            interval: Some(6),
        })
        .unwrap();
        assert_eq!(result.opening_intervals.len(), 1);
        let group = &result.opening_intervals["6"];
        assert_eq!(group.name, "tritone");
        assert!(group.count > 0);
    }

    #[test]
    fn test_find_modes_by_opening_invalid() {
        assert!(find_oth_modes_by_opening(FindOthModesByOpeningArgs {
            interval: Some(7),
        })
        .is_err());
    }
}
