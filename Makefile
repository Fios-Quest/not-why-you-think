.PHONY: %

default: check

pre-check:
	cargo fmt
	dx fmt

check: pre-check
	cargo check
	cargo build
	#cargo test
	cargo fmt --check
	#cargo clippy -- -D warnings
	#dx fmt --check