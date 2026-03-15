---
# === CORE IDENTIFICATION ===
concept: Set Class
slug: set-class

# === CLASSIFICATION ===
category: analysis
subcategory: set-theory
tier: advanced

# === PROVENANCE ===
source: "Open Music Theory"
source_slug: open-music-theory
authors: "Open Music Theory contributors"
chapter: "Set Class and Prime Form"
chapter_number: 8
pdf_page: null
section: "VIII.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "pitch-class set class"
  - "pcset class"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - pitch-class-set
  - normal-order
extends: []
related:
  - prime-form
  - forte-number
  - interval-vector
  - z-relation
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a set class?"
  - "Why do certain sonorities sound similar despite different transpositions?"
  - "How does set class relate to prime form and Forte number?"
---

# Quick Definition
A set class is a group of all pitch-class sets related by transposition (Tn) or inversion (In). All members share the same interval content, named by their prime form (e.g., (037) for major/minor triads) or Forte number (e.g., 3-11).

# Core Definition
Set class is the most abstract level of harmonic/melodic classification in set theory. Just as a "class" in set theory means "group," a set class groups together all pc sets that are transpositionally or inversionally equivalent. The terminology hierarchy: pitch class groups pitches (by octave/enharmonic equivalence); pitch-class set groups pitch classes (analyst's choice); set class groups pitch-class sets (by Tn/In equivalence). All members of a set class share the same interval vector, explaining why they sound fundamentally similar. Set classes are named by their prime form and catalogued by Forte number. There are 220 set classes total (from cardinality 0 to 12).

# Prerequisites
- Pitch-class set and normal order (to identify and compare sets)

# Key Properties
1. All members related by Tn or In share the same interval content
2. Named by prime form (parentheses, no commas): (014), (037), (02357)
3. Also identified by Forte number: cardinality-catalog position (e.g., 3-11)
4. Number of distinct members depends on symmetry (asymmetric sets: 24 members; symmetric: fewer)
5. Major and minor triads belong to the same set class (037) because they are inversionally related
6. 220 total set classes exist (including null set and aggregate)
7. Complement pairs: a set class and its complement together complete the twelve-tone aggregate

# Context & Application
Set class explains why certain sonorities sound similar across different transpositions and inversions. In analysis, tracking set classes reveals compositional unity -- a piece might use only 3-4 set classes despite dozens of different pc sets. The concept is analogous to (but more general than) tonal chord types: just as "major triad" encompasses C major, F-sharp major, etc., set class (037) encompasses all major and minor triads.

# Examples
**Example 1**: Major and minor triads -- C major {0,4,7}, D major {2,6,9}=T2, F minor {5,8,0}=I0 of C major. All are set class (037), Forte number 3-11.

**Example 2** (Bartok, "Subject and Reflection"): Four different pc sets across two passages -- right hands related by T5, left hands related by T5, hands related by I8 and I6. All belong to set class (02357).

**Example 3**: Set class (014) -- semitone + minor third. Members include {0,1,4}, {1,2,5}, {5,6,9}, and all inversions. All share interval vector <1,0,1,1,1,0>.

# Relationships
## Builds Upon
- **pitch-class-set** -- Set class groups pc sets by Tn/In equivalence
- **normal-order** -- Sets must be in normal order before finding prime form
## Related
- **prime-form** -- The label for a set class
- **forte-number** -- Catalog number for a set class
- **interval-vector** -- All members of a set class share the same interval vector

# Common Confusions
- **Confusion**: Set class and pc set are the same thing
  **Clarification**: A pc set is one specific collection; a set class is the equivalence class of all Tn/In-related pc sets
- **Confusion**: Prime form is a privileged member of the set class
  **Clarification**: (014) does not make {0,1,4} more "important" than {3,4,7}; it is just a label
- **Confusion**: Major and minor triads are different set classes
  **Clarification**: They are inversionally related and thus the same set class (037)

# Source Reference
Open Music Theory, Part VIII, Chapter 4: "Set Class and Prime Form."

# Verification Notes
- Definition source: Directly from 08-04 source chapter
- Confidence rationale: High -- clearly defined with terminology hierarchy
- Preserved from v2: Bartok example, major/minor triad comparison, (014) interval vector
- Cross-reference status: Verified against set class table on Wikipedia
