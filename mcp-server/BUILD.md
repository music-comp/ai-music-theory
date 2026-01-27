# Build System Guide

This document describes the build system and Makefile targets for the Music Theory MCP Server.

## Quick Start

```bash
# Build with simple search (smaller, faster build)
make build

# Build with FTS support (larger, includes Tantivy)
make build-fts

# Build release binaries (recommended for production)
make build-release-fts

# Run all checks (both configurations)
make check-all
```

## Feature Flags

The server supports two search backends via feature flags:

### Default (Simple Search)
- **Build**: `cargo build` or `make build`
- **Size**: ~2.8M (release)
- **Search**: Linear scan (O(n))
- **Best for**: <500 concept cards
- **Always available**: No external dependencies

### FTS Feature (Tantivy Full-Text Search)
- **Build**: `cargo build --features fts` or `make build-fts`
- **Size**: ~6.6M (release, +131%)
- **Search**: Indexed search (O(log n))
- **Best for**: 500+ concept cards
- **Optional**: Requires `--features fts`

## Build Targets

### Development Builds

```bash
make build              # Simple search (debug)
make build-fts          # With FTS (debug)
```

### Release Builds

```bash
make build-release      # Simple search (optimized)
make build-release-fts  # With FTS (optimized)
make build-both         # Both configs (for comparison)
```

### Binary Size Comparison

```bash
make build-both         # Build both configurations
make size-compare       # Compare binary sizes
```

Example output:
```
Binary Size Comparison
══════════════════════

music-theory-mcp:
  Simple:  2.8M
  FTS:     6.6M
  Diff:    +3.8M (+131%)
```

## Testing

### Run Tests

```bash
make test               # Tests without FTS
make test-fts           # Tests with FTS
make test-all           # Both configurations
```

### Test Coverage

```bash
make coverage           # Generate coverage report (FTS)
make coverage-html      # Generate HTML report
```

## Code Quality

### Linting

```bash
make lint               # Clippy + format check (both configs)
make format             # Format all code
```

The `lint` target runs clippy with strict warnings (`-D warnings`) on both configurations.

### Comprehensive Checks

```bash
make check              # Build + lint + test (simple)
make check-fts          # Build + lint + test (FTS)
make check-all          # Both configurations
```

## CLI Tools (FTS Feature Required)

The server includes CLI commands for index management when built with FTS:

### Build FTS Index

```bash
make index              # Build index (if needed)
make index-force        # Force rebuild
```

These targets automatically build the binary with FTS if needed.

### Check Index Status

```bash
make status             # Show index statistics
```

Example output:
```
Index Status:
  Location:     .tantivy-index
  Documents:    187
  Status:       ✓ Current
```

## Cleaning

```bash
make clean              # Clean bin/ directory
make clean-all          # Full cargo clean
```

## Build Information

```bash
make info               # Show build configuration
make check-tools        # Verify required tools
```

## CI/CD Integration

For automated builds, use the comprehensive check targets:

```bash
# Simple configuration only
make check

# Both configurations (recommended)
make check-all

# With coverage
make check-all coverage
```

## Advanced Usage

### Custom Build Mode

```bash
make build MODE=release         # Release build
make build-fts MODE=release     # Release with FTS
```

### Parallel Testing

```bash
# Run tests in parallel (both configs)
make test & make test-fts
```

### Development Workflow

```bash
# Recommended development workflow:
make format             # Format code
make lint               # Check for issues
make test-all           # Run all tests
make build-both         # Build both configs
make size-compare       # Verify sizes
```

## Troubleshooting

### Binary Size Issues

If binaries are larger than expected:

```bash
make clean-all          # Full clean
make build-release      # Rebuild
```

Check actual sizes:
```bash
make build-both
make size-compare
```

### Test Failures

Run tests with output:
```bash
cargo test -- --nocapture                    # Simple
cargo test --features fts -- --nocapture     # FTS
```

### Clippy Warnings

Fix clippy issues:
```bash
cargo clippy --fix --features fts --allow-dirty
make format
```

## Performance Notes

### Build Times
- Simple build: ~5-10 seconds (incremental)
- FTS build: ~15-20 seconds (incremental)
- Clean release: ~30-45 seconds (simple), ~45-60 seconds (FTS)

### Test Times
- Simple tests: ~0.2 seconds (516 tests)
- FTS tests: ~2-3 seconds (659 tests)

### Binary Sizes (Release)
- Simple: ~2.8M (stripped)
- FTS: ~6.6M (stripped)
- Difference: +3.8M (+131%)

The FTS feature adds Tantivy and its dependencies, which account for the size increase.

## Environment Variables

```bash
RUST_LOG=debug make build-fts   # Build with debug logging
CARGO_INCREMENTAL=0 make build  # Disable incremental compilation
```

## See Also

- [README.md](crates/server/README.md) - General usage and configuration
- [CLAUDE.md](crates/server/CLAUDE.md) - Development guidelines
- Design doc 0006 - Feature gating architecture
