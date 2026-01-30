---
concept: Interval-Class Vector
category: theory
source: Introduction to Post-Tonal Theory (Fifth Edition)
chapter: "Basic Concepts of Pitch and Interval"
chapter_number: 1
pdf_page: 17
unit: null
authors: Joseph N. Straus
---

# Quick Definition
An interval-class vector is a six-digit string summarizing the interval-class content of a pitch-class collection, with each position showing the count of interval classes 1 through 6.

# Formal Definition
An interval-class vector is a summary of the interval-class content of a collection of pitch classes, written as a six-element string of numbers. The first number gives the count of interval class 1, the second gives the count of interval class 2, and so on through interval class 6. The vector is enclosed in angle brackets: <ic1, ic2, ic3, ic4, ic5, ic6>.

# Mathematical Formulation/Recognition
Format: <n1 n2 n3 n4 n5 n6>
- Position 1: count of ic1 (semitones/major 7ths)
- Position 2: count of ic2 (major 2nds/minor 7ths)
- Position 3: count of ic3 (minor 3rds/major 6ths)
- Position 4: count of ic4 (major 3rds/minor 6ths)
- Position 5: count of ic5 (perfect 4ths/5ths)
- Position 6: count of ic6 (tritones)

Calculation method:
1. List all pairs of pitch classes
2. Determine the interval class for each pair
3. Tally occurrences of each interval class

# Musical Context/Application
The interval-class vector provides a compact fingerprint for identifying and comparing sonorities. All members of a set class share the same interval-class vector because transposition and inversion preserve interval-class content. The vector reveals characteristic features of a collection, such as whether it is saturated with certain intervals or lacks others entirely.

# Examples
**Example 1-19** (Schoenberg): {B, G#, G} has vector <101100> (one ic1, one ic3, one ic4)

**Example 1-20** (Stravinsky): The chord has vector <010020> (one ic2, two ic5s)

**Example 1-21** (Varese): The melodic cell has vector <100011> (one ic1, one ic5, one ic6)

**Example 1-22**: The major scale has vector <254361>:
- 2 semitones (ic1)
- 5 whole tones (ic2)
- 4 minor thirds (ic3)
- 3 major thirds (ic4)
- 6 perfect fourths/fifths (ic5)
- 1 tritone (ic6)

This vector uniquely identifies the major scale (only three other collections have all different values).

# Related Concepts
- Interval class
- Interval-class content
- Pitch-class set
- Set class
- Z-relation (sets with same vector but different prime forms)

# Common Confusions
The interval-class vector represents unordered pitch-class intervals (interval classes), not ordered intervals. The vector has exactly six positions, corresponding to the six non-zero interval classes. Interval class 0 (unisons) is typically excluded since it represents a pitch class paired with itself.

# Source Reference
Chapter 1: Basic Concepts of Pitch and Interval, Section 1.12, pp. 16-17
