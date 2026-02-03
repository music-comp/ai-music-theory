---
number: 11
title: "A Guide for Parallel Concept Card Extraction"
author: "their exact"
component: All
tags: [change-me]
created: 2026-02-03
updated: 2026-02-03
state: Active
supersedes: null
superseded-by: null
version: 1.0
---

# A Guide for Parallel Concept Card Extraction

This guide documents how to extract concept cards from primary sources (books, handbooks, papers, repositories) that have been converted from PDF, LaTeX, or EPUB formats to Markdown, using Claude Code with Opus agents working in parallel.

## Overview

This is a **two-phase process**:

1. **Phase 1: Analysis & Planning** - The coordinator (you, the primary Claude Code instance) analyzes the source material and creates detailed extraction plans
2. **Phase 2: Parallel Extraction** - Five Opus agents extract concept cards simultaneously based on the plans

## Prerequisites

### 1. Environment Setup

- **Claude Code** installed and configured
- **Model access**: Coordinator should use **Opus model**, with access to spawn Opus agents
- **Source files** in place at `sources-md/<source-slug>/` with chapter/section files as `.md`
- **Output directory** ready at `concept-cards/<source-slug>/` (will be created if needed)

### 2. Directory Structure

Your project should follow this structure (chapter filenames are examples - your source will vary):

```
project-root/
├── sources-md/
│   └── <source-slug>/
│       ├── 00-frontmatter.md
│       ├── 01-introduction.md
│       ├── 02-chapter-two.md
│       └── ... (additional chapters/sections)
└── concept-cards/
    └── <source-slug>/
        └── (concept cards will be created here)
```

### 3. Source Material Requirements

- Source must be split into logical divisions (chapters, sections, papers)
- Each division should be a separate `.md` file
- Files should be in a single directory: `sources-md/<source-slug>/`

---

## Phase 1: Analysis & Planning

**Model**: Use Opus for this phase

### Step 1: Initial Source Analysis

Use the **Explore agent** with "very thorough" mode to analyze the complete source material:

```
Please analyze the source material in sources-md/<source-slug>/ to create a comprehensive concept extraction plan. Use very thorough exploration mode.
```

The Explore agent should identify and record:

1. **Source metadata:**
   - Full title of the source
   - Author(s) or editor(s)
   - Publication type (book, handbook, collection, etc.)
   - Subject area and scope

2. **Chapter/section inventory:**
   - Total number of chapters/sections
   - Full title of each chapter/section
   - Brief description of each chapter's content
   - Length/complexity of each chapter
   - **PDF page numbers**: Extract from chapter metadata headers (see note below)

3. **Concept inventory:**
   - Complete list of concepts covered in each chapter
   - Estimated number of concept cards per chapter (give ranges: e.g., "8-10 cards")
   - Total estimated concept cards across all chapters

**Note on PDF Page Numbers**: If your source chapters were prepared using `prepare-source-for-indexing-prompt.md`, each chapter file will have a metadata header like:

```markdown
<!--
Metadata:
  PDF Page: 17
  Original Line: 466
-->
```

Read this metadata to extract PDF page numbers for each chapter. This information will be passed to agents for inclusion in concept cards.

4. **High-level categorization:**
   - Major thematic divisions of the source (e.g., "Foundation", "Advanced", "Applications")
   - Category for each chapter (which major division it belongs to)
   - Logical groupings of chapters based on theme and complexity

5. **Domain-specific considerations:**
   - Notation systems used (if any) - e.g., mathematical notation, musical notation, code syntax
   - Special terminology or conventions
   - Types of examples provided (figures, equations, code blocks, musical scores)

### Step 2: Create Balanced Agent Assignments

After analysis, divide the work among **exactly 5 agents** by:

1. **Group chapters sequentially** - maintain chapter order, no gaps or overlaps
2. **Balance workload** - aim for roughly equal estimated card counts per agent (±20%)
3. **Respect natural divisions** - don't split major thematic sections across agents when possible
4. **Create meaningful groupings** - each agent should have a coherent set of related chapters

**Assignment verification checklist:**

