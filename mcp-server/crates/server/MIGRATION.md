# Search Backend Migration Guide

**Version:** 0.2.0
**Date:** 2026-01-27
**Status:** Active

---

## Overview

Version 0.2.0 of the Music Theory MCP Server changes the default search backend from `simple` (linear scan) to `tantivy` (full-text search engine) to provide significantly better search quality.

### What Changed

**Search Quality Improvements:**
- ✅ **Multi-word query support** - Queries like `"fugue subject answer"` now work correctly
- ✅ **Smart AND/OR logic** - 2 words require both terms, 3+ words use intelligent OR matching
- ✅ **Stopword filtering** - Natural language queries like `"what is a cadence"` work properly
- ✅ **Phrase search** - Quoted strings like `"perfect authentic cadence"` match exact phrases
- ✅ **Stemming** - `write`, `writing`, `written` all match the same results
- ✅ **Relevance ranking** - Better result ordering with BM25 algorithm

**Configuration Changes:**
- Default backend changed from `"simple"` to `"tantivy"` in `config/default.toml`
- New configuration options for query mode, stopwords, and phrase search
- SimpleSearch backend marked as deprecated (removal planned for v0.3.0)

### Breaking Changes

**None.** The API remains unchanged:
- `SearchConceptsParams` interface unchanged
- `SearchResult` structure unchanged
- All existing MCP tool calls work as before

---

## Migration Steps

### For New Deployments

**1. Build with FTS feature:**

```bash
# From the mcp-server directory
cargo build --release --features fts
```

**2. Build the search index:**

```bash
# Build index from concept cards
./target/release/music-theory-mcp index
```

**3. Start the server:**

```bash
# Server will use tantivy backend by default
./target/release/music-theory-mcp serve
```

That's it! The server will now use the improved search backend.

---

### For Existing Deployments

**Option A: Upgrade to Tantivy (Recommended)**

**1. Backup your configuration (if customized):**

```bash
cp config/default.toml config/default.toml.backup
```

**2. Rebuild with FTS feature:**

```bash
cargo clean
cargo build --release --features fts
```

**3. Build the initial index:**

```bash
./target/release/music-theory-mcp index

# Expected output:
# Building Tantivy index...
# Indexed 200+ concept cards
# Index build complete: X.XXs
```

**4. Update configuration (if using custom config):**

If you have a custom config file, add these new options:

```toml
[search]
backend = "tantivy"  # Changed from "simple"

# New options (with recommended defaults)
query_mode = "smart"
minimum_match_percent = 0.6
enable_stopwords = true
custom_stopwords = []
stopword_allowlist = ["I", "V", "ii", "IV", "vi", "vii", "i", "v", "iv", "do", "re", "mi", "fa", "sol", "la", "ti"]
```

**5. Restart the server:**

```bash
./target/release/music-theory-mcp serve
```

**6. Verify search quality:**

Test a few queries to ensure the new backend is working:

```bash
# Test multi-word query
curl -X POST http://localhost:3000/search \
  -H "Content-Type: application/json" \
  -d '{"query": "fugue subject answer", "limit": 5}'

# Test natural language query
curl -X POST http://localhost:3000/search \
  -H "Content-Type: application/json" \
  -d '{"query": "what is a cadence", "limit": 5}'

# Test phrase search
curl -X POST http://localhost:3000/search \
  -H "Content-Type: application/json" \
  -d '{"query": "\"perfect authentic cadence\"", "limit": 5}'
```

All queries should now return relevant results.

---

**Option B: Keep Simple Backend (Not Recommended)**

If you need to temporarily stay on the simple backend:

**1. Update your config file:**

```toml
[search]
backend = "simple"  # Explicitly use legacy backend
```

**2. Rebuild and restart:**

```bash
cargo build --release
./target/release/music-theory-mcp serve
```

**Warning:** You will see deprecation warnings in the logs:

```
WARN: SimpleSearch backend is deprecated and will be removed in version 0.3.0.
      Please migrate to 'backend = "tantivy"' for better search quality.
```

**Migration deadline:** The simple backend will be removed in version 0.3.0 (estimated Q2 2026).

---

## Configuration Reference

### Query Modes

Configure how multi-word queries are matched:

```toml
[search]
# Smart mode (recommended): 2 words = AND, 3+ words = OR with 60% minimum match
query_mode = "smart"

# Strict mode: ALL words must be present
# query_mode = "and"

# Loose mode: ANY word matches
# query_mode = "or"

# Custom minimum match: At least 75% of words must match
# query_mode = { minimum_match = 0.75 }
```

**Examples:**

| Query | Smart Mode | AND Mode | OR Mode |
|-------|------------|----------|---------|
| `authentic cadence` | Both required (AND) | Both required | Either matches |
| `fugue subject answer` | 2 of 3 required (60%) | All 3 required | Any 1 matches |

### Stopword Filtering

Control natural language query handling:

```toml
[search]
# Enable stopword removal (recommended)
enable_stopwords = true

# Add custom stopwords (beyond default English stopwords)
custom_stopwords = ["etc", "eg"]

# Preserve domain-specific terms (never filtered)
stopword_allowlist = ["I", "V", "ii", "IV", "vi", "do", "re", "mi"]
```

**Default stopwords include:** `a`, `an`, `the`, `is`, `are`, `was`, `were`, `what`, `when`, `where`, `how`, `why`, etc.

**Query transformation examples:**

