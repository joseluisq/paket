run:
	@echo "Running application..."
	@rustc -vV
	@cargo run -- --help
.PHONY: run

build:
	@echo "Building application..."
	@rustc -vV
	@cargo build --release
.PHONY: build

test:
	@echo "Testing library..."
	@rustc -vV
	@cargo test --lib
.PHONY: test

fmt:
	@echo "Fixing and formatting source files..."
	@cargo fix --allow-dirty
	@cargo fmt --all
.PHONY: fmt
