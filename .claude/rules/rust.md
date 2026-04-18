# Rust Coding Rules

## Safety

```rust
// Every binary and library crate
#![forbid(unsafe_code)]
```

- No `unwrap()` in production code — use `?` or `expect()` with actionable message
- `unwrap()` is acceptable in tests

## Type Design

- **Newtypes for IDs:** `BattenId(String)`, `LineSetId(String)`, `CueId(String)` — prevents mixing ID types
- **Validated constructors at trust boundaries:** `new()` validates (CLI input, file parsing, catalog responses)
- **`new_unchecked()` for tests and trusted internal sources**
- **`#[non_exhaustive]` on enums that will grow** — forces callers to handle future variants
- **Private fields with getters** on types where invariants must hold

## Error Handling

```rust
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum FlyloftError {
    // Semantic variants per domain
}

pub type Result<T> = std::result::Result<T, FlyloftError>;
```

- Use `thiserror` for error enums — structured variants, not string bags
- Define `pub type Result<T>` per crate for ergonomics
- `Display` impl is for user-facing output in a CLI — be clear and actionable

## Module Structure (crates)

```
crates/
  flyloft-core/       # Library: types, traits, storage, retrieval pipeline
    src/
      lib.rs          # Public API re-exports
      error.rs        # FlyloftError enum
      batten.rs       # Batten + BattenContent (held | cataloged)
      line_set.rs
      cue.rs
      grid.rs
      catalog.rs      # Catalog trait + registry
      provenance.rs
  flyloft-cli/        # Binary: the flyperson's CLI
    src/main.rs
  flyloft-mcp/        # Binary: MCP server
    src/main.rs
```

- `flyloft-core` is the library; the binaries call into it
- Model types mirror the YAML schema exactly (serde does the mapping)
- Adapter crates (`flyloft-catalog-<name>/`) live as separate workspace members so dependencies don't infect the core

## Dependencies

- Workspace-level dependency declarations in root `Cargo.toml`
- Edition 2024, MSRV 1.85+
- Use `cargo clippy --workspace -- -D warnings` — warnings are errors

## Testing

- Unit: `#[cfg(test)] mod tests {}` in same file
- Integration: `tests/` directory per crate, named by feature
- Test names as documentation: `batten_id_rejects_empty_string()`, not `test_1()`
- Test boundaries: empty, missing fields, invalid confidence values, broken catalog pointers
- Async tests use `#[tokio::test]`
- Snapshot tests with `insta` for retrieval output
- `t.Cleanup()` analogue: `defer` via drop impls or test-scoped temp dirs

## Async Conventions

- Use `tokio` runtime; single runtime per binary
- `#[async_trait]` for traits that need async methods (catalog adapters)
- Prefer `tokio::task::spawn_blocking` for CPU-bound work (embedding, reranking)
- `tracing` for structured async logging; `tracing-subscriber` in binaries only

## Common AI mistakes to avoid

1. Don't add unused trait parameters "for future use" — YAGNI
2. Don't panic in library code — return `Result`
3. Don't use `unwrap()` outside tests
4. Don't duplicate error variants — prefer `#[from]` conversions
5. Don't expose `String` when a newtype clarifies intent (`BattenId`, `CatalogId`)
6. Don't block the async runtime — use `spawn_blocking` for CPU work
7. Don't hold locks across `.await` points
8. Don't ignore `cargo clippy` warnings — they catch real bugs
