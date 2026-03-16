---
concept: Corpus Analysis Methodology
slug: corpus-analysis-methodology

category: analysis
subcategory: corpus-analysis
tier: advanced

source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Appendix 4: Corpus Analysis, Statistics, and Grammar"
chapter_number: null
pdf_page: 569
section: null

extraction_confidence: high

aliases:
  - computational corpus study
  - open-data methodology

prerequisites:
  - functional-analysis
extends:
  - corpus-analysis-methods
related:
  - musical-grammar-as-interpretive-summary
  - epistemic-circularity-in-harmonic-analysis
contrasts_with: []

answers_questions:
  - "How do you apply corpus analysis methods to harmonic data?"
  - "What is the relationship between frequency and grammaticality in music?"
  - "How does Tymoczko's empirical approach differ from standard statistical methods?"
---

# Quick Definition

Tymoczko's empirical approach to music theory combining computational corpus analysis with traditional scholarship: releasing open data and code while writing informally, focusing on large observable effects rather than subtle statistical findings.

# Core Definition

Tymoczko adopts a "philosophy of open data -- writing in a relatively informal mode but releasing all my code and data so that readers can reproduce and criticize my results" (p. 571). He avoids null-hypothesis significance testing for three reasons: informal writing reaches more readers, "large effects can often be seen without sophisticated statistics" (p. 569), and there are "significant methodological questions that need to be resolved before we can profit from rigorous statistical methodology" (p. 570). The core methodological insight is "the paradox of analysis": "we have to know what we're looking for in order to find it" (p. 570), making theory-neutral datasets potentially impossible.

# Prerequisites

- **Functional analysis** -- The harmonic analysis that provides the data for corpus study

# Key Properties

1. Open-data philosophy: all code and data released for reproduction
2. Focus on large, visible effects rather than subtle statistical patterns
3. Informal writing combined with data transparency
4. Acknowledges theory-dependence of analysis without considering it viciously circular
5. Corpus data serve multiple functions: directing analytical attention, guiding students, pointing toward hypotheses, hinting at cognitive representations
6. Simple Roman-numeral labeling "requires a nontrivial sense of a composer's vocabulary" (p. 570)

# Construction / Recognition

## To Apply Corpus Analysis Methods:
1. Create machine-readable analyses of musical scores (e.g., Roman-numeral annotations)
2. Write code to query the data for specific patterns
3. Focus on large effects visible without sophisticated statistics
4. Release data and code for others to reproduce and critique
5. Be aware that analytical choices (e.g., what counts as a "chord") affect results
6. Acknowledge the theory-dependence of your analysis in your methodology

# Context & Application

The methodology underlies the entire book's empirical claims. Corpus analysis serves as a powerful tool for traditional scholarship, as it was for Jeppesen and McHose. Computers act as "virtual assistants, increasing our access to musical examples" (p. 569). The approach simultaneously bolsters traditional theories and identifies their limits. Data and code are available at madmusicalscience.com/taom.

# Examples

**Example 1** (p. 569): Bach chorales contain 45 instances of I-X-I6, with 34 being I-vii6-I6 and only 1 being I-ii-I6. Mozart piano sonatas have ~115 such progressions with no indisputable I-ii-I6. "Such regularities speak for themselves."

**Example 2** (p. 570): Huron's analysis of the ii-vii6 idiom produces claims about Bach's supertonic progressions that differ from Tymoczko's by a factor of at least 10, due to different analytical decisions about what constitutes a chord.

**Example 3** (p. 569): Tymoczko discovered Renaissance sevenths when they caused his nonharmonic-tone-identification algorithm to fail, leading to traditional-style theorizing.

# Relationships

## Builds Upon
- **Corpus analysis methods** -- Specific computational techniques for studying music
- **Functional analysis** -- Provides the analytical framework for labeling data

## Enables
- **Musical grammar as interpretive summary** -- The metatheoretical position that grammars summarize statistical tendencies
- **Epistemic circularity in harmonic analysis** -- Recognizing the theory-dependence of data

## Related
- **Epistemic circularity in harmonic analysis** -- The holistic interdependence of analysis and theory

## Contrasts With
- None listed

# Common Errors

- **Error**: Assuming corpus analysis can be theory-neutral
  **Correction**: Analysis is theory-dependent; "the paradox of analysis is that we have to know what we're looking for in order to find it" (p. 570)

- **Error**: Using sophisticated statistics for effects that are obvious to the naked eye
  **Correction**: "Statistics are necessary when we are dealing with subtle effects not obvious to the naked eye" -- musical behaviors of interest tend to be considerably more obvious (p. 569)

# Common Confusions

- **Confusion**: Thinking frequency and grammaticality are the same thing in music
  **Clarification**: They may be much closer in music than in language, because music may lack the semantic dimension that supports their separation in language

- **Confusion**: Assuming contemporary composition can adjudicate historical grammaticality
  **Clarification**: "The question is not what we think about classical harmony but what its native speakers thought" (p. 572)

# Source Reference

Appendix 4: "Corpus Analysis, Statistics, and Grammar," pp. 569-574. Data and code at madmusicalscience.com/taom.

# Verification Notes

- Definition source: Direct from pp. 569-571
- Confidence rationale: HIGH -- explicitly presented as the book's methodological framework
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: the Bach/Mozart I-X-I6 data, the Huron disagreement example, the "paradox of analysis" quotation
