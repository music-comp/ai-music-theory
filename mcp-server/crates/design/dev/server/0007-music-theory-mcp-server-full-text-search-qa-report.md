# Music Theory MCP Server - Full-Text Search QA Report

## Executive Summary

Full-text search is functional with single-word queries performing well and returning relevant results with snippets and relevance scores. However, **multi-word queries frequently return zero results**, severely limiting usability for natural language searches. The core issue appears to be strict AND matching without stemming or phrase handling.

---

## Test Results Summary

### ✅ Working Well

| Query | Results | Notes |
|-------|---------|-------|
| `suspension` | 5 | Good relevance ranking, snippets show context |
| `cadence` | 5 | Found all cadence types |
| `fugue` | 5 | Found fugue, subject, answer, exposition |
| `mode` | 5 | Found all church modes |
| `dominant` | 5 | Found secondary dominant, V7, schemas |
| `modulation` | 10 | Found common chord modulation and related |
| `tritone` | 5 | Good results |
| `galant` | 5 | Found schemas correctly |
| `species` | 5 | Found all species counterpoint types |
| `CHORD` | 5 | Case-insensitive ✓ |

### ❌ Failing (Zero Results)

| Query | Expected | Notes |
|-------|----------|-------|
| `suspension dissonance resolution` | Suspension card | 3-word AND fails |
| `dominant seventh resolution` | Dominant 7th card | 3-word AND fails |
| `fugue subject answer` | Fugue-related cards | 3-word AND fails |
| `V I resolution` | Cadence cards | Short terms filtered? |
| `parallel fifths forbidden` | Parallel 5ths card | 3-word AND fails |
| `raised sixth minor` | Dorian mode card | 3-word AND fails |
| `sonata form exposition development` | Sonata form card | 4-word AND fails |
| `thirds sixths` | Consonance cards | 2-word AND fails |
| `leading tone tonic` | Voice leading cards | 3-word AND fails |
| `Prinner Romanesca` | Schema cards | 2-word proper nouns fail |
| `schema opening` | Schema cards | 2-word AND fails |
| `common chord pivot` | Modulation card | 3-word AND fails |
| `species first second` | Species cards | 3-word AND fails |
| `"imperfect consonance"` | Perfect/imperfect card | Quoted phrase fails |
| `what is a cadence` | Cadence card | Natural language fails |
| `how to write counterpoint` | Counterpoint cards | Natural language fails |
| `writing melodies` | Melody cards | Verb form fails |

### ⚠️ Partial Success (Lower Relevance)

| Query | Results | Notes |
|-------|---------|-------|
| `authentic cadence` | 5 | Works, but lower relevance than single `cadence` |
| `dorian mode` | 2 | Works, fewer results than `mode` alone |
| `tonal answer` | 5 | Works well |
| `Bach fugue` | 5 | Works, but lower relevance scores |
| `secondary theme` | 5 | Works, lower relevance |
| `imperfect consonance` | 5 | Works (without quotes) |
| `stepwise motion` | 5 | Works |
| `4-3 suspension` | 3 | Works, hyphenated term OK |
| `mi contra fa` | 2 | Works, 3 words OK (all present in single doc?) |

---

## Identified Issues

### 1. **Multi-Word AND Logic Too Strict** (Critical)

The search appears to require ALL terms to be present in a document. When users search `fugue subject answer`, they likely want documents about ANY of those concepts, not only documents containing all three words.

**Impact**: Most natural search queries fail completely.

**Evidence**:
- `fugue` → 5 results
- `subject` → 5 results  
- `fugue subject answer` → 0 results

### 2. **No Stemming** (High)

Different word forms return different results:
- `resolve` → 5 results (different set)
- `resolving` → 5 results (different set)
- `resolutions` → 5 results (different set)

Users shouldn't need to guess the exact word form in the documents.

**Evidence**:
- `writing` returns nothing, `write` returns nothing, but concepts about composition exist
- `chords` (plural) gets different results than `chord` (singular)

### 3. **No Stopword Handling** (Medium)

Common words like "how", "what", "is", "a", "the" cause queries to fail.

**Evidence**:
- `what is a cadence` → 0 results
- `cadence` → 5 results

### 4. **Quoted Phrase Search Not Supported** (Medium)

`"imperfect consonance"` returns 0 results while `imperfect consonance` (without quotes) returns 5.

### 5. **Short Terms May Be Filtered** (Low)

Roman numerals and short music theory terms may be getting filtered:
- `V I resolution` → 0 results
- `V-I` → 5 results (hyphenated works)

---

## Recommendations

### Priority 1: Fix Multi-Word Query Handling

**Option A: Default to OR with relevance boosting**
- `fugue subject answer` should match documents containing ANY of those terms
- Documents with more matching terms rank higher
- This matches user expectations for search

**Option B: Implement smart AND/OR**
- 2 words: AND
- 3+ words: OR with boost for multiple matches
- Configurable threshold

### Priority 2: Add Stemming

Use a stemmer (Porter, Snowball) to normalize:
- `resolve`, `resolving`, `resolved`, `resolution`, `resolutions` → `resolv`
- `write`, `writing`, `written` → `writ`

