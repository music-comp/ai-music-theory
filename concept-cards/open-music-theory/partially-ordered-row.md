---
concept: Partially-Ordered Row
category: theory
source: Open Music Theory
chapter: "Row Properties"
part: 9
---

# Partially-Ordered Row

## Quick Definition

A partially-ordered row (or partially-ordered set) is a twelve-tone structure where only certain segments maintain fixed internal ordering while other segments can be reordered—for example, keeping the discrete hexachords, tetrachords, or trichords in place but allowing pitches within each segment to be permuted freely, representing a relaxation of strict serial ordering while maintaining the aggregate completion (all twelve pitch classes) that defines twelve-tone practice.

## Formal Definition

**Partially-ordered row**: A twelve-tone structure with constrained ordering at the segment level but flexibility within segments.

**Degrees of ordering**:
```
Strict ordering:    1-2-3-4-5-6-7-8-9-10-11-12 (fixed)
Hexachords fixed:   [1-6 can permute] [7-12 can permute]
Tetrachords fixed:  [1-4 permute] [5-8 permute] [9-12 permute]
Trichords fixed:    [1-3] [4-6] [7-9] [10-12] (each permutes)
Dyads fixed:        [1-2] [3-4] [5-6] [7-8] [9-10] [11-12]
```

**Number of orderings with fixed segments**:
```
Any ordering:         12! = 479,001,600
Hexachords fixed:     6! x 6! = 518,400
Tetrachords fixed:    4! x 4! x 4! = 13,824
Trichords fixed:      3! x 3! x 3! x 3! = 1,296
Dyads fixed:          2! x 2! x 2! x 2! x 2! x 2! = 64
```

**Key principle**: Aggregate completion preserved regardless of internal ordering.

## Musical Context

Partial ordering represents:
- **Practical reality**: Chords necessitate partial ordering (pitches sound together)
- **Compositional flexibility**: More options than strict serialism
- **Segment focus**: Emphasis on subsegments rather than complete row
- **Historical precedent**: Many "strict" serialists used partial ordering

**Babbitt's contribution**: Formalized partial ordering concept, showing how segment-based thinking relates to (and differs from) strict serial ordering.

**Compositional implications**:
- Same segments, different orderings = related but distinct rows
- Interval content changes with each permutation
- Segment identity (set class) remains constant
- Aggregate completion maintained

## Examples

### Basic

**Webern Op. 28 row with partial reordering**:
```
Original row:
B-Bb-D-Eb | F#-G-E-F | Ab-A-C-Db
(0123)      (0123)     (0123)

Dyads reversed (partial reordering):
Bb-B-Eb-D | G-F#-F-E | A-Ab-Db-C

Properties:
- Same trichords (sets): (012), (012), (012), (012)
- Same tetrachords (sets): (0123), (0123), (0123)
- Different ORDERING of pitches within segments
- Still a valid twelve-tone row
```

**Why partial ordering matters**:
```
Strict row: 1-2-3-4-5-6-7-8-9-10-11-12

Chordal realization (notes 1-4 as chord):
{1,2,3,4} sounding simultaneously
No specific "order" among chord tones
= Partial ordering in practice

Even "strict" serial music uses chords
Therefore partial ordering is implicit
```

**Counting possibilities**:
```
If hexachords fixed but internal order free:
6! ways to order first hexachord
6! ways to order second hexachord
Total: 720 x 720 = 518,400 rows

Compare: only 48 "strict" row forms

Partial ordering vastly expands possibilities
while maintaining aggregate structure
```

### From Repertoire

**Webern, String Quartet Op. 28**:
```
The row emphasizes tetrachords:
[B-Bb-D-Eb] [F#-G-E-F] [Ab-A-C-Db]

Partial ordering interpretation:
- Tetrachord CONTENT fixed
- Internal tetrachord ORDER variable
- Creates family of related rows

Webern might use:
B-Bb-D-Eb (original order)
D-Eb-B-Bb (reordered)
Bb-D-Eb-B (reordered)
...all "the same" at tetrachord level
```

**Practical compositional use**:
```
Composer establishes:
- Fixed segments (e.g., tetrachords)
- Variable internal ordering
- Aggregate always complete

Benefits:
- Motivic consistency (same sets)
- Textural variety (different orderings)
- Maintains twelve-tone foundation
```

**Relationship to strict serialism**:
```
Strict serialism:
- Fixed complete ordering
- 48 row forms maximum
- Limited flexibility

Partial ordering:
- Fixed segment structure
- Flexible internal ordering
- Many more possibilities
- Still aggregate-based
```

## Related Concepts

- **Prerequisite**: twelve-tone-row, aggregate, discrete-segments
- **Leads to**: total-serialism, combinatoriality, set-theory
- **See also**: derived-row, row-properties, Babbitt

## Common Confusions

- Partially-ordered = segments fixed, internal order variable
- NOT the same as "non-serial" or "free atonal"
- Still maintains aggregate completion (all 12 pitch classes)
- Different from strict serialism (completely fixed order)
- Chordal writing inherently involves partial ordering
- Even "strict" serialists use partial ordering in practice
- Babbitt formalized the concept mathematically
- Number of possible orderings depends on segment size
- Smaller segments = fewer orderings (more constrained)
- Larger segments = more orderings (more flexible)
- Interval content CHANGES with different orderings
- Set-class content of segments STAYS THE SAME
- Common in much twelve-tone practice, even when not explicit
- Relates to focus on cells/sets rather than specific ordering

## Source Reference

Open Music Theory, Part IX: "Row Properties"
