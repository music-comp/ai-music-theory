---
concept: Hexachordal Combinatoriality
category: theory
source: Open Music Theory
chapter: "Row Properties"
pdf_page: null
chapter_number: 9
unit: null
authors: "Open Music Theory contributors"
---

# Hexachordal Combinatoriality

## Quick Definition

Hexachordal combinatoriality is a property where the first hexachord (first six notes) of one row form complements the first hexachord of another row form—meaning together they produce all twelve pitch classes (a complete aggregate)—allowing composers to combine row forms in counterpoint while ensuring complete chromatic coverage, classified as semi-combinatorial (one transformation type works) or all-combinatorial (multiple transformation types work), with only six hexachord types being all-combinatorial.

## Formal Definition

**Hexachordal combinatoriality**: Two row forms whose corresponding hexachords are complementary (together yielding all 12 pitch classes).

**Basic principle**:
```
Row 1: [Hexachord A] [Hexachord B]    (H1-A + H1-B = aggregate)
Row 2: [Hexachord C] [Hexachord D]    (H2-C + H2-D = aggregate)

If combinatorial:
H1-A + H2-C = aggregate (all 12 pitch classes)
H1-B + H2-D = aggregate (all 12 pitch classes)
```

**Types** (Babbitt's terminology):

1. **Semi-combinatorial**: One transformation type produces combinatoriality
   - P-P combinatorial (transposition only)
   - P-I combinatorial (inversion)
   - P-RI combinatorial (retrograde inversion)

2. **All-combinatorial**: Multiple transformations produce combinatoriality
   - Row is combinatorial under P-P, P-I, and P-RI
   - Only six hexachord types have this property

**Built-in combinatoriality** (by definition):
- P and R are always combinatorial (R's first hexachord = P's second)
- I and RI are always combinatorial (same relationship)

## Musical Context

Combinatoriality provides:
- **Aggregate completion**: Ensures all 12 pitches in each hexachordal span
- **Contrapuntal freedom**: Rows combine without pitch repetition
- **Structural clarity**: Hexachordal boundaries articulate form
- **Textural variety**: Multiple rows sound simultaneously

**Historical importance**:
- Central to Babbitt's compositional technique
- Schoenberg used combinatoriality (sometimes intuitively)
- Webern's rows often all-combinatorial
- Foundation of much American twelve-tone practice

## Examples

### Basic

**P-R combinatoriality** (always exists):
```
P0: [0-11-3-7-8-4] [2-6-5-1-9-10]
     Hexachord 1     Hexachord 2

R0: [10-9-1-5-6-2] [4-8-7-3-11-0]
     H1 of R0       H2 of R0

H1 of R0 = H2 of P0 (same pitches!)

Therefore: H1 of P0 + H1 of R0 = aggregate
          {0,11,3,7,8,4} + {10,9,1,5,6,2} = all 12
```

**P-I combinatoriality** (not always):
```
For P-I combinatoriality:
- H1 of P must complement H1 of some In
- Row must have special hexachord structure

Example (Webern-type row):
P0: [0-1-2-3-4-5] [6-7-8-9-10-11]
I0: [0-11-10-9-8-7] [6-5-4-3-2-1]

H1 of P0: {0,1,2,3,4,5}
H1 of I0: {0,11,10,9,8,7}
NOT complementary (both contain 0)

Try I6:
H1 of I6: {6,5,4,3,2,1}
P0-H1 + I6-H1 = {0,1,2,3,4,5} + {6,5,4,3,2,1}
             = all 12 pitch classes!

P0 and I6 are combinatorial
```

**Semi-combinatorial example**:
```
Row with semi-combinatoriality:
- Works with transposition (P-P)
- Does NOT work with inversion (P-I)
- Combinatorial only under ONE transformation type
```

### From Repertoire

**Webern, Symphonie Op. 21**:
```
Row: 9-6-7-8-4-5 | 11-10-2-1-0-3
     Hexachord 1   Hexachord 2

Both hexachords = set class 6-1 (chromatic hexachord)
{9,6,7,8,4,5} = chromatic segment A4-A
{11,10,2,1,0,3} = chromatic segment B-Eb

Properties:
- All-combinatorial hexachords
- Combinatorial by T, I, and RI
- One of the six all-combinatorial types
```

**Six all-combinatorial hexachord types**:
```
Set class   Prime form        Description
6-1         (012345)          Chromatic hexachord
6-8         (023457)          "Dominant ninth" hexachord
6-32        (024579)          Diatonic hexachord
6-7         (012678)
6-20        (014589)          "Magic" hexachord
6-35        (02468T)          Whole-tone hexachord

These are the ONLY all-combinatorial types
```

**Magic hexachord (014589)**:
```
Webern, Konzert Op. 24 uses (014589)

Properties:
- All-combinatorial
- Combinatorial by T+/-2 and T6
- Contains hexatonic cycle
- Many internal triads
- Used by Schoenberg, Webern, Maderna, Nono
```

## Related Concepts

- **Prerequisite**: twelve-tone-row, set-class
- **Leads to**: aggregate, total-serialism
- **See also**: derived-row, segmental-invariance

## Common Confusions

- Combinatoriality = hexachords of different row forms complement each other
- P and R are ALWAYS combinatorial (by definition)
- I and RI are ALWAYS combinatorial (by definition)
- The interesting cases: P-P, P-I, P-RI combinatoriality
- Semi-combinatorial = works under ONE transformation type
- All-combinatorial = works under MULTIPLE transformation types
- Only SIX hexachord set classes are all-combinatorial
- Combinatoriality ensures aggregate completion in hexachordal spans
- "Aggregate" = complete collection of all 12 pitch classes
- Combinatoriality enables contrapuntal row combinations
- Not all rows are combinatorial (beyond basic P-R, I-RI)
- Babbitt central to developing combinatorial theory
- Combinatoriality is about HEXACHORDS, not the full row
- Same concept could apply to other segment sizes (trichords, etc.)
- Combinatorial pairs can sound simultaneously without pitch duplication

## Source Reference

Open Music Theory, Part IX: "Row Properties"