- [ ] Every chapter assigned to exactly one agent
- [ ] No gaps in chapter ranges (sequential coverage)
- [ ] No unassigned chapters
- [ ] Card count estimates roughly balanced across agents (±20%)
- [ ] Agent groupings align with natural thematic divisions where possible
- [ ] Each agent has a clear, descriptive group name

### Step 3: Customize the Concept Card Template

The default template below is designed for technical/academic content. **Customize the section headings** based on your source material's domain:

**Default sections** (suitable for most technical content):

- Quick Definition
- Formal Definition
- Construction/Recognition
- Context/Application
- Examples
- Related Concepts
- Common Confusions
- Source Reference

**Domain-specific alternatives:**

For **music theory**:

- Use "Musical Context" instead of "Context/Application"
- Add note about caret notation (^1, ^2, ^3 for scale degrees)
- Mention lead-sheet symbols, Roman numerals

For **mathematics**:

- Use "Mathematical Definition" instead of "Formal Definition"
- Add "Proof Sketch" or "Derivation" section
- Add "Special Cases" section
- Use LaTeX notation guidelines

For **computer science/programming**:

- Use "Algorithm/Implementation" instead of "Construction/Recognition"
- Add "Complexity Analysis" section
- Add "Code Examples" section
- Specify language/syntax conventions

For **physics/chemistry**:

- Add "Physical Interpretation" section
- Add "Units/Dimensions" section
- Add "Experimental Context" section

### Step 4: Compile Important Notes

For each agent, compile a consolidated list of important notes from their assigned chapters:

**Types of important notes to include:**

- Notation conventions (e.g., "Use caret notation: ^1, ^2, ^3")
- Common formatting requirements (e.g., "Include lead-sheet symbols alongside Roman numerals")
- Domain-specific rules (e.g., "Distinguish between strict and liberal approaches")
- Prerequisite knowledge (e.g., "These concepts build on Foundation tier from Chapters 1-8")
- Special handling (e.g., "For multi-chapter concepts, synthesize across sources")
- Quality requirements (e.g., "Include specific musical examples from the text")
- Cross-references (e.g., "Note which concepts appear in multiple chapters")

**Important notes consolidation:**

- Remove duplicates across chapters in the same agent's range
- Group related notes together
- Prioritize the most critical guidelines
- Keep notes concise but specific

---

## Phase 2: Parallel Extraction

### Template Variables Reference

When creating agent prompts, replace these variables:

| Variable | Description | Example |
|----------|-------------|---------|
| `<SourceTitle>` | Full title of the source | "Music Theory for the 21st-Century Classroom" |
| `<SourceSlug>` | Hyphenated ASCII slug for directories | "21st-century-classroom" |
| `<AuthorList>` | Author(s)/editor(s) of source | "Robert Hutchinson" or "John Smith, Jane Doe" |
| `<N>` | Agent number (1-5) | 1, 2, 3, 4, or 5 |
| `<ChStart>` | Starting chapter number for agent | 1, 9, 19, etc. |
| `<ChEnd>` | Ending chapter number for agent | 8, 18, 25, etc. |
| `<ChGroupName>` | Descriptive name for chapter group | "Foundation", "Chromatic Harmony + Forms" |
| `<ChapterNum>` | Specific chapter number (in context) | 5, 12, 23, etc. |
| `<ChapterTitle>` | Full title of specific chapter | "Intervals", "Mode Mixture" |
| `<ChapterSlug>` | Filename slug for chapter | "05-intervals", "19-mode-mixture" |
| `<EstimatedCardRangeTotal>` | Total card range for agent | "60-75", "110-140" |
| `<EstimatedCardRangeCh>` | Card range for specific chapter | "8-10", "4-5" |
| `<ListOfConcepts>` | Comma-separated concept list for chapter | "Major scale, circle of fifths, key signatures, order of sharps/flats" |
| `<ImportantNotes>` | Consolidated notes for agent's chapters | See "Step 4: Compile Important Notes" above |

### Agent Prompt Template

Use this template to create **5 complete prompts** (one per agent). Fill in all `<variables>` before launching agents.

---

#### Agent `<N>`: `<ChGroupName>` (Chapters `<ChStart>`-`<ChEnd>`)

**Description:** "Extract `<ChGroupName>` cards Ch `<ChStart>`-`<ChEnd>`"

**Prompt:**

