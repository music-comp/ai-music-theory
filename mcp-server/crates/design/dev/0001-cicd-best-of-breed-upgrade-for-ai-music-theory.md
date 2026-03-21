# CI/CD Best-of-Breed Upgrade for ai-music-theory

## Context

The `ai-music-theory` CI/CD was set up early and hasn't been updated to match patterns refined across `ecl`, `keystone`, and `ai-kasu`. This plan cherry-picks the best practices from all four repos into a modernized `ai-music-theory` pipeline.

## Best-of-Breed Analysis

| Feature | ai-music-theory (current) | ecl | keystone | ai-kasu | **Recommendation** |
|---------|--------------------------|-----|----------|---------|---------------------|
| Caching | 5x manual `actions/cache@v4` | `Swatinem/rust-cache@v2` (smart) | `Swatinem/rust-cache@v2` | 5x manual `actions/cache@v4` | **Swatinem** (ecl/keystone) |
| Checkout | `actions/checkout@v4` | `actions/checkout@v6` | `actions/checkout@v4` | `actions/checkout@v4` | **@v6** (ecl) |
| Cargo env vars | none | 4 env vars (INCREMENTAL, DEBUG, COLOR, RETRY) | 2 (COLOR, BACKTRACE) | none | **ecl's 4 vars** |
| Cache save strategy | always | main-only save (`save-if`) | always | always | **main-only** (ecl) — saves quota |
| cache-on-failure | no | yes | n/a (Swatinem default) | no | **yes** (ecl) |
| Format check | inside `make lint` (debug only) | standalone step, both modes | standalone job | inside `make lint` | **standalone step, both modes** (ecl) |
| Clippy | inside `make lint` (debug only) | standalone step, both modes | standalone job | inside `make lint` | **standalone step, both modes** (ecl) |
| Build flags | `make build` (no --locked) | `--locked --all-features --workspace` | `--all-targets --locked` | `make build` | **--locked** (ecl/keystone) |
| Step ordering | deps → build → lint → docs → test | deps → fmt → build → clippy → test | fmt → clippy → build → test | deps → lint → build → test → docs | **ecl pattern**: fmt first (instant), build, clippy (reuses artifacts), test |
| `cd mcp-server` | every step | n/a (root project) | n/a | n/a | **`working-directory` default** |
| Docs build | debug only | not in CI | not in CI | in CI | **keep, debug only** (unique to this project) |
| Concurrency | none | none | none | none | **add concurrency group** (cancel stale PR runs) |
| Toolchain components | none specified | `clippy, rustfmt` | per-job components | none | **`clippy, rustfmt`** (ecl) |

## Changes to Make

### File: `.github/workflows/cicd.yml` — Full rewrite

