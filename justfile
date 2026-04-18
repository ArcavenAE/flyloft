# flyloft — curated retrieval substrate for human-AI teams

# Default: list recipes
default:
    @just --list

# ─── Build & Run ───────────────────────────────────────

# Build the workspace
build:
    cargo build --workspace

# Build release binaries
build-release:
    cargo build --workspace --release

# Run the CLI with arguments
run *args:
    cargo run --bin flyloft -- {{args}}

# Run the MCP server
mcp *args:
    cargo run --bin flyloft-mcp -- {{args}}

# ─── Test ──────────────────────────────────────────────

# Run all tests
test:
    cargo test --workspace

# Run doc tests
test-doc:
    cargo test --workspace --doc

# Run a specific test by name
test-one name:
    cargo test --workspace -- {{name}}

# ─── Quality Checks ───────────────────────────────────

# Pre-commit check (matches CI)
check: check-fmt check-clippy check-deny

# Full check including extras
check-all: check check-toml

# Check formatting (nightly rustfmt)
check-fmt:
    cargo +nightly fmt --all -- --check

# Run clippy with warnings as errors
check-clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Check licenses and advisories
check-deny:
    cargo deny check advisories licenses bans

# Check TOML formatting
check-toml:
    taplo fmt --check

# Alias
lint: check

# ─── Formatting ───────────────────────────────────────

# Format Rust code (nightly)
fmt:
    cargo +nightly fmt --all

# Format TOML files
fmt-toml:
    taplo fmt

# Format everything
fmt-all: fmt fmt-toml

# ─── CI Mirror ────────────────────────────────────────

# Run all CI jobs locally (fail-fast order)
ci: check-fmt check-clippy build check-deny test

# ─── Development ──────────────────────────────────────

# Watch mode: check + test on change
watch:
    cargo watch -x 'check --workspace' -x 'test --workspace --lib'

# Generate docs
doc:
    cargo doc --workspace --no-deps

# Generate and open docs
doc-open:
    cargo doc --workspace --no-deps --open

# Clean build artifacts
clean:
    cargo clean

# ─── Setup ────────────────────────────────────────────

# First-time environment setup
setup:
    rustup component add clippy
    rustup toolchain install nightly --component rustfmt
    cargo install cargo-watch cargo-deny
    @echo "Optional: brew install taplo (TOML formatter)"
    @echo "Optional: cargo install cargo-nextest (parallel tests)"

# Install git hooks
install-hooks:
    lefthook install

# ─── Maintenance ──────────────────────────────────────

# Security audit
audit:
    cargo audit
