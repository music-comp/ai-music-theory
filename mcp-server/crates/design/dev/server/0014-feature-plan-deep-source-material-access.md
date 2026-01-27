# Feature Plan: Deep Source Material Access

## Vision

Enable Claude to have deep, nuanced conversations about music theory by providing searchable access to full source texts - not just concept card summaries, but the original arguments, examples, proofs, and context from authoritative texts like Lewin's GMIT, Tymoczko's Geometry of Music, Cohn's Audacious Euphony, etc.

## Current State

### What Exists
- `list_sources` - Returns all registered sources with metadata and conversion status
- `get_source_chapter` - Intended to retrieve chapter content (currently failing)
- `get_source_pdf_path` - Returns filesystem path to original PDF/EPUB
- One source converted: Open Music Theory (123 chapters, markdown)
- 13 sources registered but not converted (PDFs, EPUBs)

### What's Missing
- **No source text search** - Can't search within source materials
- **Chapter retrieval broken** - `get_source_chapter` failing
- **No PDF/EPUB text extraction** - Unconverted sources are inaccessible
- **No graceful handling** - No user-friendly messages when sources unavailable

## Use Cases

### 1. Deep Dive Conversations
**Scenario**: User asks "Can you explain Lewin's concept of a GIS (Generalized Interval System) and how it differs from traditional interval thinking?"

**Current behavior**: I can only offer general knowledge from training data.

**Desired behavior**: 
1. Search Lewin's GMIT for "generalized interval system" 
2. Retrieve relevant chapters/sections
3. Cite specific passages, definitions, and examples
4. Reference page numbers for user follow-up

### 2. Cross-Reference Analysis  
**Scenario**: User asks "How does Tymoczko's voice-leading geometry relate to neo-Riemannian theory?"

**Desired behavior**:
1. Search Tymoczko's Geometry of Music for voice-leading concepts
2. Search Cohn's Audacious Euphony for neo-Riemannian foundations
3. Search Gollin's Oxford Handbook for connections
4. Synthesize across sources with proper citations

### 3. Example Deep-Dives
**Scenario**: User asks "Can you walk me through Cohn's analysis of the Brahms Horn Trio hexatonic poles?"

**Desired behavior**:
1. Locate specific analytical example in Cohn
2. Retrieve full passage with musical examples referenced
3. Explain in context of surrounding argument

### 4. Graceful Degradation
**Scenario**: User without access to copyrighted texts asks same questions.

**Desired behavior**:
1. Check if source exists and is accessible
2. If not: "This analysis appears in Cohn's 'Audacious Euphony', Chapter 4. I don't have access to the full text, but I can explain the general concept from my training..."
3. Offer to work with whatever sources ARE available

---

## Proposed Architecture

### Source Status Levels

```
Level 0: REGISTERED
  - Source is known to system (in config)
  - Path configured but file may not exist
  - No content accessible

Level 1: FILE_EXISTS  
  - File exists at configured path
  - Original format (PDF/EPUB/XML)
  - Not yet processed for search

Level 2: CONVERTED
  - Text extracted to markdown/plaintext
  - Chapter structure preserved
  - Ready for retrieval but not indexed

Level 3: INDEXED
  - Full-text search enabled
  - Tantivy index built
  - Ready for deep queries
```

### New/Enhanced Tools

#### 1. `check_source_availability`
```
Input: source_id
Output: {
  id: "oxford-lewin-gmit",
  status: "FILE_EXISTS" | "CONVERTED" | "INDEXED" | "UNAVAILABLE",
  format: "pdf",
  file_exists: true,
  text_extracted: false,
  searchable: false,
  chapters: null | [...],
  message: "PDF exists but text not yet extracted. Run conversion to enable search."
}
```

#### 2. `search_sources` (NEW - Critical)
```
Input: {
  query: "generalized interval system",
  source_ids: ["oxford-lewin-gmit"] | null,  // null = all indexed sources
  limit: 10
}
Output: {
  results: [
    {
      source_id: "oxford-lewin-gmit",
      source_title: "Lewin - GMIT",
      chapter: "Chapter 2: Generalized Interval Systems",
      page: 23,  // if available
      snippet: "A Generalized Interval System (GIS) consists of...",
      relevance: 0.95
    },
    ...
  ],
  sources_searched: ["oxford-lewin-gmit"],
  sources_unavailable: ["oxford-tymoczko-geometry"]  // registered but not indexed
}
```

#### 3. `get_source_section` (Enhanced)
```
Input: {
  source_id: "oxford-lewin-gmit",
  chapter: "Chapter 2",
  section: "2.3" | null,  // optional subsection
  page_range: [23, 28] | null  // optional page range
}
Output: {
  content: "Full text of section...",
  metadata: {
    chapter: "Chapter 2: Generalized Interval Systems",
    pages: "23-28",
    word_count: 2400
  },
  status: "ok" | "source_unavailable" | "chapter_not_found"
}
```

#### 4. `list_source_chapters`
```
Input: source_id
Output: {
  source_id: "oxford-lewin-gmit",
  chapters: [
    { id: "ch1", title: "Introduction", pages: "1-15" },
    { id: "ch2", title: "Generalized Interval Systems", pages: "16-45" },
    ...
  ],
  status: "ok" | "not_converted"
}
```

---

## Implementation Plan

### Phase 1: Fix Current Tools & Add Diagnostics

**Goal**: Get existing functionality working, add visibility into source status.

