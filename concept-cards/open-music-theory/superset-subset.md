---
concept: Superset and Subset
category: theory
source: Open Music Theory
chapter: "Analyzing with Set Theory (or not!)"
pdf_page: null
chapter_number: 8
unit: null
authors: "Open Music Theory contributors"
---

# Superset and Subset

## Quick Definition

A subset is a pitch-class set entirely contained within a larger set (the superset). For example, a major triad {0, 4, 7} is a subset of the C major scale {0, 2, 4, 5, 7, 9, 11}, and the scale is a superset of the triad. Finding superset relationships can reveal how small motivic sets relate to larger organizing collections.

## Formal Definition

**Subset**: Set A is a subset of set B if every element of A is also an element of B.
- Notation: A is a subset of B
- All members of A are in B, but B may have additional members

**Superset**: Set B is a superset of set A if B contains all elements of A.
- Notation: B is a superset of A
- Equivalently: A is a subset of B

**Proper subset/superset**: A is a proper subset of B if A is a subset of B and A does not equal B (B has at least one element not in A).

**Abstract subset/superset**: Concerns set classes rather than specific sets. Set class X is an abstract subset of set class Y if some member of X is a literal subset of some member of Y.

**Common analytical relationships**:
- Trichords as subsets of hexachords
- Tetrachords as subsets of octatonic
- Modal fragments as subsets of diatonic
- Motivic cells as subsets of organizing collections

## Musical Context

Superset/subset relationships reveal hierarchies of pitch organization:

1. **Collection containment**: Small motives may be subsets of larger governing collections
2. **Motivic derivation**: Larger sets built by combining smaller subsets
3. **Harmonic membership**: Chords as subsets of scales
4. **Analytical unity**: Relating disparate passages through shared superset

**Analytical strategy**: If different pcsets in a piece all belong to the same superset (like the octatonic collection), this reveals an organizing principle even when the surface is varied.

**Compositional strategy**: Starting with a collection (superset) and deriving motives (subsets) ensures material will be related.

## Examples

### Basic

**Literal subset relationship**:
```
Set A: {0, 4, 7} (C major triad)
Set B: {0, 2, 4, 5, 7, 9, 11} (C major scale)

Is A a subset of B?
  0 in B? Yes
  4 in B? Yes
  7 in B? Yes

Yes, {0, 4, 7} is a subset of {0, 2, 4, 5, 7, 9, 11}
The C major triad is contained in the C major scale.
```

**Octatonic as superset**:
```
OCT0,1: {0, 1, 3, 4, 6, 7, 9, 10}

Subsets found in a Bartok passage:
  {0, 1, 6, 7} - is it a subset of OCT0,1? YES
  {3, 4, 9, 10} - is it a subset? YES

Both tetrachords are subsets of the same octatonic collection,
revealing the organizing principle.
```

**Abstract subset (set class level)**:
```
Set class (037) - major/minor triad
Set class (013568T) - diatonic collection

Is (037) an abstract subset of (013568T)?
  Find a member of (037) contained in a member of (013568T):
  {0, 4, 7} is contained in {0, 2, 4, 5, 7, 9, 11}? YES

(037) is an abstract subset of the diatonic set class.
```

### From Repertoire

**Bartok, "From the Island of Bali"**: The (0167) tetrachords in each hand are subsets of the octatonic collection. Recognizing this superset relationship reveals the underlying organization.

**Stravinsky, Rite of Spring**: Various surface harmonies are subsets of octatonic or diatonic collections, and tracking these relationships shows the harmonic logic beneath the complex surface.

**Webern**: Often builds hexachords from trichordal subsets, making the subset/superset relationship a compositional principle.

**Modal jazz**: Scales (supersets) generate available chord voicings (subsets), making the relationship explicit in improvisational practice.

## Related Concepts

- **Prerequisite**: pitch-class-set, set-class
- **Leads to**: collection-analysis, inclusion-lattice
- **See also**: aggregate, segmentation

## Common Confusions

- **Subset =/= smaller set class number**: The terms refer to containment, not cardinality comparison
- **Abstract vs. literal**: Literal = specific sets; abstract = set classes
- **Every set is a subset of itself**: For proper subset, the sets must be different
- **The empty set is a subset of everything**: By definition (but not analytically useful)
- **Containment is not equivalence**: A subset shares notes with its superset but may have very different character
- **Finding a superset is analytical interpretation**: It's a claim about the music, not automatic fact

## Source Reference

Open Music Theory, Part VIII, Chapter 6: "Analyzing with Set Theory (or not!)"
