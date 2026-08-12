RUST_DIR = $(top_srcdir)/..
RUST_TARGET_DIR = $(RUST_DIR)/target
RUST_PROFILE = release
RUST_OUT_SO = $(top_builddir)/modules/anydoc.so
PHP_TEST_SHARED_EXTENSIONS = -d extension=$(RUST_OUT_SO)
TESTS ?= $(RUST_DIR)/tests

ifeq ($(shell uname -s),Darwin)
RUST_DYLIB = $(RUST_TARGET_DIR)/$(RUST_PROFILE)/libanydoc.dylib
else
RUST_DYLIB = $(RUST_TARGET_DIR)/$(RUST_PROFILE)/libanydoc.so
endif

all: build-modules

build-modules: rust-build

.PHONY: rust-build

rust-build:
	@$(mkinstalldirs) "$(top_builddir)/modules"
	@echo "Building the anydoc Rust extension ($(RUST_PROFILE))"
	@cd "$(RUST_DIR)" && \
		PHP="$(PHP_EXECUTABLE)" \
		PHP_CONFIG="$(PHP_CONFIG)" \
		RUSTC="$(RUSTC)" \
		"$(CARGO)" build --release
	@test -f "$(RUST_DYLIB)" || { \
		echo "Built library not found: $(RUST_DYLIB)"; \
		exit 1; \
	}
	@$(INSTALL) "$(RUST_DYLIB)" "$(RUST_OUT_SO)"
	@echo "Built $(RUST_OUT_SO)"

install-modules: rust-build
