# Build the library
all:
	cargo build

# Build the library documentation
doc:
	cargo doc --no-deps -p rsfdisk-sys -p rsfdisk

# Publish crate to crates.io
do-publish:
	cargo publish

# Dry run cargo publish
publish: test-all doc
	cargo publish --dry-run

# Run unit/integration tests
test:
	cargo nextest run

# Run doc tests
test-doc:
	cargo test --doc

# Run all tests
test-all: test test-doc
