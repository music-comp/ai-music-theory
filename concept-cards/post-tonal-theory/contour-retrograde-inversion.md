---
concept: Contour Retrograde-Inversion
category: theory
source: Introduction to Post-Tonal Theory (Fifth Edition)
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 144
unit: null
authors: Joseph N. Straus
---

# Quick Definition
A contour transformation combining retrograde and inversion; the CSEG is both reversed in order and complemented in value, producing a shape that is the mirror image played backward.

# Formal Definition
Contour retrograde-inversion (often notated RI) applies both contour inversion and contour retrograde to a CSEG. The order of applying these operations does not matter: R(I(CSEG)) = I(R(CSEG)).

For a CSEG <a, b, c, d> with n elements:
- First invert: <(n-1)-a, (n-1)-b, (n-1)-c, (n-1)-d>
- Then retrograde: <(n-1)-d, (n-1)-c, (n-1)-b, (n-1)-a>

Or equivalently:
- First retrograde: <d, c, b, a>
- Then invert: <(n-1)-d, (n-1)-c, (n-1)-b, (n-1)-a>

# Mathematical Formulation/Recognition
For CSEG <x1, x2, ..., xn> where max value is m = n-1:

RI = <m-xn, m-x(n-1), ..., m-x2, m-x1>

To compute:
1. Reverse the order
2. Complement each value with respect to (n-1)

Or:
1. Complement each value
2. Reverse the order

Example: <2013> with n=4 (m=3)
- Method 1: Invert first -> <1320>, then retrograde -> <0231>
- Method 2: Retrograde first -> <3102>, then invert -> <0231>
- RI(<2013>) = <0231>

# Musical Context/Application
Contour retrograde-inversion:
- Combines directional mirroring with temporal reversal
- Represents the most distant transformation within a CSEG-class
- Appears in music with complex motivic transformations
- Is important for identifying complete CSEG-class membership

When a CSEG and its RI appear in music, they share neither the same directional pattern (like I) nor the same temporal unfolding (like R), yet they belong to the same contour equivalence class.

# Examples
From Example 3-37: Crawford Seeger, String Quartet

The four members of one CSEG-class:
- Original: <2013>
- Inversion: <1320>
- Retrograde: <3102>
- Retrograde-Inversion: <0231>

<0231> is the prime form of this CSEG-class (starts on 0).

Relationships:
- <2013> and <0231> are RI-related
- <1320> and <3102> are RI-related
- The "opposites" in the group are always RI-related

In the Crawford melody, <2013> and <0231> both appear, showing use of RI-related contours from the same CSEG-class.

# Related Concepts
- Contour segment (CSEG)
- CSEG-class
- Contour inversion
- Contour retrograde
- Retrograde-inversion (in twelve-tone music)

# Common Confusions
- Thinking RI is a completely different operation (it's just R and I combined)
- Not recognizing that R and I commute (order doesn't matter)
- Confusing contour RI with pitch-class RI (different domains)
- Forgetting that RI is the "most distant" relative within a CSEG-class

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.10.2, pages 144-145
