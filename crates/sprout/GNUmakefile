# Nuke built-in rules and variables.
MAKEFLAGS += -rR
.SUFFIXES:

# Sprout is a user-mode process.
override OUTPUT := sprout
override OUTPUT_DIR = bin-$(KARCH)
override OUTPUT_PATH = $(OUTPUT_DIR)/$(OUTPUT)

# Convenience macro to reliably declare user overridable variables.
override USER_VARIABLE = $(if $(filter $(origin $(1)),default undefined),$(eval override $(1) := $(2)))

# Target architecture to build for. Default to x86_64.
$(call USER_VARIABLE,KARCH,x86_64)

ifeq ($(RUST_TARGET),)
    override RUST_TARGET := $(KARCH)-unknown-none
	ifeq ($(KARCH),riscv64)
    	override RUST_TARGET := riscv64gc-unknown-none-elf
	endif
endif

ifeq ($(RUST_PROFILE),)
    override RUST_PROFILE := dev
endif

override RUST_PROFILE_SUBDIR := $(RUST_PROFILE)
ifeq ($(RUST_PROFILE),dev)
    override RUST_PROFILE_SUBDIR := debug
endif

# Default target.
.PHONY: all
all:
	# Use PIE for user-space applications to allow loading at any address
	# -C link-arg=-pie is valid for lld on some targets, or just relocation-model=pic
	RUSTFLAGS="-C relocation-model=pic -C link-arg=-pie" cargo build --target $(RUST_TARGET) --profile $(RUST_PROFILE)
	mkdir -p $(OUTPUT_DIR)
	cp ../../target/$(RUST_TARGET)/$(RUST_PROFILE_SUBDIR)/sprout $(OUTPUT_PATH)

# Remove object files and the final executable.
.PHONY: clean
clean:
	cargo clean
	rm -rf bin-*

.PHONY: distclean
distclean: clean
