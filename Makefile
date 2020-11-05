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


#######################################
########## Production tasks ###########
#######################################

# Compile release binary 
define build_release =
	set -e
	set -u

	echo "Compiling application..."
	rustc -vV
	echo "Compiling release binary for $(PKG_TARGET)..."
	cargo build --release --target $(PKG_TARGET)
	echo
	echo "Compiling release binary for $(PKG_TARGET_DARWIN)..."
	cargo build --release --target $(PKG_TARGET_DARWIN)
	echo "Release builds completed!"
endef

# Shrink a release binary size
define build_release_shrink =
	set -e
	set -u

	echo "Copying release binaries..."

	mkdir -p $(PKG_BIN_PATH)

	# Linux
	mkdir -p $(PKG_TMP_BIN_PATH)
	cp -rf ./target/$(PKG_TARGET)/release/$(PKG_NAME) $(PKG_TMP_BIN_PATH)

	# Darwin
	mkdir -p $(PKG_TMP_BIN_PATH_DARWIN)
	cp -rf ./target/$(PKG_TARGET_DARWIN)/release/$(PKG_NAME) $(PKG_TMP_BIN_PATH_DARWIN)

	# Linux only
	echo "Shrinking binaries for $(PKG_TARGET) release..."
	echo "Size before:"
	du -sh $(PKG_TMP_BIN_PATH)/$(PKG_NAME)
	strip $(PKG_TMP_BIN_PATH)/$(PKG_NAME)
	echo "Size after:"
	du -sh $(PKG_TMP_BIN_PATH)/$(PKG_NAME)
	echo "Copying $(PKG_TMP_BIN_PATH)/$(PKG_NAME) binary to $(PKG_BIN_PATH) directory..."
	cp -rf $(PKG_TMP_BIN_PATH)/$(PKG_NAME) $(PKG_BIN_PATH)/
	echo "Release size shrinking completed!"
endef

# Creates release tarball files
define build_release_files =
	set -e
	set -u

	echo "Generating checksums and creating tarballs..."

	mkdir -p $(PKG_BIN_PATH)
	cd $(PKG_BIN_PATH)

	# Linux
	tar czvf $(PKG_RELEASE_NAME).tar.gz -C $(PKG_TMP_BIN_PATH) $(PKG_NAME)
	sha256sum $(PKG_RELEASE_NAME).tar.gz > $(PKG_NAME)-v$(PKG_TAG)-SHA256SUM

	# Darwin
	tar czvf $(PKG_RELEASE_NAME_DARWIN).tar.gz -C $(PKG_TMP_BIN_PATH_DARWIN) $(PKG_NAME)
	sha256sum $(PKG_RELEASE_NAME_DARWIN).tar.gz >> $(PKG_NAME)-v$(PKG_TAG)-SHA256SUM

	du -sh ./*
	echo "Release tarball files created!"
endef

# Create a release build
prod.release:
	set -e
	set -u

	@echo "Building new release..."
	@rustc -vV

	$(build_release)
	$(build_release_shrink)
	$(build_release_files)
.ONESHELL: prod.release

docker.build:
	@docker build -t joseluisq/paket:latest -f docker/alpine/Dockerfile .
.PHONY: docker.build

docker.run:
	@docker run -it --rm joseluisq/paket:latest -h
.PHONY: docker.run
