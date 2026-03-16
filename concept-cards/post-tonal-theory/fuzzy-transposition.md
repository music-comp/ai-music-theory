---
concept: Fuzzy Transposition
slug: fuzzy-transposition
category: voice-leading
subcategory: "fuzzy T/I"
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Motive, Voice Leading, and Harmony"
chapter_number: 4
pdf_page: 175
section: "4.3.2 Fuzzy transposition and inversion"
extraction_confidence: high
aliases:
  - "T*"
  - "*Tn"
prerequisites:
  - transposition
  - transformational-voice-leading
extends:
  - transposition
related:
  - fuzzy-inversion
  - voice-leading-offset
contrasts_with:
  - transposition
answers_questions:
  - "What is fuzzy transposition?"
---

# Quick Definition

A near-transposition where one or more voices deviate slightly from exact transposition, notated as T* (or *Tn) with an offset value indicating the total semitonal deviation from crisp transposition.

# Formal Definition

**Fuzzy transposition** (designated T* or *Tn) describes a voice-leading relationship between two sets that is almost, but not quite, an exact transposition. In fuzzy transposition, most voices follow the expected Tn mapping, but one or more voices deviate by a small amount (typically one semitone). The **offset** measures the total deviation from exact transposition, calculated as the sum of the absolute semitonal differences between actual and expected mappings.

Fuzzy transposition allows analysis of:
- Progressions between different set classes
- Near-transpositional relationships with smooth voice leading
- Unified voice-leading gestures despite set-class changes

Notation: *T5(3) means "fuzzy transposition at T5 with an offset of 3 semitones"

# Mathematical Formulation/Recognition

**Crisp transposition:**
- Tn: Every pc x maps to (x + n) mod 12
- Offset = 0

**Fuzzy transposition:**
- *Tn: Most pcs follow Tn, but some deviate
- Offset = sum of |actual mapping - expected Tn mapping| for all voices

**Calculation of offset:**
1. Identify the "target" transposition level (the closest Tn)
2. For each voice, calculate expected mapping under Tn
3. Calculate actual mapping
4. Offset = sum of |actual - expected| for all voices

**Notation conventions:**
- Solid lines: voices that follow exact transposition
- Dotted lines: voices that deviate (fuzzy mappings)
- *Tn(k): fuzzy transposition at Tn with offset k

**Example:**
- Set 1: {C, E, G}
- Set 2: {C#, F, G#}
- Expected T1: {C#, F, G#} - all voices off by 0
- Actual mapping matches T1 exactly: offset = 0 (crisp)
- If Set 2 were {C#, F, A}: E→F is T1, but G→A deviates by 1; offset = 1

# Musical Context/Application

Fuzzy transposition is useful when:
- **Connecting different set classes**: Two chords of different set classes cannot be related by exact T or I; fuzzy T bridges them
- **Smooth voice leading**: A small offset allows near-parallel motion with slight adjustments
- **Large-scale progressions**: An entire passage may form a single fuzzy transpositional gesture despite local set-class changes
- **Preserving voice-leading logic**: The underlying transpositional structure remains audible despite deviations

Analytical applications:
- Identifying near-transpositional relationships
- Measuring voice-leading distance between chords
- Tracing unified gestures across set-class boundaries

# Examples

**Webern, Movements for String Quartet, op. 5, no. 5** (Example 4-16):
- Six chords representing three different set classes
- First to second chord: exact transposition (same set class)
- Second to third chord: *T2 with offset of 3 (three voices each off by one semitone)
- Fifth to sixth chord: *T with offset of only 1 (one voice off by one semitone)
- Individual moves combine into larger gesture: first to last chord is *T5 with offset of only 2
- The progression is heard as a single, unified gesture with clear, parallel voice leading

**Sessions, Piano Sonata, first movement** (Example 4-17):
- Five chords representing four different set classes
- Fuzzy transpositions and inversions connect chords
- Produces voice crossing in the transformational voice leading
- Demonstrates flexibility of fuzzy operations across varying set classes

**Analytical utility:**
- Even when exact T is impossible (different set classes), *T reveals near-transpositional relationships
- Small offsets indicate smooth voice leading
- Large offsets indicate disjunct voice leading

# Related Concepts

- Transposition (Tn)
- Transformational Voice Leading
- Fuzzy Inversion (I*)
- Voice-Leading Offset
- Set class
- Voice-Leading Space
- Parsimonious voice leading

# Common Confusions

- **Fuzzy vs. crisp**: Crisp transposition (Tn) has offset 0; fuzzy transposition (*Tn) has offset > 0
- **Offset calculation**: The offset is the TOTAL deviation across all voices, not the deviation of a single voice
- **Fuzzy transposition vs. different transformation**: *T is not the same as a different exact T; it acknowledges deviation from the target T
- **Notation variants**: *T5, T*5, and T5* may all appear in literature; the asterisk indicates fuzziness regardless of position

# Source Reference

Chapter 4: Motive, Voice Leading, and Harmony, Section 4.3.2, pages 193-195