Libraries:
- Rust: `rust-stemmers`
- If using SQLite FTS5: built-in Porter stemmer

### Priority 3: Add Stopword List

Filter common English stopwords before search:
```
a, an, the, is, are, was, were, be, been, being,
have, has, had, do, does, did, will, would, could,
should, may, might, must, shall, can, need, dare,
ought, used, what, how, why, when, where, which,
who, whom, this, that, these, those, it, its, of,
for, to, in, on, at, by, with, from, about, into,
through, during, before, after, above, below, between
```

### Priority 4: Support Phrase Search

Allow quoted phrases to match exact sequences:
- `"imperfect consonance"` matches that exact phrase
- Useful for technical terms and proper nouns

### Priority 5: Handle Short Terms

Don't filter out short terms that are meaningful in context:
- Roman numerals: I, V, ii, IV, vi, vii
- Music terms: do, re, mi, fa, sol, la, ti

Consider a domain-specific allowlist for short terms.

---

## Optional Enhancements

### Fuzzy Matching
Allow for typos: `suspenison` → `suspension`

### Synonym Expansion
- `V7` ↔ `dominant seventh` ↔ `dominant 7th`
- `PAC` ↔ `perfect authentic cadence`
- `HC` ↔ `half cadence`

### Field-Specific Boosting
Weight matches in title higher than body text.

### Category Scoping
Allow: `search_concepts(query: "suspension", category: "voice-leading")`

---

## Test Cases for Validation

After implementing fixes, these queries should return relevant results:

```
# Multi-word (currently failing)
fugue subject answer          → fugue-related cards
suspension dissonance         → suspension card
sonata form exposition        → sonata form card
parallel fifths octaves       → parallel fifths card

# Stemming (currently inconsistent)
resolving                     → same as "resolve"
writing counterpoint          → counterpoint cards
chords progressions           → chord/harmony cards

# Natural language (currently failing)
what is a cadence             → cadence cards
how to write a fugue          → fugue cards

# Phrase search (currently failing)
"perfect authentic cadence"   → authentic cadence card
"leading tone"                → scale degree / voice leading cards

# Short terms (currently uncertain)
V I                           → cadence cards
ii V I                        → circle of fifths card
```

---

## Backend Information

Current backend reported: `"backend": "simple"`

Consider upgrading to:
- **SQLite FTS5**: Full-featured, built-in stemming, phrase search
- **Tantivy** (Rust): Fast, modern, full-featured
- **MeiliSearch**: Easy setup, typo tolerance, great relevance

---

## Appendix: Raw Test Data

### Single-Word Queries (All Successful)
| Query | Total | Top Result | Top Relevance |
|-------|-------|------------|---------------|
| suspension | 5 | Suspension | 18.14 |
| cadence | 5 | Plagal Cadence | 20.99 |
| fugue | 5 | Fugue | 18.81 |
| mode | 5 | Ionian Mode | 26.62 |
| dominant | 5 | Secondary Dominant | 14.04 |
| tritone | 5 | Tritone | 25.11 |
| galant | 5 | Galant Schema | 9.83 |
| species | 5 | Fifth Species | 22.34 |
| chord | 5 | Chord | 32.95 |
| melody | 5 | Melody and Accompaniment | 26.96 |
| modulation | 10 | Common Chord Modulation | 13.48 |
| pivot | 3 | Common Chord Modulation | 12.36 |
| opening | 5 | Meyer-Family Opening Schemas | 11.03 |
| raised | 5 | Dorian Mode | 11.29 |
| avoid | 5 | Musica Ficta | 4.49 |
| rules | 5 | Species Counterpoint | 2.24 |
| thirds | 5 | Parallel Motion | 4.03 |
| conjunct | 4 | Gap Fill | 0.77 |

### Two-Word Queries
| Query | Total | Success | Notes |
|-------|-------|---------|-------|
| authentic cadence | 5 | ✅ | Lower relevance than single word |
| dorian mode | 2 | ✅ | Fewer results |
| tonal answer | 5 | ✅ | Good results |
| Bach fugue | 5 | ✅ | Lower relevance |
| secondary theme | 5 | ✅ | Works |
| imperfect consonance | 5 | ✅ | Works |
| stepwise motion | 5 | ✅ | Works |
| leading tone | 5 | ✅ | Works |
| mi contra fa | 2 | ✅ | 3 words, works |
| parallel fifths | 5 | ✅ | Works |
| thirds sixths | 0 | ❌ | AND too strict |
| Prinner Romanesca | 0 | ❌ | Proper nouns fail |
| schema opening | 0 | ❌ | AND too strict |
| V-I | 5 | ✅ | Hyphenated works |

### Three+ Word Queries (All Failed)
| Query | Total |
|-------|-------|
| suspension dissonance resolution | 0 |
| dominant seventh resolution | 0 |
| fugue subject answer | 0 |
| V I resolution | 0 |
| parallel fifths forbidden | 0 |
| raised sixth minor | 0 |
| sonata form exposition development | 0 |
| common chord pivot | 0 |
| species first second | 0 |
| leading tone tonic | 0 |
| what is a cadence | 0 |
| how to write counterpoint | 0 |
| writing melodies | 0 |