```
Extract concept cards from "<SourceTitle>" Chapters <ChStart>-<ChEnd> (<ChGroupName> tier).

**Your scope**: Chapters <ChStart>-<ChEnd>
- Ch <ChStart>: <ChapterTitle>
- Ch <ChStart+1>: <ChapterTitle>
... (list all chapters in range)
- Ch <ChEnd>: <ChapterTitle>

**Target**: ~<EstimatedCardRangeTotal> concept cards total

**Source files**: Read from `sources-md/<SourceSlug>/`

**Output directory**: `concept-cards/<SourceSlug>/`

**PDF Page Numbers**: Each chapter file contains a metadata header with the original PDF page number. Extract this information and include it in the `pdf_page` field of each concept card. If a chapter has no PDF page metadata, use `null`.

**File Structure Requirements**:
  - YAML frontmatter for metadata
  - Markdown content sections
  - Filename: concept-name.md (lowercase, hyphenated, NO numerical prefixes)
  - ✅ CORRECT: `pitch.md`, `major-scale.md`, `harmonic-function.md`
  - ❌ WRONG: `001-pitch.md`, `02-major-scale.md`, `concept-3.md`
  - ✅ Extension: Always .md (NOT .yaml, NOT .yml)

**Concept Card Template**:

```markdown
---
concept: [Concept Name - properly capitalized]
category: [choose: theory/technique/analysis/form/application]
source: <SourceTitle>
chapter: "<ChapterTitle>"
chapter_number: [chapter number]
pdf_page: [PDF page number from chapter metadata, or null if not available]
unit: [number or null if not applicable]
authors: <AuthorList>
---

# Quick Definition
[1-2 sentence accessible definition for quick reference]

# Formal Definition
[Precise technical/academic definition with proper terminology]

# Construction/Recognition
[How to build/construct it OR how to identify/recognize it - if applicable]

# Context/Application
[When/where this concept appears, typical usage, relevant domains]

# Examples
[Specific examples from the source text - cite page/section if available]

# Related Concepts
- [List related concepts that students should cross-reference]
- [Include prerequisite concepts]
- [Include advanced extensions]

# Common Confusions
[What students/readers commonly mix up or misunderstand about this concept]

# Source Reference
Chapter <ChapterNum>: <ChapterTitle>, [Unit <UnitNum> if applicable], [pages/sections as referenced in source]
```

**Key concepts to extract** (estimated):

- Ch <ChStart>: <ListOfConcepts> (~<EstimatedCardRangeCh> cards)
- Ch <ChStart+1>: <ListOfConcepts> (~<EstimatedCardRangeCh> cards)
... (list all chapters)
- Ch <ChEnd>: <ListOfConcepts> (~<EstimatedCardRangeCh> cards)

**Important notes for this agent**:

<ImportantNotes>

**General extraction guidelines**:

- Create one concept card per distinct concept (don't combine related concepts into single cards)
- Use clear, descriptive filenames that match the concept name
- Include specific examples from the source text, not generic examples
- Cross-reference related concepts by their exact concept names
- If a concept appears in multiple chapters, decide whether to:
  - Create one synthesized card referencing all chapters, OR
  - Create separate cards for different treatments/contexts
- Maintain consistent terminology with the source material
- Extract concepts in chapter order for organizational clarity

Work systematically through each chapter in your assigned range.

```

**Model:** opus

---

### Launching Agents

**CRITICAL**: Launch all 5 agents **in parallel** using a **single message** with 5 Task tool calls.

**Example message to send:**

```

I need to extract concept cards from "<SourceTitle>" using parallel agents. Please launch all 5 Opus agents simultaneously in this single message using the prompts I've prepared:

[Paste Agent 1 prompt here]

[Paste Agent 2 prompt here]

[Paste Agent 3 prompt here]

[Paste Agent 4 prompt here]

[Paste Agent 5 prompt here]

```

**Why parallel launch matters:**
- All agents start simultaneously
- No waiting for sequential completion
- Maximum efficiency (15-30 min total vs. 75-150 min sequential)
- Even workload distribution

---

## Expected Output

### File Creation

