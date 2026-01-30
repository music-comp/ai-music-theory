---
concept: Unrolling Interval Vector
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
An unrolling interval vector tracks how a set's interval vector develops over time, showing the progressive accumulation of intervallic relationships as events unfold.

# Formal Definition
Given a set Y of time spans, the unrolling interval vector articulates Y into stages based on release-ordering (when spans end, not when they begin). At each stage, the interval vector is computed for the subset of Y whose spans have fully occurred.

Stage articulation:
1. List Y = (s1, s2, ..., sN) in release-ordering
2. Identify stages at distinct release points
3. At stage k, compute interval vector for Y_k = spans released by that point
4. Track how the vector grows from stage to stage

# Mathematical Formulation
Release-ordering: s precedes t if:
1. s ends before t ends, OR
2. They end simultaneously and s is longer (began earlier)

Stage sequence: Y1 subset Y2 subset ... subset Y = Y_final

At each stage k:
- Interval vector entries = IFUNC(Y_k, Y_k)(i, p) for forwards-oriented (i, p)
- New entries come from dyads involving newly-released spans

Update rule: When span s releases, add to the vector all intervals between s and previously-released spans.

# Musical Context/Application
Unrolling captures how a listener perceives intervallic structure developing in real time. A span is not fully "perceived" until it ends (we don't know its duration until then). This models the evolving sense of rhythmic pattern as a passage unfolds.

# Examples
From Figure 5.13, string trio passage:

Stage 1 (time 18): Y1 = {vn1, vn2}
- First interval: between first two violin notes

Stage 2 (time 18.5): Y2 = {vn1, vn2, vc1}
- Add intervals involving cello span

Stage 3 (time 18.75): Y3 = {vn1, vn2, vc1, va1}
- Add intervals involving viola span

Stage 4 (time 20): Y4 = all of Y
- Add intervals involving final simultaneous releases

Computer visualization: Color dots on half-plane grid at (i, log p) for each forwards-oriented interval (i, p). Color intensity reflects multiplicity.

# Related Concepts
- Release-Ordering
- Time-Span Interval Vector
- IFUNC (Interval Function)
- EMB (Embedding Function)
- Perceptual Rhythm

# Common Confusions
Release-ordering differs from attack-ordering. A span that attacks early may release late (if it's long). The unrolling uses release-ordering because we don't "know" a span's duration until it ends. This subtlety is crucial for modeling perception.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, Example 5.4.2 and Figures 5.13-5.15
