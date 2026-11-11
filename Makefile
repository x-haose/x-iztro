.PHONY: check check-rust check-python check-go check-docs check-full
.NOTPARALLEL:

VENV ?= $(CURDIR)/.venv

check: check-rust check-python check-go check-docs

check-rust:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings
	cargo check --features schemars
	RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
	cargo build --release --locked
	cargo test --locked
	cargo test --release --lib calendar -- --ignored

check-python:
	VIRTUAL_ENV="$(VENV)" PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 "$(VENV)/bin/maturin" develop --features python --locked
	"$(VENV)/bin/pytest" python/tests/ -q

check-go:
	cd go/iztro && go test -race ./...
	cargo build --release --locked --target wasm32-wasip1
	cp target/wasm32-wasip1/release/x_iztro.wasm go/iztro/x_iztro.wasm
	cd go/iztro && go test -race ./...
	cd examples/go && go build -o ../../target/x-iztro-go-example .

check-docs:
	cd docs && ./node_modules/.bin/next typegen && npm run types:check
	cd docs && npm run build

check-full: check
	cargo test --release --test golden_tier3 -- --ignored
	cargo test --release --test full_domain -- --ignored
