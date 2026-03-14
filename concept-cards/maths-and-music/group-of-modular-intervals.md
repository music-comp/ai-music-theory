---
concept: Group of Modular Intervals
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
---

# Quick Definition
The group of interval equivalence classes modulo octave, identified with $(\mathbb{R}/{\sim}, +)$, where interval composition is defined by choosing representatives, adding, and taking the equivalence class of the sum.

# Formal Definition
The group of modular intervals is $(\mathbb{R}/{\sim}, +)$, where $\bar{x} + \bar{y} = \overline{x + y}$ (well-defined by an argument analogous to the well-definedness proof for $\mathbb{Z}_m$). Elements are equivalence classes of intervals modulo octave. The identity is $\bar{0}$ (unison class), and the inverse of $\bar{x}$ is $\overline{-x}$ (opposite interval class).

# Mathematical Context
This group sits between the group of all intervals $(\mathbb{R}, +)$ and the group of modular chromatic intervals $(\mathbb{Z}_{12}, +)$. The wrapping homomorphism $w: (\mathbb{R}, +) \to (\mathbb{R}/{\sim}, +)$ is surjective but not injective. The group $\mathbb{Z}_{12}$ (or $\mathbb{Z}_n$) is a subgroup of $\mathbb{R}/{\sim}$ consisting of classes representable by integer multiples of the chromatic unit.

# Musical Context
In this group, interval composition respects octave equivalence: a third + a ninth = a tritone (since a ninth is octave-equivalent to a second, and a second + a third = a tritone, minus the octave). A fourth + a fifth = unison (not an octave, since octave = unison in this group). This captures the musical intuition that interval relationships persist across different octave registers.

# Examples
- Third + ninth = tritone: $\overline{4} + \overline{14} = \overline{18} = \overline{6}$ (in semitone measure, mod 12)
- Fourth + fifth = unison: $\overline{5} + \overline{7} = \overline{12} = \overline{0}$
- This group includes non-chromatic intervals: the just major third (ratio 5/4, $\approx 386.3$ cents) has its own equivalence class distinct from the tempered major third (400 cents)

# Related Concepts
- Group of Intervals
- Modular Chromatic Intervals
- Modular Equivalence on the Real Numbers
- Octave Equivalence Formalized

# Common Confusions
- The group of modular intervals includes ALL intervals modulo octave, not just chromatic ones; it is a continuous group parameterized by the circle
- $\mathbb{Z}_{12}$ is a discrete subgroup of this continuous group, containing only chromatic interval classes
- The identity element is the unison/octave class, not zero in any absolute sense

# Source Reference
Chapter 7, "The Group of Modular Intervals" section, p. 82 (PDF)
