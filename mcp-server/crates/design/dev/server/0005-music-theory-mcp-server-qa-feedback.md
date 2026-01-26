# Music Theory MCP Server - QA Feedback

## Issue: Category Indexing Mismatch

### Observed Behavior

When calling `list_concepts`, all 146 concepts return with:
```json
"category": "open-music-theory"
```

However, examining an actual concept card file reveals a different `category` value in the YAML frontmatter:

```yaml
---
concept: Species Counterpoint
category: voice-leading        # <-- This is the intended category
source: Open Music Theory
chapter: "Introduction to Species Counterpoint"
part: 2
---
```

### Expected Behavior

The `category` field returned by `list_concepts` should reflect the YAML frontmatter `category` value (e.g., `voice-leading`), not the source name.

### Impact

Currently unable to:
- List all distinct categories in the knowledge base
- Filter concepts by thematic category (e.g., `voice-leading`, `harmony`, `form`, `post-tonal`)
- Navigate the knowledge base by topic rather than by source

This limits the connector's usefulness for thematic exploration and cross-source concept discovery.

---

## Suggested Improvements

### 1. Fix Category Indexing (High Priority)

Index the YAML `category` field from frontmatter instead of (or in addition to) source name.

**Current behavior:**
```json
{ "id": "species-counterpoint", "category": "open-music-theory" }
```

**Expected behavior:**
```json
{ "id": "species-counterpoint", "category": "voice-leading", "source": "open-music-theory" }
```

### 2. Add `list_categories` Tool (High Priority)

New tool to return distinct category names with counts:

```
Tool: list_categories
Parameters: none (or optional source filter)
Returns: { "categories": [
  { "name": "voice-leading", "count": 23 },
  { "name": "harmony", "count": 31 },
  { "name": "form", "count": 18 },
  ...
]}
```

### 3. Separate `category` and `source` Fields (Medium Priority)

Modify `list_concepts` response to include both:

```json
{
  "id": "species-counterpoint",
  "title": "Species Counterpoint",
  "category": "voice-leading",
  "source": "open-music-theory",
  "chapter": "Introduction to Species Counterpoint",
  "preview": "..."
}
```

This allows filtering by thematic category AND by source text.

### 4. Add Category Parameter to `search_concepts` (Low Priority)

Allow scoping searches to specific categories:

```
search_concepts(query: "dissonance", category: "voice-leading")
```

---

## Questions to Resolve

1. What is the canonical list of categories being used in concept cards?
   - Need to audit existing cards for consistency
   - Consider standardizing category taxonomy

2. Should categories be hierarchical?
   - e.g., `counterpoint/species` vs flat `voice-leading`

3. Should a concept card be allowed multiple categories?
   - e.g., `category: [voice-leading, counterpoint]`

---

## Testing Checklist

After fixes are implemented:

- [ ] `list_concepts` returns YAML `category` field value
- [ ] `list_concepts` with `category` parameter filters correctly
- [ ] `list_categories` tool exists and returns accurate counts
- [ ] New cards added while server is running are indexed with correct category
- [ ] Search results include category field for filtering in responses
