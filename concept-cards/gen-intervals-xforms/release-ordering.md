---
concept: Release-Ordering
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
Release-ordering arranges time spans by when they end (release), not when they begin (attack), reflecting the order in which spans are fully perceived.

# Formal Definition
Given distinct spans s and t, s precedes t in the release-ordering if:
1. s ends before t ends, OR
2. They end simultaneously and s is longer (equivalently, s began earlier)

If spans s and t correspond to musical events event1 and event2, then s precedes t in release-ordering when we perceive event1's full time span before we perceive event2's.

# Mathematical Formulation
For time spans s = (a, x) and t = (b, y):

Release time of s = a + x
Release time of t = b + y

s <_release t if:
- (a + x) < (b + y), OR
- (a + x) = (b + y) AND x > y (i.e., a < b since durations differ)

This defines a total ordering on any finite set of distinct time spans.

Contrast with attack-ordering:
s <_attack t if:
- a < b, OR
- a = b AND x < y

# Musical Context/Application
Release-ordering models perceptual reality: we cannot know a span's duration until it ends. When "unrolling" interval vectors, we need release-ordering to track which spans have been fully perceived at each moment. This differs from score-order (attack-ordering).

# Examples
From Figure 5.13, Y = {vn1, vn2, vc1, va1, vn3, vc2, va2, vn4}

Attack-ordering: vn1, va1, vc1, vn2, vn3, vc2, va2, vn4
(ordered by when each span begins)

Release-ordering: vn1, vn2, vc1, va1, vn3, vc2, va2, vn4
(ordered by when each span ends)

The difference matters: at time 17, va1 and vc1 have attacked but not released. We have "heard" them begin but don't yet know how long they will last. Only vn1 has fully "happened" at that point in release-ordering.

# Related Concepts
- Unrolling Interval Vector
- Attack-Ordered Dyad
- Time-Span GIS
- Perceptual Time

# Common Confusions
Release-ordering is not simply "reverse" attack-ordering. A short note that attacks late can still release early. The ordering depends on both onset time and duration. In ensemble passages with varying note lengths, release-ordering can differ substantially from attack-ordering.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, Example 5.4.2
