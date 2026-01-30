---
concept: Common Tones under Transposition
category: theory
source: Introduction to Post-Tonal Theory (Fifth Edition)
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 111
unit: null
authors: Joseph N. Straus
---

# Quick Definition
The pitch classes that remain unchanged when a set is transposed; the number of common tones at any transposition level Tn equals the number of occurrences of interval-class n within the set.

# Formal Definition
When a pitch-class set is transposed at Tn, common tones are those pitch classes that appear in both the original set and the transposed set. The number of common tones is determined by the interval-class content of the set: for each occurrence of interval n within the set, there will be one common tone under Tn. This is because when a set is transposed at Tn, each member maps onto a note n semitones higher; if two notes in the set were already n semitones apart, transposing by n maps one onto the other.

The exception is the tritone (interval-class 6): because the tritone maps onto itself under T6, each occurrence of ic6 produces two common tones, not one.

# Mathematical Formulation/Recognition
For a set S transposed at Tn:
- Count occurrences of interval-class n in S (from the interval-class vector)
- Number of common tones = number of occurrences of ic n
- Exception: At T6, number of common tones = 2 times the number of tritones

For T0: All notes are common tones (the set maps onto itself)

To identify which specific pitch classes are common tones: Find pairs of notes in S that are n semitones apart. The higher note of each pair becomes a common tone at Tn; the lower note becomes a common tone at T(12-n).

# Musical Context/Application
Common tones under transposition provide important musical continuity between different statements of the same set class. Composers often exploit common tones to create smooth connections between transpositions, or conversely, avoid transposition levels that produce common tones to emphasize contrast.

This property is crucial for understanding:
- Key relationships in tonal music (the major scale's unique multiplicity of interval class)
- Voice-leading connections in post-tonal music
- The "limited transposition" quality of certain symmetrical sets

# Examples
From Example 3-2 in the text: [4, 5, 7, 8], a member of sc(0134), contains two occurrences of ic3 (between 4-7 and 5-8). At T3: 4 maps to 7, 5 maps to 8, producing common tones 7 and 8. At T9: 8 maps to 5, 7 maps to 4, producing common tones 4 and 5.

From Example 3-4: Stravinsky, Rite of Spring - A chain of (0134)s linked by common tones. T4 produces one common tone; T11 and T3 each produce two common tones.

The major scale [254361] has six ic5s, so T7 (up a fifth) retains six of seven pitch classes, explaining why the dominant is closely related to the tonic.

# Related Concepts
- Interval-class vector
- Transposition (Tn)
- Transpositional symmetry
- Common tones under inversion
- Set class

# Common Confusions
- Forgetting the tritone exception: ic6 produces twice as many common tones as expected at T6
- Confusing common pitch classes with common pitches (register matters for pitch, not for pitch class)
- The interval-class vector counts unordered intervals, so ic5 and ic7 are the same; T5 and T7 produce the same number of common tones

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.1, pages 111-116
