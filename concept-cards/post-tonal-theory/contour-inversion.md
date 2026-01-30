---
concept: Contour Inversion
category: theory
source: Introduction to Post-Tonal Theory (Fifth Edition)
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 144
unit: null
authors: Joseph N. Straus
---

# Quick Definition
A contour transformation that replaces each contour element with its complement relative to the maximum value; the highest note becomes the lowest, the lowest becomes the highest, creating a mirror image of the original contour shape.

# Formal Definition
Contour inversion (often notated INV or I) transforms a CSEG by replacing each integer x with (n-1) - x, where n is the number of distinct pitch levels in the CSEG. This operation exchanges the highest and lowest positions: what was highest (n-1) becomes lowest (0), and vice versa.

Contour-inverted CSEGs are mirror images of each other in terms of up-down motion. The two CSEGs will have corresponding elements that sum to (n-1).

# Mathematical Formulation/Recognition
For a CSEG with n distinct pitch levels:
- INV(x) = (n-1) - x for each element x

For CSEG <a, b, c, d> where max value is 3:
- INV = <3-a, 3-b, 3-c, 3-d>

To verify contour inversion:
- Add corresponding elements of original and inverted CSEG
- All sums should equal (n-1)

Example: <2013> with n=4 (max=3)
- INV(<2013>) = <3-2, 3-0, 3-1, 3-3> = <1320>
- Check: 2+1=3, 0+3=3, 1+2=3, 3+0=3 (all equal n-1)

# Musical Context/Application
Contour inversion is useful for:
- Identifying melodic mirror images
- Analyzing thematic variation based on contour
- Understanding relationships in music where pitch is indeterminate but direction matters
- Creating systematic melodic transformations

Unlike pitch-class inversion (which maps around a fixed axis), contour inversion simply flips the up-down pattern, making high notes low and low notes high.

# Examples
From Example 3-37: Crawford Seeger, String Quartet

Original CSEG: <2013>
- Pattern: second-highest, lowest, second-lowest, highest
- Direction: down (2 to 0), up (0 to 1), up (1 to 3)

Contour inversion: <1320>
- Pattern: second-lowest, highest, second-highest, lowest
- Direction: up (1 to 3), down (3 to 2), down (2 to 0)

The inverted contour is a mirror image: where the original goes down, the inversion goes up.

From Example 3-39: Stockhausen, Klavierstuck II
- Upper melody: CSEG <3201>
- Lower melody: CSEG <0132>
- These are contour inversions of each other
- The dynamics of the upper melody also follow <3201>

# Related Concepts
- Contour segment (CSEG)
- CSEG-class
- Contour retrograde
- Contour retrograde-inversion
- Inversion (In) for pitch classes

# Common Confusions
- Confusing contour inversion with pitch-class inversion (different operations)
- Thinking contour inversion changes the order (it doesn't; only the values change)
- Forgetting to use (n-1) as the complement base (not 12 or any other number)
- Confusing inversion with retrograde (retrograde reverses order; inversion complements values)

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.10.2, pages 144-145
