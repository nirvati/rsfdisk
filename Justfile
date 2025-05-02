alias d   := doc
alias t   := test
alias tu  := test-unit
alias td  := test-doc
alias tdu := test-doc-unit
alias ta  := test-all

packages := "-p rsfdisk-sys -p rsfdisk"
dependencies := ""

# Build the library
default:
	cargo build {{packages}}

# Build the library documentation
doc:
	cargo doc --no-deps {{packages}} {{dependencies}}

# Publish crate to crates.io
do-publish:
 cargo publish

# Dry run cargo publish
publish: test-all doc
  cargo publish --dry-run

# Run all unit/integration tests
test:
	cargo nextest run {{packages}}

# Run unit test named TESTNAME
test-unit TESTNAME:
	cargo nextest run {{TESTNAME}} {{packages}}

# Run doc tests
test-doc:
	cargo test --doc {{packages}}

# Run doc tests containing the string [TESTNAME]
test-doc-unit TESTNAME:
	cargo test --doc {{TESTNAME}} {{packages}}

# Run unit, integration, and doc tests
test-all: test test-doc
