# A Guide for Parallel Concept Card Extraction

This guide documents how to extract concept cards from primary sources that have been converted from PDF, LaTeX, or EPUB formats to Markdown, using Claude Code with Opus agents.

## Prerequisites

### 1. Environment Setup

- **Claude Code** installed and configured
- **Source files** must be in place, e.g.:
  - `sources-md/<source-slug>/*.md` (chapters, sections, etc.)
- **Output directory** ready:
  - `concept-cards/<source-slug>/` (will be created if needed)

### 2. Directory Structure

Your project should have this overall structure (note that the chapter file names are given just as an example -- each source will have different chapters/markdown file names):

```
skill-project-root/
├── sources-md/
│   └── <source-slug>/
│        ├── 00-frontmatter.md
│        ├── 01-basic-concepts.md
│        ├── 02-major-scales-and-key-signatures.md
│        └── ... (through 39-colophon.md)
└── concept-cards/
    └── <source-slug>/
        └── (concept cards will be created here)
```

### 3. Model Requirements

The extraction uses **Opus agents** (claude-opus-4-5) for high-quality concept extraction. Ensure your Claude Code instance has access to Opus models.

---

## Context Prepration

You, the primary Claude Code instance, should be using the Opus Model. You need to analyse the text of the given text (the path to that text will follow the template form of `sources-md/<source-slug>`) by reading and categorising, and performing a high-level first-appoximation inventory of concepts and estimating the number of concept cards that will be necessary to cover the material presented in each chapter.

In this phase you will identify and record the following:

- The full title of the text being processed
- The author or authors of the text being processed
- The number of chapters to be processed
- The full title of each chapter
- The full list of concepts covered by each chapter
- How many concepts are covered in each chapter
- A higher-level of analysis, identifying the major categories of the book based upon the total list of concepts covered
- The identification of the major category for each concept
- You will identify important notes for each chapter

Once you have those estimates, you will split the concepts into five sensible, sequential (by chapter order) groups. Each group of concepts will then be given to a dedicated agent that will perform the actual extraction, following the prompt below. You will need to provide a term or short phrase that summarises the chapter grouping for each agent.

## Agent Prompts

Launch all 5 agents **in parallel** using a single message with multiple Task tool calls. The templates below have the following fields which you will need to replace when you deliver each of the agents with thier own prompt.

Variables:

- <SourceTitle>: the full title of the text (paper, book, repository, etc.) in question
- <SourceSlug>: the short, hyphenated, ASCII-only name for the title (used in directory names, metadata, etc., for internal use)
- <N>: agent index, starting at 1 (valid values: 1, 2, 3, 4, 5)
- <ChStart>: the starting chapter number for an agent
- <ChEnd>: the ending chapter number for an agent
- <ChGroupName>: the term or short phrase that summarises the chapter grouping for the given agent; this term represents a generalisation/grouping of the structure of the book, aligned with the percieved intent of its authors/publishers; this term is orthoganal to the analysis-derived category mentioned above
- <ChapterTitle>: Each markdown file that represents a division of the text (e.g., chapter or title -- the term we use here to apply to any type of division is "chapter") will have a long form; this long form is the chapter title; note that this variable only makes sense in the context of a specific chapter, and doesn't apply in any context outside a specific chapter
- <ChapterSlug>: Each markdown file that represents a division of the text will be of the form <ChapterSlug>.md; note that this variable only makes sense in the context of a specific chapter, and doesn't apply in any context outside a specific chapter
- <EstimatedCardRangeTotal>: Each agent will have an estimated range for the total number of cards its work is expected to create; this is that range
- <EstimatedCardRangeCh>: You will have created an estimate of the total number of cards each chapter is expected to generate; this is that estimated range; note that this variable only makes sense in the context of a specific chapter, and doesn't apply in any context outside a specific chapter
- <ListOfConcepts>: Each chapter will have a list of concepts associated with itm with each concept getting a card; this variable is that list of concepts; note that this variable only makes sense in the context of a specific chapter, and doesn't apply in any context outside a specific chapter
- <ImportantNotes>: For each chapter, you will have already recorded important notes; these need to be assembled, de-duped, and prepared for agaent based upon the complete set of chapters that agent will be responsible for; <ImportantNotes> is that processed list

Checks you need to perform:

- look at each group of chapters (Agent<N>, <ChStart>,<ChEnd> ...) and ensure that:

  - every chapter has been assigned to an agent
  - there are no gaps between ranges
  - there are no chapters unassigned
  - chapter assignments are as balanced as possible on total estimated concept cards for the given groups of chapters

