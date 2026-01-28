---
concept: Cube Dance
category: harmony
source: Open Music Theory
chapter: "Neo-Riemannian Triadic Progressions"
part: 5
---

# Cube Dance

## Quick Definition

A diagram originally conceived by Jack Douthett and Peter Steinbach (1998) that represents all 24 major and minor triads as vertices on four interconnected cubes, with each cube representing a PL cycle (hexatonic system) and augmented triads serving as shared vertices that connect adjacent cubes, allowing visualization of how to "modulate" between PL cycles and providing a comprehensive map of triadic voice-leading space where adjacent vertices differ by a single half-step in one voice.

## Formal Definition

**Cube Dance** structure:

**Four cubes** (one per PL cycle/hexatonic system):
```
Cube 1: C, Cm, Ab, Abm, E, Em + C+ and Eb+
Cube 2: Db, Dbm, A, Am, F, Fm + Db+ and E+
Cube 3: D, Dm, Bb, Bbm, Gb, Gbm + D+ and F+
Cube 4: Eb, Ebm, B, Bm, G, Gm + Eb+ and Gb+
```

**Each cube contains**:
- Six triads from one PL cycle (3 major, 3 minor)
- Two augmented triads at opposite corners
- Edges = single half-step voice movement

**Augmented triads as connections**:
- Each augmented triad appears in TWO cubes
- Augmented triads connect adjacent PL cycles
- "Modulation" between cycles via augmented chord

**Total structure**:
- 24 major/minor triads + 4 augmented triads = 28 vertices
- Adjacent vertices differ by one half step in one voice
- Parsimonious voice-leading throughout

## Cube Structure Detail

**Single cube (PL cycle starting on C)**:
```
        C+ ─────────── Cm
       /|             /|
      / |            / |
     /  |           /  |
    E ──+──────── Em   |
    |   |          |   |
    |   Ab ────────+── Abm
    |  /           |  /
    | /            | /
    |/             |/
   Eb+ ─────────── C

(Note: This is a flattened representation of a 3D cube)

Vertices: C, Cm, E, Em, Ab, Abm, C+, Eb+
Edges: Each represents one half-step voice motion

P transformations: vertical edges (C↔Cm, E↔Em, Ab↔Abm)
L transformations: horizontal edges within faces
Augmented triads: corner positions, connect to 3 triads each
```

**Connections between cubes**:
```
C+ appears in:
- Cube 1 (C-Cm-E-Em-Ab-Abm cycle)
- Adjacent cube

Eb+ appears in:
- Cube 1
- Cube 4 (Eb-Ebm-B-Bm-G-Gm cycle)

To "modulate" from one PL cycle to another:
1. Navigate to an augmented triad
2. That augmented triad is shared with another cube
3. Continue in the new cube (new PL cycle)
```

## Modulating Between PL Cycles

**Example: C major to D major**:
```
C major is in Cube 1 (with Ab and E major)
D major is in Cube 3 (with Bb and Gb major)

These cubes don't share an augmented triad directly!

Path from C to D requires TWO modulations:
C → ... → C+ → ... → [adjacent cube] → ... → D+ → ... → D

Or: C → ... → Eb+ → ... → [Cube 4] → ... → F+ → ... → [Cube 3] → D

Multiple paths possible, but must pass through 2 augmented triads
```

**Shortest paths**:
```
Within same cube: up to 4 steps (cube geometry)
Between adjacent cubes: pass through 1 shared augmented triad
Between non-adjacent cubes: pass through 2 augmented triads
```

## Voice-Leading Proximity

**Cube Dance shows proximity**:
- Adjacent vertices = minimal voice-leading (1 half step)
- Close on diagram = close in voice-leading space
- Far on diagram = more voice-leading work

**No tonal center**:
- Diagram doesn't privilege any key
- All triads equally connected to neighbors
- Great for non-functional analysis

**Comparison to Tonnetz**:
- Both show voice-leading proximity
- Cube Dance includes augmented triads explicitly
- Cube Dance better shows PL cycle structure
- Tonnetz better shows all P, L, R relationships uniformly

## Musical Context

The Cube Dance serves specific purposes:
- **Visualization**: All triads in voice-leading space
- **Composition tool**: Generate smooth progressions
- **Analysis**: Map non-functional progressions
- **Cycle modulation**: Show how to move between hexatonic systems
- **Film music**: Visualize supernatural/magical harmonic worlds
- **Neo-Riemannian pedagogy**: Comprehensive diagram of the theory
- **19th/20th-century analysis**: Brahms, Wagner, Mahler, film scores

## Examples

### Basic

**Navigating one cube (C major's PL cycle)**:
```
Within the C/E/Ab hexatonic system:

C ─P─ Cm ─L─ Ab ─P─ Abm ─L─ E ─P─ Em ─L─ C

All six triads + two augmented triads form a cube
Every edge = one half step movement in one voice
```

**Modulating via augmented triad**:
```
Starting in C major's cube:
C → Cm → Ab → Ab+ (now at corner of cube)

Ab+ is ALSO in an adjacent cube!
Ab+ = C+ = E+ (same chord, different spellings)

From Ab+, can move to triads in the ADJACENT cube:
Ab+ → F → Fm → Db → Dbm → A → Am → (back to Ab+)

The augmented triad is the "portal" between hexatonic worlds
```

**Path from C to remote D**:
```
C → Em (L) → E (P) → [within cube 1]
E → E+ (via half step from E)
E+ is shared with cube 2 (containing A, F, Db)
E+ → A (enter cube 2)
A → Am (P) → F (L) → F+ (augmented, corner)
F+ is shared with cube 3 (containing D, Bb, Gb)
F+ → D (enter cube 3)

Path: C → Em → E → E+ → A → Am → F → F+ → D
Uses two augmented "portals" to traverse from cube 1 to cube 3
```

### From Repertoire

**Brahms, late works**: Progressions that traverse multiple cubes in the Cube Dance, using augmented triads as pivots.

**Film music (John Williams)**: Magical or supernatural scenes often use complete PL cycles (one cube) or modulate between cubes via augmented chords.

**Minimalist music**: Single-cube progressions create the characteristic sound of cycling through a hexatonic system.

**Wagner, Parsifal**: Cube Dance analysis reveals the underlying logic of seemingly "irrational" chromatic progressions.

## Related Concepts

- **Prerequisite**: neo-riemannian-cycles, plr-transformations, weitzmann-region, augmented-triad
- **Leads to**: hexatonic-system, voice-leading-geometry, transformational-analysis
- **See also**: tonnetz, neo-riemannian-theory, parsimonious-voice-leading

## Common Confusions

- Cube Dance = diagram showing all 24 triads + 4 augmented triads
- Created by Douthett and Steinbach (1998)
- Four cubes, one per PL cycle (hexatonic system)
- Each cube has 6 triads (3 major, 3 minor) + 2 augmented
- Edges = single half-step movement in one voice
- Augmented triads connect adjacent cubes
- To move between PL cycles, go through an augmented triad
- Some cube pairs share an augmented triad (adjacent)
- Other pairs require TWO augmented triads to traverse
- Better than Tonnetz for showing PL cycle structure
- Tonnetz better for showing all P, L, R uniformly
- Great for composition and analysis of non-functional progressions
- Shows voice-leading proximity without reference to key
- Four hexatonic systems = four cubes = four augmented triads

## Source Reference

Open Music Theory, Part V: "Chromaticism"
Open Music Theory, Part V: "Neo-Riemannian Triadic Progressions"
Douthett, Jack, and Peter Steinbach. 1998. "Parsimonious Graphs." Journal of Music Theory 42, no. 2: 241-63.