Each agent creates `.md` files in `concept-cards/<SourceSlug>/`:
- **Agent 1**: ~<EstimatedCardRangeTotal> files
- **Agent 2**: ~<EstimatedCardRangeTotal> files
- **Agent 3**: ~<EstimatedCardRangeTotal> files
- **Agent 4**: ~<EstimatedCardRangeTotal> files
- **Agent 5**: ~<EstimatedCardRangeTotal> files

**Total**: Should match your pre-analysis estimated range

### File Naming Standards

✅ **Correct naming:**
- `pitch.md`
- `harmonic-function.md`
- `neapolitan-chord.md`
- `set-theory-overview.md`

❌ **Incorrect naming:**
- `001-pitch.md` (numerical prefix)
- `Pitch.md` (wrong case)
- `harmonic_function.md` (underscore instead of hyphen)
- `pitch.yaml` (wrong extension)

### Post-Processing

After agents complete, run these verification and cleanup steps:

#### 1. Navigate to output directory
```bash
cd concept-cards/<SourceSlug>/
```

#### 2. Check for wrong file extensions

```bash
# Check if any .yaml or .yml files were created
ls -1 *.yaml 2>/dev/null || echo "No .yaml files"
ls -1 *.yml 2>/dev/null || echo "No .yml files"

# If found, rename them to .md
for file in *.yaml; do [ -f "$file" ] && mv "$file" "${file%.yaml}.md"; done
for file in *.yml; do [ -f "$file" ] && mv "$file" "${file%.yml}.md"; done
```

#### 3. Remove numerical prefixes

```bash
# Remove 3-digit prefixes (001-, 002-, etc.)
for file in [0-9][0-9][0-9]-*.md; do
  [ -f "$file" ] || continue
  newname="${file#[0-9][0-9][0-9]-}"
  if [ -f "$newname" ]; then
    # Conflict exists - add -1 suffix
    newname="${newname%.md}-1.md"
  fi
  mv "$file" "$newname"
  echo "Renamed: $file -> $newname"
done

# Remove 2-digit prefixes (01-, 02-, etc.)
for file in [0-9][0-9]-*.md; do
  [ -f "$file" ] || continue
  newname="${file#[0-9][0-9]-}"
  if [ -f "$newname" ]; then
    newname="${newname%.md}-1.md"
  fi
  mv "$file" "$newname"
  echo "Renamed: $file -> $newname"
done
```

#### 4. Verify total count

```bash
total=$(ls -1 *.md | wc -l)
echo "Total concept cards created: $total"
echo "Expected range: <EstimatedCardRangeTotal>"
```

#### 5. Check for duplicates

```bash
# List any files with -1 suffix (indicates naming conflicts)
ls -1 *-1.md 2>/dev/null && echo "⚠️  WARNING: Duplicate concept names found" || echo "✓ No duplicate names"
```

---

## Quality Verification

After extraction completes, perform quality checks:

### Sample Verification

Spot-check 3-5 cards from each agent's output:

**Agent 1** (<ChGroupName>): Check `<example-concept>.md`, `<example-concept>.md`
**Agent 2** (<ChGroupName>): Check `<example-concept>.md`, `<example-concept>.md`
... (etc. for all 5 agents)

### Quality Checklist

For each sampled card, verify:

- [ ] **Complete YAML frontmatter** with all required fields
  - [ ] `concept:` field present and properly formatted
  - [ ] `category:` is one of the allowed values
  - [ ] `source:` matches exact source title
  - [ ] `chapter:` matches exact chapter title
  - [ ] `chapter_number:` is present and correct
  - [ ] `pdf_page:` is present (number or null)
  - [ ] `authors:` field present and accurate

- [ ] **All required sections present**
  - [ ] Quick Definition (1-2 sentences)
  - [ ] Formal Definition (technical/precise)
  - [ ] Construction/Recognition OR appropriate domain section
  - [ ] Context/Application
  - [ ] Examples (specific to source text)
  - [ ] Related Concepts (list format)
  - [ ] Common Confusions
  - [ ] Source Reference (with chapter/section citations)

- [ ] **Content quality**
  - [ ] Definitions are accurate and clear
  - [ ] Examples are specific (from source text, not generic)
  - [ ] Related concepts use exact concept names from other cards
  - [ ] Common confusions are relevant and helpful
  - [ ] Source references include specific chapter/page/section

