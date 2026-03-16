---
concept: RI-Chaining
slug: ri-chaining

category: transformation-theory
subcategory: serial-operations
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (1): Intervals and Transpositions"
chapter_number: 7
pdf_page: 188
section: "7.2"

extraction_confidence: high

aliases:
  - "retrograde-inversion chaining"

prerequisites:
  - rich-transformation
extends: []
related:
  - tch-transformation
  - structural-sequencing
  - wagner-parsifal-zauber-motive-analysis
contrasts_with: []

answers_questions:
  - "How do I apply RI-chaining (RICH) to a series?"
  - "What is the relationship between RI-chaining and structural sequencing?"
---

# Quick Definition
A compositional and analytical technique where serial forms are linked by the RICH operation, with each new form beginning on the final two notes of the previous form, creating chains of related motivic statements that generate structural sequences.

# Core Definition
RI-chaining links successive serial forms via the RICH transformation: given a series s, RICH(s) is the retrograde-inverted form whose first two elements are s_{N-1} and s_N (the last two of s, in order). When applied repeatedly, alternate forms are transposed: RICH(RICH(s)) = T_i(s), where i = int(s_1, s_N) + int(s_2, s_{N-1}). This TCH interval governs the resulting structural sequences (Lewin, 7.2, p. 164; 8.2.1, p. 180).

# Prerequisites
- **RICH transformation** — The specific operation that generates each link in the chain

# Key Properties
1. Each form is the retrograde-inversion of the preceding form
2. The RI form specifically uses the final two notes of the predecessor as its opening two notes
3. Alternate forms (Z_1 and Z_3, Z_2 and Z_4) are transposed forms of each other
4. The transposition interval depends on the series' internal structure
5. The technique generates structural sequencing when applied repeatedly

# Construction / Recognition
## To Construct:
1. Start with series s = s_1, s_2, ..., s_N
2. Form RICH(s): the RI of s beginning with s_{N-1}, s_N
3. Form RICH(RICH(s)): the RI of RICH(s) beginning with its last two elements
4. Continue the chain as needed
## To Recognize:
1. Successive serial forms overlap by their last/first two elements
2. Each form is an RI of its predecessor
3. Alternate forms are related by transposition at the TCH interval

# Context & Application
RI-chaining appears in Wagner's Parsifal (transformation music, Act 1), Webern's Piano Variations op. 27, Webern's serial works generally, and even in Bach's Two-Part Invention No. 1 (via MUCH chaining). The technique creates deep structural connections between passages whose surfaces may differ entirely.

# Examples
**Example 1** (Figure 7.4, p. 163): The Zauber motive A-C-Eb-E chains via RICH to Eb-E-G-Bb, then to G-Bb-Db-D = T_{10}(A-C-Eb-E). TCH interval = int(A,E) + int(C,Eb) = 7 + 3 = 10.

**Example 2** (Figure 8.4b, p. 182): Webern's Piano Variations row Eb-B-Bb-D-C#-C-F#-E-G-F-A-G# chains with TCH interval = int(Eb,G#) + int(B,A) = 5 + 10 = 3.

# Relationships
## Builds Upon
- **RICH transformation** — The operation generating each link
## Enables
- **Structural sequencing** — The large-scale result of repeated RI-chaining
- **TCH transformation** — TCH = RICH composed with RICH
## Related
- **Wagner Parsifal Zauber motive analysis** — Primary demonstration
- **Webern Piano Variations analysis** — Another key demonstration

# Common Errors
- **Error**: Confusing RI-chaining with generic RI operations
  **Correction**: RI-chaining requires the specific overlap (last two to first two); generic RI does not specify which RI form

# Common Confusions
- **Confusion**: Thinking the TCH interval is a fixed transposition level
  **Clarification**: The TCH interval depends on the series' internal structure; different series produce different levels
- **Confusion**: Expecting structural sequences to match foreground sequences
  **Clarification**: "The musical foregrounds of the two passages are not related sequentially" (p. 164)

# Source Reference
Chapter 7: Transformation Graphs and Networks (1): Intervals and Transpositions, Section 7.2, pages 163-164; Chapter 8, Section 8.2.1, pages 180-181.

# Verification Notes
- Definition source: Direct from 7.2 and 8.2.1
- Confidence rationale: Explicitly defined with worked examples
- Re-extraction notes: Re-extracted from v2 card; preserved: Zauber and Webern examples, TCH interval formula
