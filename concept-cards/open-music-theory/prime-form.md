---
concept: Prime Form
slug: prime-form

category: analysis
subcategory: set-theory
tier: advanced

source: "Open Music Theory"
source_slug: open-music-theory
authors: "Open Music Theory contributors"
chapter: "Set Class and Prime Form"
chapter_number: 8
pdf_page: null
section: "VIII.4"

extraction_confidence: high

aliases: []

prerequisites:
  - normal-order
  - set-class
extends: []
related:
  - forte-number
  - interval-vector
contrasts_with: []

answers_questions:
  - "What is prime form?"
  - "How do you calculate the prime form of a set?"
  - "What is the relationship between prime form and set class?"
---

# Quick Definition
Prime form is the label for a set class: the version of a pitch-class set that is transposed to start on 0 and is most compact to the left (compared with its inversion), written in parentheses without commas, e.g., (014) or (02357).

# Core Definition
Prime form provides a unique, canonical label for each set class. The algorithm: (1) put the pc set in normal order; (2) transpose so the first pc is 0; (3) invert the result (I0) and put in normal order; (4) transpose the inversion so the first pc is 0; (5) compare the two results -- whichever is most compact to the left is the prime form. Write in parentheses without commas. Prime form is just a label; it has no special musical status. A composer using {3,4,7} is not using a "transposition of the prime form" in any meaningful sense -- both {0,1,4} and {3,4,7} are equally valid members of set class (014). All possible set classes are catalogued in the set class table (available on Wikipedia and many other resources), organized by cardinality.

# Prerequisites
- Normal order (to standardize set representation)
- Set class (to understand what prime form labels)

# Key Properties
1. Written in parentheses with no commas: (014), (037), (02357)
2. Always starts on 0 (transposed to zero)
3. Most compact to the left compared with its inversion
4. Serves as the name/label for a set class
5. The set class table lists all possible prime forms organized by cardinality
6. Has no special musical status -- it is purely a label

# Context & Application
Prime form is the standard naming convention for set classes in post-tonal theory. Analysts use it to classify sonorities, compare materials across passages, and communicate about set-class relationships. The set class table (organized by cardinality) lists all prime forms alongside their Forte numbers, interval vectors, and complements.

# Examples
**Example 1**: From Bartok's "Subject and Reflection" -- pc set [10,0,2,3,5] in normal order. Transpose to 0: T2=[0,2,4,5,7]. Invert (I0) and normalize: [5,7,8,10,0]. Transpose to 0: T7=[0,2,3,5,7]. Compare: (02457) vs (02357). Most compact left: (02357).

**Example 2**: Major triad {0,4,7}. Normal order [0,4,7]. Already starts on 0: (047). Invert and normalize: {0,8,5}=[5,8,0]. Transpose to 0: [0,3,7]=(037). Compare (047) vs (037): (037) is more compact. Prime form is (037).

# Relationships
## Builds Upon
- **normal-order** -- First step in finding prime form
- **set-class** -- Prime form is the label for a set class
## Related
- **forte-number** -- Alternative naming system for set classes
- **interval-vector** -- Another property shared by all members of a set class

# Common Confusions
- **Confusion**: Prime form has special musical significance
  **Clarification**: It is only a label; no pc set is more important because it matches the prime form integers
- **Confusion**: Prime form and normal order are the same
  **Clarification**: Normal order describes one specific pc set; prime form identifies the set class (transposed to 0, compared with inversion)
- **Confusion**: Every set class has a unique prime form
  **Clarification**: Yes, each set class has exactly one prime form, but Z-related set classes share the same interval vector despite different prime forms

# Source Reference
Open Music Theory, Part VIII, Chapter 4: "Set Class and Prime Form."

# Verification Notes
- Definition source: Directly from 08-04 source chapter with step-by-step algorithm
- Confidence rationale: High -- well-defined algorithm with clear examples
- Preserved from v2: Bartok example, major triad example, algorithm steps
- Cross-reference status: Verified against Wikipedia set class table
