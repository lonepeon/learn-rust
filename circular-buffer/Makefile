test: test-unit test-format test-staticcheck

test-unit:
	cargo test

test-format:
	cargo fmt --check

test-staticcheck:
	cargo clippy --all-targets --all-features -- -D warnings

