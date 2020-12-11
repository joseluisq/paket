PKG_TARGET=x86_64-unknown-linux-musl
PKG_TARGET_DARWIN=x86_64-apple-darwin
RUST_VERSION ?= $(shell rustc --version | cut -d ' ' -f2)

PKG_BIN_PATH=./bin
PKG_TMP_PATH=/tmp

PKG_NAME=$(shell cat Cargo.toml | sed -n 's/name = "\([^}]*\)"/\1/p' | head -n1)
PKG_TAG=$(shell cat Cargo.toml | sed -n 's/version = "\([^}]*\)"/\1/p' | head -n1)

PKG_RELEASE_NAME=$(PKG_NAME)-v$(PKG_TAG)-$(PKG_TARGET)
PKG_RELEASE_NAME_DARWIN=$(PKG_NAME)-v$(PKG_TAG)-$(PKG_TARGET_DARWIN)

PKG_TMP_BIN_PATH=$(PKG_TMP_PATH)/$(PKG_RELEASE_NAME)
PKG_TMP_BIN_PATH_DARWIN=$(PKG_TMP_PATH)/$(PKG_RELEASE_NAME_DARWIN)


run:
	@echo "Running application..."
	@rustc -vV
	@cargo run -- --help
.PHONY: run

build:
	@echo "Building application..."
	@rustc -vV
	@cargo build --release --target x86_64-unknown-linux-musl
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

docker.build:
	@docker build -t joseluisq/paket:latest -f docker/alpine/Dockerfile .
.PHONY: docker.build

docker.run:
	@docker run -it --rm joseluisq/paket:latest -h
.PHONY: docker.run


#######################################
########## Production tasks ###########
#######################################

# Create a release build
prod.release.linux:
	@rustc -vV
	@echo "Compiling release binary for $(PKG_TARGET)..."
	@cargo build --release --target $(PKG_TARGET)
	@du -sh ./target/$(PKG_TARGET)/release/$(PKG_NAME)

	@echo "Shrinking binary release..."
	@strip ./target/$(PKG_TARGET)/release/$(PKG_NAME)
	@du -sh ./target/$(PKG_TARGET)/release/$(PKG_NAME)
.PHONY: prod.release.linux

prod.release.darwin:
	@rustc -vV
	@echo "Compiling release binary for $(PKG_TARGET_DARWIN)..."
	@cargo build --release --target $(PKG_TARGET_DARWIN)
	@du -sh ./target/$(PKG_TARGET_DARWIN)/release/$(PKG_NAME)

	@echo "Shrinking binary release..."
	@x86_64-apple-darwin15-strip ./target/$(PKG_TARGET_DARWIN)/release/$(PKG_NAME)
	@du -sh ./target/$(PKG_TARGET_DARWIN)/release/$(PKG_NAME)
.PHONY: prod.release.darwin
