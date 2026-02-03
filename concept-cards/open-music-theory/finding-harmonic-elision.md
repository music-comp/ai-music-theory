---
concept: Finding Harmonic Elision
category: harmony
source: Open Music Theory
chapter: "Harmonic Elision"
part: 5
---

# Finding Harmonic Elision

## Quick Definition
Finding harmonic elision in analysis involves looking for incongruities in Roman numeral sequences where an expected resolution chord appears to be skipped, particularly when a dominant-function chord is followed by another dominant-function chord targeting a different key, indicating that the expected resolution was suppressed and replaced.

## Formal Definition
**Finding harmonic elision** requires:

**Roman numeral incongruities**: Unexpected sequences in analysis

**Expected resolution missing**: The anticipated chord doesn't appear

**Dominant-after-dominant**: V followed by another V/X

**Voice leading trace**: Following the leading tone or root motion

**Pattern recognition**: Identifying elision types (leading-tone or raised-root)

## Roman Numeral Clues

**Standard resolutions**:
```
V -> I (authentic cadence)
V6/5 -> I (dominant seventh to tonic)
V4/3 -> I6 (passing motion)
viio7 -> I (diminished seventh resolves)
V7/X -> X (applied dominant resolves)
```

**Elision indicators**:
```
V6/5 -> V4/2/IV (expected I, got V/IV instead)
V7 -> V7/ii (expected I, got V/ii instead)
viio7 -> V7/X (expected I, reinterpreted)
V/vi -> V/V (expected vi, got V/V instead)
```

**The pattern**: V of something -> V of something else
- Expected target doesn't appear
- Another dominant takes its place

## Step-by-Step Analysis

**Step 1: Identify dominant function**:
```
Find V, V7, viio, viio7, or any applied dominant
These chords "want" to resolve to their target
```

**Step 2: Check resolution**:
```
Does the expected target appear?
V7 in C -> should go to C major (I)
V7/ii -> should go to D minor (ii)
```

**Step 3: Note the incongruity**:
```
If expected target doesn't appear:
What came instead?

V7 in C -> expected I, got V7/IV?
This is elision!
```

**Step 4: Classify the elision**:
```
Leading-tone elision: Leading tone went DOWN
   V6/5 -> V4/2/IV (B -> Bb)

Raised-root elision: Root went UP
   V -> V/ii (expected C -> actual C#)
```

## Voice Leading Traces

**Following the leading tone**:
```
Expected: B -> C (leading tone to tonic)
Actual:   B -> Bb (leading-tone elision)

The leading tone moved DOWN instead of UP
This confirms leading-tone elision
```

**Following the expected root**:
```
Expected: Root C appears (tonic)
Actual:   C# appears (raised root)

The expected root was RAISED
This confirms raised-root elision
```

## Common Elision Patterns

**Descending sequence of dominants**:
```
V/vi -> V/V -> V/IV -> V -> I

Expected: vi after V/vi
Actual:   V/V (elision!)

Expected: V after V/V
Actual:   V/IV (elision!)

Each dominant's target is elided
Pattern moves DOWN through dominants
```

**Ascending sequence**:
```
I -> V/ii -> V/iii -> V/IV -> ...

Expected: ii after V/ii
Actual:   V/iii (elision!)

Each time, expected target is replaced
Pattern moves UP through dominants
```

**Mixed patterns**:
```
V -> V/IV -> IV -> V/vi -> vi -> ...

Some resolutions complete
Others are elided
Creates varied harmonic rhythm
```

## Analysis Workflow

**Practical steps**:
```
1. Write Roman numerals for each chord
2. Check: Does each V resolve to expected target?
3. Flag: V followed by another V
4. Trace: What happened to the leading tone?
5. Trace: Was the expected root raised?
6. Label: Leading-tone or raised-root elision
```

**Notation options**:
```
V6/5 -> [I] -> V4/2/IV
        ↑
    Elided chord shown in brackets

Or: V6/5 -> V4/2/IV (LT elision)
    Mark elision type in parentheses
```

## What Elision Is NOT

**Not a deceptive cadence**:
```
Deceptive: V -> vi (V resolves to unexpected but functional chord)
Elision:   V -> V/X (V replaced by another dominant)

Deceptive = unexpected resolution
Elision = suppression and replacement
```

**Not simple tonicization**:
```
Tonicization: I -> V/V -> V (applied chord resolves)
Elision:      V -> V/X -> ... (expected resolution skipped)

Tonicization leads TO a target
Elision skips PAST a target
```

**Not pivot chord modulation**:
```
Pivot: Chord functions in two keys (reinterpreted)
Elision: Expected chord is suppressed (skipped)

Pivot = double function
Elision = missing function
```

## Musical Context

Finding harmonic elision helps analysts:
- **Understand complexity**: Why progression seems to "skip"
- **Follow chromatic logic**: How chromaticism extends tonal motion
- **Recognize patterns**: Elision sequences are common
- **Appreciate drama**: Elision creates harmonic surprise
- **Connect to style**: Romantic harmony uses elision extensively

## Examples

### Basic

**Identifying leading-tone elision**:
```
Given progression in C major:
G7 - C7 - F - G - C
V7   ?    IV  V   I

Analysis:
V7 (G7) expected to go to I (C major)
But C7 appears (V7/IV, not I!)

Check leading tone: B in G7
Where did B go? To Bb (in C7)
B -> Bb = leading tone DOWN

Confirmed: Leading-tone elision
Label: V7 -> V7/IV (LT elision) -> IV -> V -> I
```

**Identifying raised-root elision**:
```
Given progression in C major:
G7 - A7 - Dm - G7 - C
V7   ?    ii   V7   I

Analysis:
V7 (G7) expected to go to I (C major)
But A7 appears (V7/ii, not I!)

Check expected root: C (tonic)
What appeared? C# (in A7)
Expected C -> actual C# = raised root

Confirmed: Raised-root elision
Label: V7 -> V7/ii (RR elision) -> ii -> V7 -> I
```

### From Repertoire

**Beethoven, Development Sections**: Multiple elisions create harmonic ambiguity.

**Schubert, "Der Wegweiser"**: Leading-tone elisions for text painting (wandering, lost).

**Wagner, Tristan Prelude**: Extensive elision patterns create endless melody.

**Brahms, Symphonies**: Elisions in development create large-scale harmonic motion.

## Related Concepts

- **Prerequisite**: harmonic-elision, leading-tone-elision, raised-root-elision
- **Leads to**: development-section-analysis-challenges
- **See also**: roman-numeral-analysis, secondary-dominant, harmonic-sequence

## Common Confusions

- Finding elision requires noting Roman numeral incongruities
- Look for: Dominant followed by another dominant
- Check: Did expected target appear?
- V -> I expected, V -> V/X found = elision
- **Two traces**:
  - Leading tone: Did it go down? (leading-tone elision)
  - Root: Was it raised? (raised-root elision)
- Common patterns: Descending dominant sequence (V/vi -> V/V -> V/IV -> V)
- Different from deceptive cadence (which is unexpected resolution, not suppression)
- Different from tonicization (which completes, not skips)
- Different from pivot modulation (which reinterprets, not suppresses)
- Label in analysis with brackets [elided chord] or annotation (LT/RR elision)
- Romantic harmony uses elision extensively
- Understanding elision explains "why" progression skips expected steps

## Source Reference

Open Music Theory, Part V: "Chromaticism"
Open Music Theory, Part V: "Harmonic Elision - Finding Harmonic Elision"