- [ ] **Formatting consistency**
  - [ ] Proper use of domain-specific notation (if applicable)
  - [ ] Consistent terminology with source material
  - [ ] Proper markdown formatting (headers, lists, code blocks)

- [ ] **Filename correctness**
  - [ ] Lowercase with hyphens
  - [ ] No numerical prefixes
  - [ ] Extension is .md
  - [ ] Name matches concept

### Domain-Specific Quality Checks

Add these checks based on your source material:

**For music theory sources:**

- [ ] Caret notation used correctly (^1, ^2, ^3 for scale degrees)
- [ ] Lead-sheet symbols included where applicable
- [ ] Roman numeral analysis formatted correctly

**For mathematics sources:**

- [ ] LaTeX/mathematical notation rendered correctly
- [ ] Proofs/derivations are accurate
- [ ] Special cases noted appropriately

**For programming sources:**

- [ ] Code examples use proper syntax highlighting
- [ ] Algorithm complexity noted where relevant
- [ ] Language/framework versions specified

---

## Troubleshooting

### Issue: Agents create files in wrong location

**Symptoms**: Files appear in unexpected directories or at project root

**Solution**:

- Verify agent prompts specify correct path: `concept-cards/<SourceSlug>/`
- Check that output directory exists before launching agents
- Ensure agents are working from project root directory

### Issue: Duplicate concepts across agents

**Symptoms**: Multiple files for the same concept, or overlapping coverage

**Solution**:

- Review your agent assignments - ensure no chapter overlaps
- Check if a concept genuinely appears in multiple chapters:
  - If same treatment: Consolidate into single card
  - If different treatments: Keep separate, add cross-references
- Use post-processing to identify `-1.md` files indicating conflicts

### Issue: Wrong file extensions (.yaml or .yml instead of .md)

**Symptoms**: Agents create `.yaml` or `.yml` files

**Solution**: Run the post-processing script above to rename all to `.md`

### Issue: Numerical prefixes on filenames

**Symptoms**: Files named like `001-concept.md`, `02-concept.md`

**Solution**: Run the post-processing script above to remove prefixes

### Issue: Agent runs out of context

**Symptoms**: Agent stops mid-extraction with context limit error

**Solution**:

- Opus has very large context windows - this is rare
- If it occurs, note the agent ID and resume using `resume: "<agent-id>"`
- Or re-run just that agent's chapter range with same prompt

### Issue: Inconsistent card quality across agents

**Symptoms**: Some agents produce better cards than others

**Solution**:

- Review "Important Notes" for each agent - ensure clarity and completeness
- Check if some chapters are more complex and need more detailed guidelines
- Consider having one agent (the one with issues) redo their extraction with enhanced prompts

### Issue: Cards missing required sections

**Symptoms**: Some cards don't have all template sections

**Solution**:

- Verify the template was correctly included in agent prompts
- Check if some sections truly don't apply (mark as "N/A" rather than omit)
- Re-run specific agent with explicit instruction to complete all sections

### Issue: Total card count far from estimate

**Symptoms**: Got 150 cards when expecting 250, or vice versa

**Solution**:

- Review actual source material - initial estimates may have been off
- Check if agents over-split (multiple cards for one concept) or under-split (combined concepts)
- Acceptable variance: ±20% of estimate
- If >20% variance: Review sample cards and adjust extraction criteria

---

## Performance Notes

### Timing

- **Phase 1 (Analysis)**: 10-20 minutes using Explore agent
- **Phase 2 (Extraction)**: 15-30 minutes for all 5 agents in parallel
- **Post-processing**: 2-5 minutes
- **Quality verification**: 10-15 minutes
- **Total**: ~40-70 minutes for complete extraction

### Cost Considerations

**Using Opus throughout** (recommended for quality):

- Coordinator (Phase 1): ~$2-5 depending on source size
- 5 Opus agents (Phase 2): ~$10-25 depending on card count
- **Total**: ~$12-30 for complete high-quality extraction

**Cost-saving alternative** (use Sonnet for agents):

- Change `model: "opus"` to `model: "sonnet"` in agent prompts
- Reduces Phase 2 cost by ~70%
- May reduce quality slightly - review samples carefully
- **Total**: ~$5-15 for extraction