**Adopt from ecl:**
- `Swatinem/rust-cache@v2` replacing 5 manual cache blocks
- `shared-key: "ci-${{ matrix.mode }}"` for mode-aware caching
- `save-if: ${{ github.ref == 'refs/heads/main' }}` to save cache quota
- `cache-on-failure: true` to preserve partial builds
- `actions/checkout@v6`
- 4 cargo env vars: `CARGO_INCREMENTAL: 0`, `CARGO_PROFILE_TEST_DEBUG: 0`, `CARGO_TERM_COLOR: always`, `CARGO_NET_RETRY: 10`
- Toolchain with `components: clippy, rustfmt`
- Step order: fmt → build → clippy → test (clippy/test reuse build artifacts)
- Inline cargo commands with `--locked --all-features --workspace` instead of delegating to Make for core build/lint/test (Make targets do extra work like copying binaries that CI doesn't need)

**Adopt from keystone:**
- Concurrency group to cancel stale PR runs:
  ```yaml
  concurrency:
    group: ${{ github.workflow }}-${{ github.ref }}
    cancel-in-progress: ${{ github.ref != 'refs/heads/main' }}
  ```

**Keep from current ai-music-theory:**
- debug/release matrix with `fail-fast: false`
- Scheduled daily build (`cron: "20 4 * * *"`)
- `workflow_dispatch` trigger
- `make check-deps` (debug only) — valuable dependency freshness check
- `make docs` (debug only) — validates doc generation
- Separate `release-caches.yml` workflow (unchanged)

**Add `working-directory` default** to eliminate repeated `cd mcp-server`:
```yaml
defaults:
  run:
    working-directory: mcp-server
```

### File: `.github/workflows/release-caches.yml` — Minor updates

- Update `actions/checkout@v4` → `@v6`
- Replace 3 manual cache blocks with `Swatinem/rust-cache@v2`
- Add `working-directory: mcp-server` default
- Add cargo env vars for consistency

## Proposed cicd.yml

```yaml
name: CI/CD

on:
  workflow_dispatch:
  push:
  pull_request:
  schedule:
  - cron: "20 4 * * *"

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: ${{ github.ref != 'refs/heads/main' }}

env:
  CARGO_INCREMENTAL: 0
  CARGO_PROFILE_TEST_DEBUG: 0
  CARGO_TERM_COLOR: always
  CARGO_NET_RETRY: 10

defaults:
  run:
    working-directory: mcp-server

jobs:
  build:
    name: Build and Test (${{ matrix.mode }})
    runs-on: ubuntu-latest

    strategy:
      fail-fast: false
      matrix:
        mode: [debug, release]

    steps:
    - uses: actions/checkout@v6

    - name: Install Rust toolchain
      uses: dtolnay/rust-toolchain@stable
      with:
        components: clippy, rustfmt

    - uses: Swatinem/rust-cache@v2
      with:
        shared-key: "ci-${{ matrix.mode }}"
        cache-on-failure: true
        save-if: ${{ github.ref == 'refs/heads/main' }}
        workspaces: mcp-server

    - name: Check dependencies
      if: matrix.mode == 'debug'
      run: make check-deps

    - name: Check formatting
      run: cargo fmt --all -- --check

    - name: Build
      run: cargo build --locked --all-features --workspace ${{ matrix.mode == 'release' && '--release' || '' }}

    - name: Lint
      run: cargo clippy --locked --all-features --workspace ${{ matrix.mode == 'release' && '--release' || '' }} -- -D warnings

    - name: Run tests
      run: cargo test --locked --all-features --workspace ${{ matrix.mode == 'release' && '--release' || '' }}

    - name: Build docs
      if: matrix.mode == 'debug'
      run: cargo doc --locked --all-features --workspace --no-deps
```

## Proposed release-caches.yml

```yaml
name: Release Caches

on:
  release:
    types: [published]
  workflow_dispatch:
    inputs:
      tag:
        description: 'Release tag (e.g., 0.4.0)'
        required: true

env:
  CARGO_INCREMENTAL: 0
  CARGO_TERM_COLOR: always
  CARGO_NET_RETRY: 10

defaults:
  run:
    working-directory: mcp-server

jobs:
  package-caches:
    name: Build and Upload Cache Packages
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v6

    - name: Install Rust toolchain
      uses: dtolnay/rust-toolchain@stable

    - uses: Swatinem/rust-cache@v2
      with:
        shared-key: "release"
        workspaces: mcp-server

    - name: Build release binary
      run: cargo build --release --features full

    - name: Build all indexes
      run: |
        ./target/release/music-theory-mcp index --force
        ./target/release/music-theory-mcp graph build
        ./target/release/music-theory-mcp vectordb build --force

    - name: Package caches
      run: ./target/release/music-theory-mcp cache package all --output ../dist

    - name: Generate checksums
      working-directory: dist
      run: |
        for f in *.tar.gz; do
          shasum -a 256 "$f" > "$f.sha256"
        done
        echo "Cache packages:"
        ls -lh

    - name: Upload to release
      if: github.event_name == 'release'
      uses: softprops/action-gh-release@v2
      with:
        files: dist/*
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

    - name: Upload as artifact (for workflow_dispatch)
      if: github.event_name == 'workflow_dispatch'
      uses: actions/upload-artifact@v4
      with:
        name: cache-packages
        path: dist/*
```

## Verification

1. Push to a feature branch → verify CI triggers, both debug/release matrix jobs run
2. Confirm caching works: second push should be noticeably faster
3. Confirm format/clippy/test all pass
4. Open a PR → verify concurrency cancels stale runs on force-push
5. Merge to main → verify cache is saved (check Actions cache page)
6. `release-caches.yml`: trigger via `workflow_dispatch` to verify it still builds and packages
