---
concept: Row Class
category: theory
source: Open Music Theory
chapter: "Basics of Twelve-Tone Theory"
pdf_page: null
chapter_number: 9
unit: null
authors: "Open Music Theory contributors"
---

# Row Class

## Quick Definition

A row class is the complete collection of row forms related to a given twelve-tone row by the operations of transposition, inversion, retrograde, and retrograde inversion—typically containing 48 distinct forms (12 transpositions each of P, I, R, and RI), though rows with special symmetry properties may have fewer distinct forms (24 or 12), representing all the pitch material available when composing with that particular row.

## Formal Definition

**Row class**: The set of all row forms related by serial operations.

**Standard row class structure**:
```
Prime forms:              P0, P1, P2, P3... P11  (12 forms)
Inversion forms:          I0, I1, I2, I3... I11  (12 forms)
Retrograde forms:         R0, R1, R2, R3... R11  (12 forms)
Retrograde inversion:     RI0, RI1, RI2... RI11 (12 forms)
                          ________________________________
                          Total: 48 forms
```

**Relationship between forms**:
```
P and R: Exact retrogrades of each other
I and RI: Exact retrogrades of each other
P and I: Inversionally related
R and RI: Inversionally related
```

**Effect of symmetry**:
```
Standard row:           48 distinct forms
R-symmetric row:        24 forms (P/R pairs collapse)
I-symmetric row:        24 forms (P/I pairs collapse)
R and I symmetric:      12 forms (maximum symmetry)
```

## Musical Context

The row class defines:
- **Available pitch material**: All row forms for composition
- **Relationships**: How forms connect to each other
- **Coherence**: All forms share intervallic DNA
- **Variety**: Multiple related forms for development

**Compositional significance**:
- Composer selects from 48 (or fewer) related forms
- Different forms for different sections/voices
- Relationships between forms can articulate structure
- Matrix displays entire row class at once

## Examples

### Basic

**Standard row class** (48 forms):
```
Given P0: 0-11-3-7-8-4-2-6-5-1-9-10

Row class contains:
P0:  0-11-3-7-8-4-2-6-5-1-9-10
P1:  1-0-4-8-9-5-3-7-6-2-10-11
P2:  2-1-5-9-10-6-4-8-7-3-11-0
... (all 12 P forms)

I0:  0-1-9-5-4-8-10-6-7-11-3-2
I1:  1-2-10-6-5-9-11-7-8-0-4-3
... (all 12 I forms)

R0:  10-9-1-5-6-2-4-8-7-3-11-0
... (all 12 R forms)

RI0: 2-3-11-7-6-10-8-4-5-9-1-0
... (all 12 RI forms)

Total: 48 distinct rows, all related
```

**Row class with symmetry** (24 forms):
```
Webern Op. 21:
P0 = R6 (transposed retrograde)

This means:
- P0 and R6 are the same row
- P1 and R7 are the same
- etc.

P/R pairs collapse:
P0=R6, P1=R7, P2=R8, P3=R9, P4=R10, P5=R11
P6=R0, P7=R1, P8=R2, P9=R3, P10=R4, P11=R5

Only 24 distinct forms in row class
```

**Maximum symmetry** (12 forms):
```
Webern Op. 24:
P0 = RI7 (rotation)
I0 = R (something)

P/RI pairs collapse AND I/R pairs collapse

Only 12 distinct forms!
Most concentrated row class possible
```

### From Repertoire

**Lutyens, Motet Op. 27** (standard 48):
```
Row: 0-11-3-7-8-4-2-6-5-1-9-10

No special symmetry
48 distinct forms available
Composer chooses among them
Matrix shows all 48
```

**Webern, Symphonie Op. 21** (24 forms):
```
Row: 9-6-7-8-4-5-11-10-2-1-0-3

Retrograde equivalence:
Playing row backwards = transposed row forwards
P0 reversed = R6 (which = P6)

Implications:
- Can overlap row endings/beginnings
- Canonic writing facilitated
- Symmetry built into material
```

**Using row class in composition**:
```
Typical approach:
Movement/Section 1: P0, I0
Movement/Section 2: P5, I5
Development: Multiple forms
Recapitulation: Return to P0

Row forms can:
- Articulate formal sections
- Combine in counterpoint (combinatoriality)
- Overlap at shared pitches (invariance)
- Create canonic relationships
```

## Related Concepts

- **Prerequisite**: twelve-tone-row, row-operations, transposition
- **Leads to**: row-matrix, row-symmetry, twelve-tone-analysis
- **See also**: invariants, combinatoriality

## Common Confusions

- Row class = all 48 (or fewer) related row forms
- NOT just "any row" but all forms of ONE row
- Standard: 48 forms (4 types x 12 transpositions)
- Symmetry reduces: 24 forms (one symmetry) or 12 forms (both)
- All forms in class share intervallic content
- Matrix displays entire row class on one grid
- P and R are retrogrades of each other (same subscript)
- I and RI are retrogrades of each other (same subscript)
- Different row classes have different properties
- Composer works within ONE row class per piece (usually)
- Some pieces use multiple row classes (rare)
- Row class defines available pitch material
- Fewer distinct forms = more concentrated material

## Source Reference

Open Music Theory, Part IX: "Basics of Twelve-Tone Theory"