### Efficiency Tips

- **Parallel launch is critical**: Always launch all 5 agents in one message
- **Prepare prompts beforehand**: Complete all Phase 1 analysis before starting Phase 2
- **Use Opus for coordinator**: Better analysis leads to better agent instructions
- **Balance workload**: Unbalanced agents can bottleneck completion time
- **Run post-processing immediately**: Catch issues while context is fresh

---

## Workflow Summary

**For the coordinator (you) to execute:**

### Phase 1: Analysis (10-20 minutes)

1. ✅ Read this guide completely
2. ✅ Launch Explore agent to analyze `sources-md/<source-slug>/`
3. ✅ Review analysis results and refine estimates
4. ✅ Create 5 balanced agent assignments
5. ✅ Customize concept card template for your domain
6. ✅ Compile important notes for each agent
7. ✅ Verify assignment checklist (no gaps, balanced, sequential)
8. ✅ Fill in the agent prompt template 5 times (one per agent)

### Phase 2: Extraction (15-30 minutes)

1. ✅ Launch all 5 agents in parallel in a **single message**
2. ✅ Monitor progress (agents report completion individually)
3. ✅ Wait for all 5 agents to complete

### Phase 3: Post-Processing (2-5 minutes)

1. ✅ Navigate to output directory
2. ✅ Run file extension cleanup
3. ✅ Run numerical prefix removal
4. ✅ Verify total count against estimate
5. ✅ Check for duplicate concept names

### Phase 4: Quality Verification (10-15 minutes)

1. ✅ Sample 3-5 cards from each agent
2. ✅ Run through quality checklist
3. ✅ Check domain-specific requirements
4. ✅ Review any flagged issues
5. ✅ Re-run specific agents if needed (rare)

**Done!** You now have a complete set of high-quality concept cards ready for use.

---

## Example: End-to-End Execution

Here's what a complete execution looks like:

### Example Analysis Output

After Phase 1 analysis, you should have:

```
Source: "Advanced Quantum Mechanics" by J. Sakurai
Chapters: 15 total (01-15)
Total estimated cards: 180-220

Agent assignments:
- Agent 1: Ch 1-3 (Foundation) - 35-45 cards
- Agent 2: Ch 4-6 (Scattering Theory) - 40-50 cards
- Agent 3: Ch 7-9 (Symmetries) - 35-45 cards
- Agent 4: Ch 10-12 (Approximation Methods) - 35-45 cards
- Agent 5: Ch 13-15 (Advanced Topics) - 35-45 cards

Domain customizations:
- Add "Mathematical Formulation" section
- Add "Physical Interpretation" section
- Use LaTeX for equations
- Reference equation numbers from source
```

### Example Agent Prompt (Agent 1)

```
Extract concept cards from "Advanced Quantum Mechanics" Chapters 1-3 (Foundation tier).

**Your scope**: Chapters 1-3
- Ch 1: Fundamental Concepts
- Ch 2: Quantum Dynamics
- Ch 3: Theory of Angular Momentum

**Target**: ~35-45 concept cards total

**Source files**: Read from `sources-md/sakurai-quantum/`

**Output directory**: `concept-cards/sakurai-quantum/`

... [rest of template filled in]
```

### Example Post-Processing

```bash
$ cd concept-cards/sakurai-quantum/
$ ls -1 *.md | wc -l
195

$ ls -1 *.yaml
# (none found - good!)

$ for file in [0-9][0-9][0-9]-*.md; do
>   newname="${file#[0-9][0-9][0-9]-}"
>   mv "$file" "$newname"
> done

$ ls -1 *.md | wc -l
195  # Same count, just renamed

$ echo "Within expected range of 180-220: ✓"
```

---

## Final Notes

- This template is designed for flexibility across domains - customize freely
- The 5-agent split is optimal for parallel processing - don't use more or fewer
- Quality comes from good analysis in Phase 1 - don't rush it
- Parallel launch is essential - sequential will take 5x longer
- Post-processing catches common issues - always run it
- Spot-checking quality is faster than reviewing every card
- If in doubt, over-specify in the Important Notes section

Good luck with your extraction!
