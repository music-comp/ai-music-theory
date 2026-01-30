---
concept: Fuzzy Inversion
category: technique
source: Introduction to Post-Tonal Theory (Fifth Edition)
chapter: "Motive, Voice Leading, and Harmony"
chapter_number: 4
pdf_page: 175
unit: null
authors: Joseph N. Straus
---

# Quick Definition

A near-inversion where one or more voices deviate slightly from exact inversion, notated as I* (or *In) with an offset value indicating the total semitonal deviation from crisp inversion.

# Formal Definition

**Fuzzy inversion** (designated I* or *In) describes a voice-leading relationship between two sets that is almost, but not quite, an exact inversion. In fuzzy inversion, most voices follow the expected In mapping, but one or more voices deviate by a small amount (typically one semitone). The **offset** measures the total deviation from exact inversion, calculated as the sum of the absolute semitonal differences between actual and expected mappings.

Fuzzy inversion extends the concept of inversional voice leading to:
- Progressions between different set classes
- Near-inversional relationships with voice-leading adjustments
- Analytical connections that would otherwise be impossible with crisp operations

Like fuzzy transposition, fuzzy inversion reveals underlying voice-leading logic even when exact transformational relationships do not exist.

# Mathematical Formulation/Recognition

**Crisp inversion:**
- In: Every pc x maps to (n - x) mod 12
- Offset = 0

**Fuzzy inversion:**
- *In: Most pcs follow In, but some deviate
- Offset = sum of |actual mapping - expected In mapping| for all voices

**Calculation of offset:**
1. Identify the "target" inversion level (the closest In)
2. For each voice, calculate expected mapping under In
3. Calculate actual mapping
4. Offset = sum of |actual - expected| for all voices

**Notation conventions:**
- Solid lines: voices that follow exact inversion
- Dotted lines: voices that deviate (fuzzy mappings)
- *In(k): fuzzy inversion at In with offset k

**Example:**
- Set 1: {C, E, G} = {0, 4, 7}
- Under I7: 0→7, 4→3, 7→0 = {G, Eb, C}
- If actual Set 2 is {G, E, C} = {7, 4, 0}: the E (4) should be Eb (3)
- One voice deviates by 1 semitone; offset = 1

# Musical Context/Application

Fuzzy inversion is useful when:
- **Connecting different set classes via inversion**: When sets of different classes have near-inversional relationships
- **Preserving common tones**: Slight adjustments may preserve important common tones
- **Voice crossing with flexibility**: Inversional voice leading often involves crossing; fuzzy I allows for smooth adjustments
- **Combined with fuzzy T**: Complex progressions may alternate or combine fuzzy T and fuzzy I

Analytical applications:
- Identifying near-inversional relationships
- Connecting chords that share inversional potential but differ in set class
- Analyzing passages with both transpositional and inversional logic

# Examples

**Sessions, Piano Sonata, first movement** (Example 4-17):
- Five chords representing four different set classes: (014)-(016)-(025)-(015)-(014)
- The progression uses both fuzzy transpositions and fuzzy inversions
- Fuzzy inversions help connect chords of different set classes
- Voice crossing occurs in the transformational voice leading
- The passage demonstrates how fuzzy operations (both T* and I*) can unite diverse harmonic materials

**General application:**
- When a progression seems inversionally motivated but exact In does not work, *In may reveal the underlying relationship
- Fuzzy inversion, like fuzzy transposition, prioritizes smooth voice leading over exact pitch-class transformation
- The offset quantifies how far the actual voice leading deviates from ideal inversional motion

**Comparison with fuzzy transposition:**
- Fuzzy transposition: deviations from parallel motion (all voices same direction)
- Fuzzy inversion: deviations from inversional motion (voices may cross, move in opposite directions)

# Related Concepts

- Inversion (In)
- Transformational Voice Leading
- Fuzzy Transposition (T*)
- Voice-Leading Offset
- Set class
- Voice-Leading Space
- Contextual inversion

# Common Confusions

- **Fuzzy vs. crisp**: Crisp inversion (In) has offset 0; fuzzy inversion (*In) has offset > 0
- **Index selection**: Choosing the correct In to measure deviation from requires identifying the best-fit inversion
- **Fuzzy I vs. fuzzy T**: The same chord progression might be analyzed as either; context and voice-leading patterns determine which is more appropriate
- **Voice crossing**: Inversion naturally produces voice crossing; this is not necessarily a deviation, but part of the expected inversional behavior

# Source Reference

Chapter 4: Motive, Voice Leading, and Harmony, Section 4.3.2, pages 193-195