1. **Debug `get_source_chapter`** - Figure out why it's failing
2. **Implement `check_source_availability`** - Essential for graceful degradation
3. **Implement `list_source_chapters`** - For converted sources
4. **Add status checks** - Verify file existence, not just config

**Estimated effort**: Small (1-2 days)

### Phase 2: PDF/EPUB Text Extraction Pipeline

**Goal**: Convert registered sources to searchable text.

1. **Research extraction tools**:
   - PDF: `pdftotext`, `PyMuPDF`, `pdfplumber`, `marker-pdf` (preserves structure)
   - EPUB: `ebooklib`, `epub2txt`
   - Consider OCR for scanned PDFs: `tesseract`, `surya`

2. **Design conversion pipeline**:
   ```
   Source File → Text Extraction → Chapter Detection → 
   Markdown Output → Tantivy Indexing
   ```

3. **Handle edge cases**:
   - Scanned PDFs (need OCR)
   - Complex layouts (tables, musical examples)
   - Mathematical notation
   - Page number preservation

4. **Build conversion CLI**:
   ```bash
   music-theory convert --source oxford-lewin-gmit
   music-theory convert --all
   music-theory convert --status
   ```

**Estimated effort**: Medium (1-2 weeks)

### Phase 3: Source Search Implementation

**Goal**: Full-text search across source materials.

1. **Extend Tantivy index** to include:
   - Source ID
   - Chapter/section
   - Page numbers (where available)
   - Larger text chunks than concept cards

2. **Implement `search_sources`** tool

3. **Tune relevance** for longer documents

4. **Consider chunking strategy**:
   - Paragraph-level for precision?
   - Page-level for context?
   - Section-level for coherent retrieval?

**Estimated effort**: Medium (1 week)

### Phase 4: Copyright & Availability Handling

**Goal**: Graceful handling of missing/unavailable sources.

1. **Source availability categories**:
   - Open Access (e.g., Open Music Theory) - Always available
   - User-Provided (PDFs user owns) - Available if file exists
   - Not Available - Registered but file not present

2. **User-friendly messages**:
   ```
   "I'd like to reference Lewin's GMIT for this, but I don't have 
   access to the full text. The concept you're asking about is 
   discussed in Chapter 2. If you have access to the book, pages 
   23-28 cover this in detail. Based on my general knowledge..."
   ```

3. **Fallback chain**:
   - Try full source text
   - Fall back to concept cards
   - Fall back to general knowledge
   - Always cite what's being used

**Estimated effort**: Small (2-3 days)

---

## Technical Considerations

### PDF Extraction Quality

Music theory PDFs are challenging:
- **Mathematical notation**: GIS definitions, set theory formulas
- **Musical examples**: Staff notation, chord diagrams
- **Complex layouts**: Multi-column, footnotes, figures
- **Scanned sources**: Older texts may be image-based

**Recommendation**: Start with `marker-pdf` (best for academic PDFs) or `PyMuPDF`, evaluate quality, consider `surya` for OCR if needed.

### Index Size & Performance

With 14 sources, potentially thousands of pages:
- Tantivy handles this scale easily
- Consider separate indices per source vs. unified index
- Memory usage for large indices

### Chapter Detection

Automatic chapter detection is hard. Options:
1. **Manual TOC mapping** - Most reliable, requires human input
2. **Heuristic detection** - Look for "Chapter N" patterns
3. **ML-based** - Document segmentation models
4. **Hybrid** - Auto-detect, human-verify

**Recommendation**: Start with manual TOC mapping for key sources, automate later.

### Legal/Ethical Considerations

- Only index files user legally owns
- Don't redistribute extracted text
- Respect DRM (though academic PDFs rarely have it)
- Consider: extracted text stays local, only snippets sent to Claude

---

## Success Criteria

### Phase 1 Complete When:
- [ ] `get_source_chapter` works for Open Music Theory
- [ ] `check_source_availability` returns accurate status
- [ ] `list_source_chapters` works for converted sources

### Phase 2 Complete When:
- [ ] At least 3 PDF sources successfully converted
- [ ] Chapter structure preserved in conversion
- [ ] Conversion CLI functional

### Phase 3 Complete When:
- [ ] `search_sources` returns relevant results
- [ ] Can search across multiple sources
- [ ] Snippets include source/chapter/page info

### Phase 4 Complete When:
- [ ] Unavailable sources return helpful messages
- [ ] Fallback chain works smoothly
- [ ] User experience is graceful regardless of source availability

---

## Prioritized Source Conversion Order

Based on depth and uniqueness of content:

1. **Lewin - GMIT** - Foundational for transformation theory, highly technical
2. **Tymoczko - Geometry of Music** - Unique geometric perspective
3. **Cohn - Audacious Euphony** - Neo-Riemannian theory masterwork
4. **Straus - Post-Tonal Theory** - Standard pedagogical text
5. **Gollin - Oxford Handbook** - Comprehensive neo-Riemannian reference
6. **Messiaen - Technique** - Composer's own voice, unique modes/rhythms
7. **Tymoczko - Tonality: An Owner's Manual** - More accessible Tymoczko
8. **Math/Group theory papers** - Supporting mathematical foundations

---

## Questions for Discussion

1. **Chunking granularity**: Should search return paragraphs, pages, or sections?
2. **Citation format**: How should Claude cite sources? (Author, page) vs. full bibliographic?
3. **Cross-source linking**: Should we link related content across sources?
4. **User source addition**: Should users be able to add their own PDFs?
5. **Offline vs. online**: Should converted text be indexable without internet?
