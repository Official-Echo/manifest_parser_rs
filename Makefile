.PHONY: all fmt clippy test build run check clean doc install example

all: fmt clippy test build

fmt:
	cargo fmt --all

clippy:
	cargo clippy -- -D warnings
	cargo clippy --tests -- -D warnings

test:
	cargo test --all-features
	cargo test --no-default-features

build:
	cargo build --release
	cargo build

example:
	@echo ===Running example manifest parser...=================
	cargo run -- parse Cargo.toml
	@echo ===Getting section...=================================
	cargo run -- get-by-section Cargo.toml package
	@echo ===Getting specific value...==========================
	cargo run -- get-by-key Cargo.toml dependencies pest

check:
	cargo check --all-targets --all-features

doc:
	cargo doc --no-deps --document-private-items
	cargo doc --open

clean:
	cargo clean
	rm -rf target/

install:
	cargo install --path .

run:
	cargo run -- parse Cargo.toml

lint: clippy
docs: doc
tests: test
b: build
r: run