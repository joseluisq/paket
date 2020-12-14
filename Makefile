PKG_TARGET=x86_64-unknown-linux-musl
PKG_TARGET_DARWIN=x86_64-apple-darwin
RUST_VERSION ?= $(shell rustc --version | cut -d ' ' -f2)

PKG_BIN_PATH=./bin

PKG_NAME=$(shell cat Cargo.toml | sed -n 's/name = "\([^}]*\)"/\1/p' | head -n1)
PKG_TAG=$(shell cat Cargo.toml | sed -n 's/version = "\([^}]*\)"/\1/p' | head -n1)

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

pipeline-prod:
	@drone exec \
		--trusted \
		--privileged \
		--event=tag \
		--exclude=test \
		--exclude=publish-linux-local \
		--exclude=publish-linux-dockerhub \
		--exclude=github-release \
		--pipeline=production
.PHONY: pipeline-prod


#######################################
########## Production tasks ###########
#######################################

# Create a release build
prod.release.linux:
	@rustc -vV
	@echo "Compiling release binary for $(PKG_TARGET)..."
	@cargo build --release --target $(PKG_TARGET)
	@du -sh ./target/$(PKG_TARGET)/release/$(PKG_NAME)

	@echo "Shrinking release binary..."
	@strip ./target/$(PKG_TARGET)/release/$(PKG_NAME)
	@du -sh ./target/$(PKG_TARGET)/release/$(PKG_NAME)
	@mkdir -p $(PKG_BIN_PATH)/$(PKG_TARGET)/
	@cp ./target/$(PKG_TARGET)/release/$(PKG_NAME) $(PKG_BIN_PATH)/$(PKG_TARGET)/
.PHONY: prod.release.linux

prod.release.darwin:
	@rustc -vV
	@echo "Compiling release binary for $(PKG_TARGET_DARWIN)..."
	@cargo build --release --target $(PKG_TARGET_DARWIN)
	@du -sh ./target/$(PKG_TARGET_DARWIN)/release/$(PKG_NAME)

	@echo "Shrinking release binary..."
	@x86_64-apple-darwin15-strip ./target/$(PKG_TARGET_DARWIN)/release/$(PKG_NAME)
	@du -sh ./target/$(PKG_TARGET_DARWIN)/release/$(PKG_NAME)
	@mkdir -p $(PKG_BIN_PATH)/$(PKG_TARGET_DARWIN)/
	@cp ./target/$(PKG_TARGET_DARWIN)/release/$(PKG_NAME) $(PKG_BIN_PATH)/$(PKG_TARGET_DARWIN)/
.PHONY: prod.release.darwin