```
"what is a cadence"           → "cadence"
"how to write counterpoint"   → "write counterpoint"
"V I resolution"              → "V I resolution" (preserved)
```

### Phrase Search

Use double quotes for exact phrase matching:

```
"perfect authentic cadence"   → Exact phrase match
"leading tone" resolution     → Phrase + additional term
"V I" "IV V"                  → Multiple phrases (OR)
```

**Note:** Stopwords are NOT filtered from phrases to preserve user intent.

---

## Index Management

### Building the Index

**Initial build:**

```bash
music-theory-mcp index
```

**Force rebuild (if content changes):**

```bash
music-theory-mcp index --force
```

**Check index status:**

```bash
music-theory-mcp status
```

### Index Location

By default, the index is stored in `.tantivy-index/` (relative to the server binary).

**Custom index location:**

```toml
[search]
index_path = "/path/to/custom/index"
```

### When to Rebuild

The index needs rebuilding when:
- ✅ First time setup
- ✅ Concept card content changes
- ✅ New concept cards added
- ✅ Concept cards deleted

**Auto-detection:** The server checks a content hash and rebuilds automatically if stale (when `rebuild_on_startup = true` or on-demand via CLI).

---

## Rollback Plan

If you encounter issues with the new backend:

### Temporary Rollback

**1. Switch back to simple backend:**

```toml
[search]
backend = "simple"
```

**2. Restart server:**

```bash
cargo build --release
./target/release/music-theory-mcp serve
```

### Report Issues

If you need to rollback, please report the issue:

- **GitHub Issues:** https://github.com/oxur/ai-music-theory/issues
- **Include:**
  - Error messages or logs
  - Query examples that failed
  - Configuration file (sanitized)
  - Server version (`music-theory-mcp --version`)

---

## Performance Comparison

| Metric | Simple Backend | Tantivy Backend |
|--------|----------------|-----------------|
| **Query latency** | ~50-100ms | ~5-20ms |
| **Memory usage** | ~50MB | ~100-150MB |
| **Disk usage** | 0 (no index) | ~10-50MB (index) |
| **Startup time** | <1s | 1-3s (index load) |
| **Multi-word queries** | ❌ Poor | ✅ Excellent |
| **Natural language** | ❌ Poor | ✅ Excellent |
| **Phrase search** | ❌ No | ✅ Yes |
| **Relevance ranking** | ⚠️ Basic | ✅ BM25 |

**Recommendation:** Use Tantivy for collections >100 cards or any production deployment.

---

## Troubleshooting

### Issue: "Index not found" error

**Cause:** Index not built yet.

**Solution:**

```bash
music-theory-mcp index
```

### Issue: "Feature 'fts' not enabled" warning

**Cause:** Server built without `--features fts`.

**Solution:**

```bash
cargo clean
cargo build --release --features fts
```

### Issue: Search returns zero results

**Possible causes:**

1. **Index stale:** Rebuild index with `music-theory-mcp index --force`
2. **Query too strict:** Try relaxing query mode to "or"
3. **All stopwords:** Query like "what is a" has no searchable terms

**Debug:**

```bash
# Check index status
music-theory-mcp status

# Try simple query first
music-theory-mcp search "cadence"

# Check logs for errors
tail -f /path/to/server.log
```

### Issue: Deprecation warnings in logs

**Cause:** Using `backend = "simple"`.

**Solution:** Migrate to `backend = "tantivy"` following steps above.

**Suppress warning (temporary):**

```toml
[logging]
level = "error"  # Hide warnings (not recommended)
```

---

## FAQ

**Q: Do I need to rebuild the binary?**
A: Yes, you must rebuild with `--features fts` to use the tantivy backend.

**Q: Will my existing queries still work?**
A: Yes, all queries work as before, but with better results.

**Q: How long does index building take?**
A: ~1-5 seconds for 200-500 concept cards.

**Q: Can I use both backends?**
A: Only one backend can be active at a time. Switch via configuration.

**Q: What happens if I don't migrate?**
A: SimpleSearch still works but shows deprecation warnings. It will be removed in v0.3.0.

**Q: Do I need to change my MCP client code?**
A: No, the API is unchanged. Only server-side configuration changes.

**Q: Can I customize stopwords?**
A: Yes, use `custom_stopwords` to add more and `stopword_allowlist` to preserve terms.

**Q: How do I disable phrase search?**
A: You can't disable it, but it only activates with quoted strings. Regular queries unchanged.

**Q: What if my queries are slower?**
A: Tantivy is typically faster than SimpleSearch. Check index status and logs for issues.

---

## Version History

### v0.2.0 (2026-01-27)

- ✅ Changed default backend to tantivy
- ✅ Added multi-word query support (Smart mode)
- ✅ Added stopword filtering
- ✅ Added phrase search support
- ✅ Deprecated SimpleSearch backend
- ✅ No breaking API changes

### v0.1.0 (Previous)

- Initial release with SimpleSearch backend
- Basic substring matching

---

## Support

For questions or issues:

- **Documentation:** See README.md for general usage
- **Design Docs:** See `crates/design/dev/server/` for technical details
- **GitHub Issues:** https://github.com/oxur/ai-music-theory/issues

---

## Summary

**Recommended Action:** Upgrade to tantivy backend for significantly better search quality.

**Steps:**
1. Rebuild with `--features fts`
2. Run `music-theory-mcp index`
3. Restart server
4. Test queries

**Rollback:** Set `backend = "simple"` if needed (temporary).

**Deadline:** Migrate by v0.3.0 release (Q2 2026).
