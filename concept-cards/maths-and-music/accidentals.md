---
concept: Accidentals
category: theory
source: "Mathematics and Music"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
authors: "David Wright"
unit: null
---

# Quick Definition
Symbols (sharp, flat, natural, double sharp, double flat) placed before notes to raise or lower their pitch by one or two semitones, creating chromatic alterations.

# Formal Definition
An accidental is a symbol that alters the pitch of a note:
- Sharp ($\sharp$): raises pitch by one semitone
- Flat ($\flat$): lowers pitch by one semitone
- Natural ($\natural$): cancels a previous sharp or flat
- Double sharp ($\times$): raises pitch by two semitones
- Double flat ($\flat\flat$): lowers pitch by two semitones

The altered note class is denoted with the accidental as a superscript: $D^\sharp$, $A^\flat$.

# Mathematical Context
Accidentals define a function on the set of note classes: the sharp operation is a function $\sharp: \text{Notes} \to \text{Notes}$ that shifts each note up by one semitone, and flat is the inverse operation. In modular arithmetic terms, $\sharp$ adds 1 (mod 12) and $\flat$ subtracts 1 (mod 12). Double sharp adds 2, double flat subtracts 2.

# Musical Context
When two different note names produce the same pitch (e.g., $F^\sharp$ and $G^\flat$), they are said to be enharmonically equivalent. This gives rise to an equivalence relation on notes. Accidentals within a measure apply to all subsequent notes of the same note class within that measure, unless cancelled. When an altered note is tied across a bar line, the alteration applies to the tied note but not to other notes of the same class in the new measure. Cautionary accidentals (enclosed in parentheses) are redundant reminders for clarity.

# Examples
- $F^\sharp$ is the same pitch as $G^\flat$ (enharmonic equivalence)
- $C^\flat_5$ is the same note as $B_4$
- $B^\sharp_3$ coincides with $C_4$
- A $\natural$ cancels a key signature's sharp or flat for the remainder of the measure

# Related Concepts
- Enharmonic Equivalence
- Note Classes
- Key Signatures and the Circle of Fifths
- Diatonic and Chromatic Scales
- Musical Intervals

# Common Confusions
- An accidental applies to all notes of the same note class for the rest of the measure, not just the single note it precedes
- Tied notes carry the accidental across bar lines, but other notes of the same class in the new measure are not affected
- $\sharp\hat{3} = \hat{4}$ in many keys -- raising a diatonic note by a semitone can land on another diatonic note

# Source Reference
Chapter 1, "Accidentals" section, p. 21 (PDF); see also Chapter 2, "Rules about accidentals" section, p. 37 (PDF)
