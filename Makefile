.PHONY: check check-rust check-python check-go check-js check-docs check-full
.NOTPARALLEL:

VENV ?= $(CURDIR)/.venv

# Go 内嵌的 wasm 随仓库分发，panic 位置串里的本机路径（工具链、cargo 缓存、仓库目录）换成固定前缀，
# 产物不带构建机的目录与用户名
WASIP1_RUSTFLAGS := --remap-path-prefix=$(shell rustc --print sysroot)=/rust \
	--remap-path-prefix=$(or $(CARGO_HOME),$(HOME)/.cargo)=/cargo \
	--remap-path-prefix=$(CURDIR)=/src

check: check-rust check-python check-go check-js check-docs

check-rust:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings
	cargo check --features schemars
	RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
	cargo build --release --locked
	cargo test --locked
	cargo test --release --lib calendar -- --ignored
	cargo test --release --test calendar_api -- --ignored

check-python:
	VIRTUAL_ENV="$(VENV)" PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 "$(VENV)/bin/maturin" develop --features python --locked
	"$(VENV)/bin/pytest" python/tests/ -q

check-go:
	cd go/iztro && go test -race ./...
	RUSTFLAGS="$(WASIP1_RUSTFLAGS)" cargo build --release --locked --target wasm32-wasip1
	cp target/wasm32-wasip1/release/x_iztro.wasm go/iztro/x_iztro.wasm
	cd go/iztro && go test -race ./...
	cd examples/go && go build -o ../../target/x-iztro-go-example .

check-js:
	cd js && npm run check
	cd tests/js-types && npm ci && npm run check
	cd examples/js && npm ci && node main.mjs > /dev/null

check-docs:
	cd docs && ./node_modules/.bin/next typegen && npm run types:check
	cd docs && npm run build

check-full: check
	cargo test --release --test golden_tier3 -- --ignored
	cargo test --release --test full_domain -- --ignored
