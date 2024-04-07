#!/usr/bin/env just --justfile

default: build

# Build nostr CLI (release)
build:
	cargo build --release --all-features

# Check format and crates
check: check-fmt check-crate

# Format the code and execute some checks
precommit: fmt
    cargo check
    cargo test
    cargo clippy

# Format the entire Rust code
fmt:
	@bash contrib/scripts/check-fmt.sh

# Check if the Rust code is formatted
check-fmt:
	@bash contrib/scripts/check-fmt.sh check

# Check crate
check-crate:
	@bash contrib/scripts/check-crate.sh

# Remove artifacts that cargo has generated
clean:
	cargo clean

# Count the lines of codes of this project
loc:
	@echo "--- Counting lines of .rs files (LOC):" && find crates/ bindings/ -type f -name "*.rs" -not -path "*/target/*" -exec cat {} \; | wc -l