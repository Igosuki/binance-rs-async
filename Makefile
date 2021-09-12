VENV := precommit_venv
HOOKS := .git/hooks/pre-commit

# PRE-COMMIT HOOKS

$(VENV): .requirements-precommit.txt
	virtualenv -p python3 $(VENV)
	$(VENV)/bin/pip install -r .requirements-precommit.txt

.PHONY: env
env: $(VENV)

.PHONY: clean-env
clean-env:
	rm -rf $(VENV)

## Cargo
CARGO_BIN ?= `which cargo`
TARGET_PATH ?= `pwd`/target/release
BIN_VERSION ?= 0.1.0
BIN_NAME ?= voik
BIN_PATH ?= $(TARGET_PATH)/$(BIN_NAME)

## Testing
FUNZZY_BIN ?= `which funzzy`

PWD ?= `pwd`

HOME ?= `echo $HOME`

$(HOOKS): $(VENV) .pre-commit-config.yaml
	$(VENV)/bin/pre-commit install -f --install-hooks
	@$(CARGO_BIN) fmt --help > /dev/null || rustup component add rustfmt
	@$(CARGO_BIN) clippy --help > /dev/null || rustup component add clippy
	@$(CARGO_BIN) readme --help > /dev/null || cargo install cargo-readme

.PHONY: install-hooks
install-hooks: $(HOOKS)

.PHONY: clean-hooks
clean-hooks:
	rm -rf $(HOOKS)


.PHONY: build
build:
	@$(CARGO_BIN) build

.PHONY: build_all
build_all:
	@$(CARGO_BIN) build --all-features

.PHONY: test
test_all: ## Tests all features
	@$(CARGO_BIN) test --all-features

test: ## Tests all features
	RUST_LOG=info BITCOINS_REPO=$(shell pwd) $(CARGO_BIN) test --all-targets -- --skip coinnect_tests --skip coinbase_tests --nocapture

.PHONY: coverage
coverage: ## Tests all features
	@$(CARGO_BIN) tarpaulin -v

.PHONY: test_watcher ## Starts funzzy, test watcher, to run the tests on every change
test_watcher:
	@$(FUNZZY_BIN)

.PHONY: bench
bench:
	@$(CARGO_BIN) bench

.PHONY: profile
profile:
	@$(CARGO_BIN) flamegraph --dev --bin=trader --features flame_it

.PHONY: lint
lint:
	@$(CARGO_BIN) clippy --all-targets --all-features -Z unstable-options -- -Dclippy::all -Dunused_imports

.PHONY: lintfix
lintfix:
	@$(CARGO_BIN) clippy --fix --all-targets --all-features -Z unstable-options -- -Dclippy::all -Dunused_imports

.PHONY: clean-lint
clean-lint:
	find . -type f -name *.rs.bk -delete

## alias rust-musl-builder-nightly='docker run --cpus=$(nproc) --rm -it --user rust $MUSL_FLAGS -v "$HOME/.cargo/git":/home/rust/.cargo/git -v "$(pwd)/cargo-registry":/home/rust/.cargo/registry -v "$(pwd)/cargo-target":/home/rust/src/target -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder:nightly-2020-06-17'
release:
	$(rust-musl-builder-nightly) cargo build --release --target=x86_64-unknown-linux-gnu

build_test:
	@$(CARGO_BIN) test --message-format=json-diagnostic-rendered-ansi --color=always --no-run --lib $(TEST_NAME) --manifest-path $(MANIFEST_PATH)

