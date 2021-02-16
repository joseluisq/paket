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


linux:
	@docker run --rm -it \
		-v $(PWD):/root/src/paket \
		-v cargo-git:/root/.cargo/git \
		-v cargo-registry:/root/.cargo/registry \
		-v cargo-paket-target:/root/src/paket/target \
\
		--workdir /root/src/paket \
		joseluisq/rust-linux-darwin-builder:$(RUST_VERSION) \
\
		bash -c "\
			echo Building Linux release binary... && \
			rustc -vV && \
			cargo build --release --target $(PKG_TARGET) && \
			du -sh ./target/$(PKG_TARGET)/release/$(PKG_NAME) && \
			mkdir -p release && \
			cp -rf ./target/$(PKG_TARGET)/release/$(PKG_NAME) release/$(PKG_NAME)-linux && \
			echo \"Shrinking Linux binary file...\" && \
			strip release/$(PKG_NAME)-linux && \
			du -sh ./release/$(PKG_NAME)-linux"
.PHONY: linux

darwin:
	@docker run --rm -it \
		-v $(PWD):/root/src/paket \
		-v cargo-git:/root/.cargo/git \
		-v cargo-registry:/root/.cargo/registry \
		-v cargo-paket-target:/root/src/paket/target \
\
		--workdir /root/src/paket \
		joseluisq/rust-linux-darwin-builder:$(RUST_VERSION) \
\
		bash -c "\
			echo Building Darwin release binary... && \
			rustc -vV && \
			cargo build --release --target $(PKG_TARGET_DARWIN) && \
			du -sh ./target/$(PKG_TARGET_DARWIN)/release/$(PKG_NAME) && \
			mkdir -p release && \
			cp -rf ./target/$(PKG_TARGET_DARWIN)/release/$(PKG_NAME) release/$(PKG_NAME)-darwin && \
			echo \"Shrinking Darwin binary file...\" && \
			x86_64-apple-darwin20.2-strip release/$(PKG_NAME)-darwin && \
			du -sh ./release/$(PKG_NAME)-darwin"
.PHONY: darwin


# Create a release build
prod.release.linux:
	@rustc -vV
	@echo "Compiling Linux release binary for $(PKG_TARGET)..."
	@cargo build --release --target $(PKG_TARGET)
	@du -sh ./target/$(PKG_TARGET)/release/$(PKG_NAME)

	@echo "Shrinking Linux release binary..."
	@strip ./target/$(PKG_TARGET)/release/$(PKG_NAME)
	@du -sh ./target/$(PKG_TARGET)/release/$(PKG_NAME)
	@mkdir -p $(PKG_BIN_PATH)/$(PKG_TARGET)/
	@cp ./target/$(PKG_TARGET)/release/$(PKG_NAME) $(PKG_BIN_PATH)/$(PKG_TARGET)/
.PHONY: prod.release.linux

prod.release.darwin:
	@rustc -vV
	@echo "Compiling Darwin release binary for $(PKG_TARGET_DARWIN)..."
	@cargo build --release --target $(PKG_TARGET_DARWIN)
	@du -sh ./target/$(PKG_TARGET_DARWIN)/release/$(PKG_NAME)

	@echo "Shrinking Darwin release binary..."
	@x86_64-apple-darwin20.2-strip ./target/$(PKG_TARGET_DARWIN)/release/$(PKG_NAME)
	@du -sh ./target/$(PKG_TARGET_DARWIN)/release/$(PKG_NAME)
	@mkdir -p $(PKG_BIN_PATH)/$(PKG_TARGET_DARWIN)/
	@cp ./target/$(PKG_TARGET_DARWIN)/release/$(PKG_NAME) $(PKG_BIN_PATH)/$(PKG_TARGET_DARWIN)/
.PHONY: prod.release.darwin
