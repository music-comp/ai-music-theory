# Extraction Log: The Oxford Handbook of Neo-Riemannian Music Theories

## Source
- **Title**: The Oxford Handbook of Neo-Riemannian Music Theories
- **Editors**: Edward Gollin and Alexander Rehding
- **Publisher**: Oxford University Press, 2011
- **Source slug**: neo-riemannian-handbook
- **Chapters**: 20 (across 6 parts)

## Re-Extraction (v3.1) — 2026-03-15

### Pre-Extraction Audit
- **Existing cards**: 86 (all v2 format)
- **Missing frontmatter fields**: 12/12 fields missing across all 86 cards
- **Missing body sections**: 5+ sections missing across all cards
- **LLM artifacts found**: 1 (leittonwechsel.md — "Wait - let me recalculate")

### Extraction Process
- **Method**: 5 parallel Opus agents + 1 cleanup agent
- **Agent assignments**:
  - Agent 1: Ch 1-4 (Holtmeier, Steege, Hyer x2) — 28 cards
  - Agent 2: Ch 5-8 (Bent, Klumpenhouwer, Rehding, Tymoczko) — 33 cards
  - Agent 3: Ch 9-12 (Gollin, Clark, Cohn, Engebretsen) — 33 cards
  - Agent 4: Ch 13-16 (Gollin, Kopp, Caplin, Burnham) — 30 cards
  - Agent 5: Ch 17-20 (Berry, Rings, Cook, Harrison) — 30 cards
  - Cleanup Agent: 24 orphaned v2 cards across multiple chapters

### Post-Extraction Results
- **Total cards**: 178
- **Cards re-extracted from v2**: 63
- **New cards created**: 92
- **Cards re-extracted by cleanup agent**: 24 (orphaned from initial pass)
- **Net gain**: +92 cards (86 → 178, 107% increase)

### Validation Results
- Required frontmatter fields: 178/178 complete (0 missing)
- Slug/filename consistency: 178/178 match (0 mismatches)
- Body sections (all 11 required): 178/178 complete
- LLM artifacts: 0 (leittonwechsel.md artifact removed)

### Coverage by Part
| Part | Chapters | Cards |
|------|----------|-------|
| Part 1: Riemann's Legacy | Ch 1-2 | ~20 |
| Part 2: Dualism | Ch 5-7 | ~30 |
| Part 3: Tone Relations & Spaces | Ch 3-4, 8-11 | ~50 |
| Part 4: Harmonic Relations & Spaces | Ch 12-14 | ~25 |
| Part 5: Rhythmic-Metric Theories | Ch 15 | ~15 |
| Part 6: Analytical Practices | Ch 16-20 | ~38 |

### Competency Questions
- 45 CQs defined in `competency-questions.md`
- Coverage verified across all 5 agent reports