### Agent <N>: <ChGroupName> (Chapters <ChStart>-<ChEnd>)

**Description:** "Extract <ChGroupName> cards Ch <ChStart>-<ChEnd>"

**Prompt:**

```
Extract concept cards from "<SourceTitle>" Chapters <ChStart>-<ChEnd> (<ChGroupName> tier).

**Your scope**: Chapters <ChStart>-<ChEnd>
- Ch <ChStart>: <ChapterTitle>
- ...
- ...
- Ch <ChEnd>: <ChapterTitle>

**Target**: ~<EstimatedCardRange> concept cards

**Source files**: Read from `sources-md/<SourceSlug>/*.md`

**Output directory**: `concept-cards/<SourceSlug>/`

**File Structure**: Each concept card will have the following structure:
  - YAML front matter for metadata
  - Markdown content
  - a filename using the contept + the .md file extension
  - files should have descriptive names without numerical prefixes:
     - ✅ `pitch.md`, `staff.md`, `major-scale.md`
     - ❌ `001-pitch.md`, `002-staff.md` (do NOT use numbered prefixes)

**File Template**:

```markdown
---
concept: [Concept Name]
category: [theory/technique/analysis/form]
source: <SourceTitle>
chapter: "[Full Chapter Title]"
unit: [1 or 2]
authors: [author], ...
---

# Quick Definition
[1-2 sentence accessible definition]

# Formal Definition
[Precise technical definition]

# Construction/Recognition
[How to build it or identify it - if applicable]

# Musical Context
[When/where it appears, typical usage]

# Examples
[Specific musical examples from the chapter]

# Related Concepts
- [List related concepts]

# Common Confusions
[What students commonly mix up]

# Source Reference
Chapter X: [Title], Unit X, pages/sections as referenced
```

**Key concepts to extract** (estimated):

- Ch <ChStart>: <ListOfConcepts> (~<EstimatedCardRangeCh> cards)
...
...
- Ch <ChEnd>:  <ListOfConcepts> (~<EstimatedCardRangeCh> cards)

**Important notes**:

<ImportantNotes>

Create all concept card files directly. Work systematically through each chapter.

```

**Model:** opus

---


## Expected Output

### File Creation

Each agent will create `.md` files in `concept-cards/<SourceSlug>/`

### File Naming

Files should have descriptive names without numerical prefixes:

- ✅ `pitch.md`, `staff.md`, `major-scale.md`
- ❌ `001-pitch.md`, `002-staff.md` (do NOT use numbered prefixes)

### Post-Processing

After agents complete:

1. **Ensure naming consistency** (and file extension consistency); for example, some agents may create .yaml instead of .md:

   ```bash
   cd concept-cards/21st-century-classroom
   for file in *.yaml; do mv "$file" "${file%.yaml}.md"; done
   ```

1. **Remove numerical prefixes** if present:

   ```bash
   for file in [0-9][0-9][0-9]-*.md; do
     newname="${file#[0-9][0-9][0-9]-}"
     if [ -f "$newname" ]; then
       newname="${newname%.md}-1.md"
     fi
     mv "$file" "$newname"
   done
   ```

2. **Verify count:**

   ```bash
   ls -1 *.md | wc -l
   # Should show a total within the expected range of <EstimatedCardRangeTotal>
   ```

---

## Troubleshooting

### Issue: Agents create files in wrong location

**Solution:** Check that agents are reading from correct source path. The prompts specify relative paths from project root.

### Issue: Duplicate concepts

**Solution:** Agents work on separate chapter ranges with no overlap. If duplicates appear, check that chapters weren't assigned to multiple agents.

### Issue: Wrong file extensions (.yaml instead of .md)

**Solution:** Run the post-processing command above to rename all .yaml to .md

### Issue: Agent runs out of context

**Solution:** Opus has large context windows, but if an agent fails, you can resume it using its agent ID or re-run just that agent's chapters.

---

## Performance Notes

- **Parallel execution:** All 5 agents should run simultaneously for maximum efficiency
- **Expected runtime:** 15-30 minutes total (depends on machine and API latency)
- **Cost consideration:** Using Opus for all agents will be more expensive than using Sonnet, but produces higher quality concept cards
- **Alternative:** If cost is a concern, use Sonnet instead of Opus by changing `model: "sonnet"` in the Task calls

---

## Quality Verification

After extraction completes, spot-check a few cards from each tier.

Verify each card has:

- ✅ Complete YAML frontmatter
- ✅ All required sections (Quick Definition, Formal Definition, etc.)
- ✅ Proper use of caret notation (^1, ^2, ^3)
- ✅ Specific examples from the source text
- ✅ Accurate source references
