.PHONY : build
build :
	cargo build

.PHONY : setup
setup :
	rustup component add rustfmt
	rustup component add clippy

.PHONY : format
format :
	cargo fmt --

.PHONY : check-fmt
check-fmt :
	cargo fmt --all -- --check

.PHONY : check-clippy
check-clippy :
	cargo clippy --workspace --all-targets --all-features -- \
			-D warnings \
			-A clippy::upper-case-acronyms

.PHONY : lint
lint : check-fmt check-clippy

.PHONY : install-mdbook
check-fmt :
	cargo install mdbook


.PHONY : test
test :
	@cargo test --workspace --lib
	@cargo test --workspace --doc
	@cargo test --test codegen task_codegen
	@cargo test --no-run --test codegen app_codegen
	@cargo test --no-run --test codegen beat_codegen

.PHONY : run-all-tests
run-all-tests :
	@cargo test --workspace --lib
	@cargo test --workspace --doc
	@cargo test --test codegen task_codegen
	@cargo test --no-run --test codegen app_codegen
	@cargo test --no-run --test codegen beat_codegen

.PHONY : test-docs
build-docs :
	cargo test --doc  --workspace

.PHONY : build-docs
build-docs :
	mdbook build
	cargo doc --workspace --message-format short --no-deps --color always
