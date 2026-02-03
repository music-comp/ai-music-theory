---
concept: Row Symmetry
category: theory
source: Open Music Theory
chapter: "Row Properties"
part: 9
---

# Row Symmetry

## Quick Definition

Row symmetry occurs when a twelve-tone row maps onto itself (or a transposition of itself) under one of the serial operations—such as retrograde-equivalent rows (where R equals a transposed P) or inversion-equivalent rows (where I equals a transposed P)—reducing the total number of distinct row forms from 48 to 24 or even 12, creating special structural properties that composers like Webern exploited to build compositions around symmetrical pitch relationships.

## Formal Definition

**Row symmetry**: A property where transformations produce equivalent row forms.

**Types of symmetry**:

1. **Retrograde equivalence**: P = transposed R
   ```
   If P0 = Rn for some n:
   Only 24 distinct forms (P/R pairs collapse)
   Row has palindromic interval structure
   ```

2. **Inversion equivalence**: P = transposed I
   ```
   If P0 = In for some n:
   Only 24 distinct forms (P/I pairs collapse)
   Row has inversionally symmetrical interval structure
   ```

3. **Multiple symmetries**: Both R and I equivalence
   ```
   If P0 = Rn AND P0 = Im:
   Only 12 distinct forms
   Maximum symmetry
   ```

**Standard row**: 48 distinct forms (4 types x 12 transpositions)
**Single symmetry**: 24 distinct forms
**Double symmetry**: 12 distinct forms

## Musical Context

Row symmetry provides:
- **Economy**: Fewer distinct forms to manage
- **Coherence**: Transformations relate more closely
- **Structure**: Symmetry can articulate form
- **Unity**: Different operations produce same material

**Webern's preference**: Strongly attracted to symmetrical rows
- Op. 21: Retrograde equivalent (24 forms)
- Op. 24: P=RI equivalent, I=R equivalent (12 forms)

**Compositional implications**:
- Overlapping row forms possible
- Canonic writing facilitated
- Symmetry audible in careful handling
- Form often reflects row symmetry

## Examples

### Basic

**Retrograde equivalence**:
```
Row with intervallic palindrome:
+2, +5, -3, +1, -1, +3, -5, -2 (intervals)
       ←  mirror point  →

Forward and backward give same intervals
Therefore P0 = Rn for some n

Result: 24 distinct row forms
P and R pairs are "the same" row
```

**Testing for symmetry**:
```
Given P0, check if:
1. P0 reversed = P0 transposed? → Retrograde equivalent
2. P0 inverted = P0 transposed? → Inversion equivalent

Example:
P0: 0-1-4-5-9-10-3-2-6-7-11-8
Reversed: 8-11-7-6-2-3-10-9-5-4-1-0
Is reversed = Tn(P0)? Check intervals...
```

**Symmetric interval succession**:
```
For R-equivalence, intervals must be:
a, b, c, d, e, X, e, d, c, b, a
(palindromic around center)

For I-equivalence:
Row must map onto its own inversion
Requires specific interval relationships
```

### From Repertoire

**Webern, Symphonie Op. 21**:
```
P0: 9-6-7-8-4-5-11-10-2-1-0-3

Intervals: -3,+1,+1,-4,+1,+6,-1,-8,-1,-1,+3

Is this R-equivalent?
R0 should end on 9 (P0's first note)
Check: P0 reversed starting from note that ends on 9

Result: P0 = R6 (transposition)
Only 24 distinct forms

Webern exploits this:
- Overlaps ends of row forms
- Creates canonic structures
- Symmetry reflected in form
```

**Webern, Konzert Op. 24**:
```
P0: 11-8-2-3-7-6-8-4-5-0-1-9

Four trichords, each (014):
Cell 1: P version of (014)
Cell 2: RI version of (014)
Cell 3: R version of (014)
Cell 4: I version of (014)

Symmetry at trichord level:
P0 = RI7 (when rotated)
I0 = R (something)

Result: Only 12 distinct row forms!
Maximum symmetry for a derived row
```

**Implications for Op. 24**:
```
With only 12 forms:
- Every transposition has multiple labels
- P and RI are "the same"
- I and R are "the same"
- Extraordinary concentration of material

Webern structures piece around this:
- Trichords as building blocks
- Symmetry pervades all levels
- Sator Square analogy
```

## Related Concepts

- **Prerequisite**: twelve-tone-row, row-operations, row-matrix
- **Leads to**: twelve-tone-analysis, webern-op7-analysis
- **See also**: derived-row, invariants

## Common Confusions

- Symmetry = row maps onto transposed version of itself under transformation
- Retrograde equivalence: P = transposed R (palindromic intervals)
- Inversion equivalence: P = transposed I
- Standard row: 48 distinct forms
- Single symmetry: 24 forms (pairs collapse)
- Double symmetry: 12 forms (quadruples collapse)
- Symmetry is about ROW FORMS, not just intervals
- Not all rows have symmetry (most don't)
- Webern particularly sought symmetric rows
- Symmetry enables overlapping row forms
- Symmetry can be exploited in canonic writing
- Row symmetry often reflected in formal symmetry
- Different from "all-interval" or "derived" (separate properties)
- A row can be symmetric AND derived AND combinatorial

## Source Reference

Open Music Theory, Part IX: "Row Properties"
Open Music Theory, Part IX: "Analysis Examples - Webern Op. 21 and 24"
