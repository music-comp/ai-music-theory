---
concept: Corpus Analysis Methods
category: technique
source: "Tonality: An Owner's Manual"
chapter: "Implicit Musical Knowledge"
chapter_number: 1
pdf_page: 1
authors: "Dmitri Tymoczko"
unit: null
---

# Quick Definition
Computational study of large collections of musical scores to reveal patterns, tendencies, and regularities that are invisible to conventional analytical methods.

# Formal Definition
Corpus analysis uses computers to systematically examine large bodies of musical works, providing a "ground truth" against which traditional theoretical claims can be evaluated. Tymoczko creates machine-readable Roman-numeral analyses of more than one thousand pieces stretching from Dufay to Brahms. These handmade annotations greatly increase the power of computational analysis but at the cost of introducing subjectivity. Corpus data allow reconstruction of the dialogue between explicit "theorist theory" and implicit "composer theory."

# Musical Context
Computational corpus studies stand alongside geometry as an important twenty-first-century addition to the music theorist's toolbox. They were cultivated by David Huron and brought to the masses by Michael Cuthbert (music21). Tymoczko adopted the practice around 2000, first using MIDI files and Max/MSP, then using Python, music21, and actual scores. The method extends our conception of "music theory" beyond written treatises to theories implicitly encoded in musical works themselves.

# Examples
- Von Hippel and Huron's investigation of post-skip reversal, limited by small sample (35 Schubert lieder + 176 folk songs); Tymoczko's larger analysis of Palestrina's mass movements reveals substantially greater tendency for leaps to change direction (Figure 1.4.1)
- Frequency of root progressions between major chords in the Rolling Stone "500 Greatest Songs" list confirms model predictions about rock harmony (Figure 2.2.4)
- Data showing the percentage of triads above ^3 that are in first inversion, tracking the gradual emergence of functional harmony (Figure 1.5.4)

# Related Concepts
- Implicit Musical Knowledge
- Schema Theory
- Dogmatic Musical Conventions
- Weak vs. Strong Root Progressions

# Common Confusions
- Musical scores are "for many computational purposes opaque" -- D-F-A can be tonic in D minor, supertonic in C major, or a nonharmonic agglomeration
- Simple counting will not necessarily answer musical questions -- concepts like "voice" and "step" can be difficult to pin down
- Corpus analysis provides "limited and defeasible evidence" -- it undercuts extreme skepticism but also challenges textbook verities

# Source Reference
Chapter 1, Section 4, pp. 22-26. Tymoczko's corpus is freely available on the internet (appendix 4), including music, data, code, and instructions for reproducing all graphs and statistics.
