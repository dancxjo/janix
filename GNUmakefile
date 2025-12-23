
# Nuke built-in rules and variables.
MAKEFLAGS += -rR
.SUFFIXES:

# Convenience macro to reliably declare user-overridable variables.
override USER_VARIABLE = $(if $(filter $(origin $(1)),default undefined),$(eval override $(1) := $(2)))

# Default Rust warning policy.
$(call USER_VARIABLE,RUSTFLAGS,-Awarnings)

# Default architecture.
$(call USER_VARIABLE,KARCH,x86_64)

# Determine Rust TARGET triple.
ifeq ($(RUST_TARGET),)
    override RUST_TARGET := $(KARCH)-unknown-none
    ifeq ($(KARCH),riscv64)
        override RUST_TARGET := riscv64gc-unknown-none-elf
    endif
endif

# Determine Rust profile.
ifeq ($(RUST_PROFILE),)
    override RUST_PROFILE := dev
endif

override RUST_PROFILE_SUBDIR := $(RUST_PROFILE)
ifeq ($(RUST_PROFILE),dev)
    override RUST_PROFILE_SUBDIR := debug
endif

ENABLE_ROOTFS ?= 0
ENABLE_geographer ?= 1
ENABLE_PLATARO_ICONS ?= 1
APPS := init init_debug clock window_demo compositor hello_world geographer debug_alloc debug_thread
ifneq ($(ENABLE_geographer),1)
# APPS += geographer
endif
DRIVERS := framebuffer ps2_keyboard_driver ps2_mouse_driver pci usb
ifeq ($(ENABLE_ROOTFS),1)
APPS := rootfs $(APPS)
endif

APPS_TARGET_DIR := target/$(RUST_TARGET)/$(RUST_PROFILE_SUBDIR)
COMPOSITOR_FONT_DIR := target/compositor-fonts

# QEMU defaults
$(call USER_VARIABLE,QEMUFLAGS,-m 2G)
$(call USER_VARIABLE,QEMU_NO_REBOOT,-no-reboot)
$(call USER_VARIABLE,QEMU_WATCHER,scripts/qemu-watcher.sh)

QEMUFLAGS_EXTRA ?= -d int,cpu_reset -D qemu_debug.log
QEMU_DISPLAY ?=

ifeq ($(QEMU_DISPLAY),none)
QEMUFLAGS_EXTRA += -display none
endif

override IMAGE_NAME := template-$(KARCH)

FEATURES ?= debug_logging
ifneq ($(strip $(FEATURES)),)
FEATURES_ARG := --features "$(strip $(FEATURES))"
endif


###############################################################################
# TOP LEVEL
###############################################################################

.PHONY: all
all: $(IMAGE_NAME).iso

.PHONY: iso
iso: $(IMAGE_NAME).iso

.PHONY: all-hdd
all-hdd: $(IMAGE_NAME).hdd

TEST_EXCLUDES := --exclude boot \
	--exclude geographer \
	--exclude compositor \
	--exclude window_demo \
	--exclude clock \
	--exclude hello_world \
	--exclude init \
	--exclude debug_alloc \
	--exclude ps2_keyboard_driver \
	--exclude ps2_mouse_driver \
	--exclude framebuffer \
	--exclude pci \
	--exclude usb \
	--exclude compositor_api

.PHONY: test
test:
	@echo "=== Running ThingOS test suite ==="
	cargo test --workspace $(TEST_EXCLUDES) --lib --tests

.PHONY: run
run: run-$(KARCH)

.PHONY: run-hdd
run-hdd: run-hdd-$(KARCH)

# Run-and-capture logs (non-paused) and analyze them
.PHONY: run-log
run-log: run-log-$(KARCH)

.PHONY: run-log-hdd
run-log-hdd: run-log-hdd-$(KARCH)

# Debug run targets: start QEMU with GDB server (-s) and paused CPU (-S)
.PHONY: run-debug
run-debug: run-debug-$(KARCH)

.PHONY: run-debug-hdd
run-debug-hdd: run-debug-hdd-$(KARCH)

.PHONY: smoke
smoke:
	@echo "=== Running Smoke Tests (QEMU) ==="
	THINGOS_QEMU_SMOKE=1 cargo test --package smoke_tests --test qemu_smoke -- --ignored --test-threads=1


###############################################################################
# RUN TARGETS (ISO)
###############################################################################

.PHONY: run-x86_64
run-x86_64:
	$(MAKE) KARCH=x86_64 launch-x86_64

.PHONY: launch-x86_64
launch-x86_64: ovmf/ovmf-code-x86_64.fd ovmf/ovmf-vars-x86_64.fd $(IMAGE_NAME).iso
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-x86_64 $(QEMU_NO_REBOOT) \
		-M q35 \
		-serial stdio \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-x86_64.fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-x86_64.fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA)

.PHONY: run-log-x86_64
run-log-x86_64:
	$(MAKE) KARCH=x86_64 launch-log-x86_64

.PHONY: launch-log-x86_64
launch-log-x86_64: ovmf/ovmf-code-x86_64.fd ovmf/ovmf-vars-x86_64.fd $(IMAGE_NAME).iso
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-x86_64 $(QEMU_NO_REBOOT) \
		-M q35 \
		-serial stdio \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-x86_64.fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-x86_64.fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA) | tee qemu.log ; \
	python3 scripts/analyze_crash.py qemu.log $(APPS_TARGET_DIR) || true ; \
	python3 scripts/analyze_crash.py qemu.log boot || true

.PHONY: run-debug-hang-x86_64
run-debug-hang-x86_64:
	$(MAKE) KARCH=x86_64 launch-debug-hang-x86_64

.PHONY: launch-debug-hang-x86_64
launch-debug-hang-x86_64: ovmf/ovmf-code-x86_64.fd ovmf/ovmf-vars-x86_64.fd $(IMAGE_NAME).iso
	scripts/debug_hang.sh $(APPS_TARGET_DIR) -- \
	qemu-system-x86_64 $(QEMU_NO_REBOOT) \
		-M q35 \
		-serial stdio \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-x86_64.fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-x86_64.fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA)


# ---- AARCH64 ----

.PHONY: run-aarch64
run-aarch64:
	$(MAKE) KARCH=aarch64 launch-aarch64

.PHONY: run-log-aarch64
run-log-aarch64:
	$(MAKE) KARCH=aarch64 launch-log-aarch64

.PHONY: launch-log-aarch64
launch-log-aarch64: ovmf/ovmf-code-aarch64.fd ovmf/ovmf-vars-aarch64.fd $(IMAGE_NAME).iso
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-aarch64 $(QEMU_NO_REBOOT) \
		-M virt \
		-cpu cortex-a72 \
		-serial stdio \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-bios ovmf/ovmf-code-aarch64.fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA) | tee qemu.log ; \
	python3 scripts/analyze_crash.py qemu.log $(APPS_TARGET_DIR) || true ; \
	python3 scripts/analyze_crash.py qemu.log boot || true

.PHONY: launch-aarch64
launch-aarch64: ovmf/ovmf-code-aarch64.fd ovmf/ovmf-vars-aarch64.fd $(IMAGE_NAME).iso
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-aarch64 $(QEMU_NO_REBOOT) \
		-M virt \
        -cpu cortex-a72 \
		-serial stdio \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-bios ovmf/ovmf-code-aarch64.fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA)


# ---- RISCV64 ----

.PHONY: run-riscv64
run-riscv64:
	$(MAKE) KARCH=riscv64 launch-riscv64

.PHONY: run-log-riscv64
run-log-riscv64:
	$(MAKE) KARCH=riscv64 launch-log-riscv64

.PHONY: launch-log-riscv64
launch-log-riscv64: ovmf/ovmf-code-riscv64.fd ovmf/ovmf-vars-riscv64.fd $(IMAGE_NAME).iso
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-riscv64 $(QEMU_NO_REBOOT) \
		-M virt \
		-serial stdio \
		-device ramfb \
		-bios ovmf/ovmf-code-riscv64.fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA) | tee qemu.log ; \
	python3 scripts/analyze_crash.py qemu.log $(APPS_TARGET_DIR) || true ; \
	python3 scripts/analyze_crash.py qemu.log boot || true

.PHONY: launch-riscv64
launch-riscv64: ovmf/ovmf-code-riscv64.fd ovmf/ovmf-vars-riscv64.fd $(IMAGE_NAME).iso
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-riscv64 $(QEMU_NO_REBOOT) \
		-M virt \
		-serial stdio \
		-device ramfb \
		-bios ovmf/ovmf-code-riscv64.fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA)


# ---- LOONGARCH64 ----

.PHONY: run-loongarch64
run-loongarch64:
	$(MAKE) KARCH=loongarch64 launch-loongarch64

.PHONY: run-log-loongarch64
run-log-loongarch64:
	$(MAKE) KARCH=loongarch64 launch-log-loongarch64

.PHONY: launch-log-loongarch64
launch-log-loongarch64: ovmf/ovmf-code-loongarch64.fd ovmf/ovmf-vars-loongarch64.fd $(IMAGE_NAME).iso
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-loongarch64 $(QEMU_NO_REBOOT) \
		-M virt \
		-serial stdio \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-loongarch64.fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-loongarch64.fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA) | tee qemu.log ; \
	python3 scripts/analyze_crash.py qemu.log $(APPS_TARGET_DIR) || true ; \
	python3 scripts/analyze_crash.py qemu.log boot || true

.PHONY: launch-loongarch64
launch-loongarch64: ovmf/ovmf-code-loongarch64.fd ovmf/ovmf-vars-loongarch64.fd $(IMAGE_NAME).iso
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-loongarch64 $(QEMU_NO_REBOOT) \
		-M virt \
		-serial stdio \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-loongarch64.fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-loongarch64.fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA)


###############################################################################
###############################################################################
# DEBUG RUN TARGETS (ISO + HDD)
# These launch targets start QEMU with a GDB server (-s) and keep the CPU
# paused at startup (-S) so you can attach a debugger. Ports default to
# QEMU's default (1234) when using `-s`.

.PHONY: run-debug-x86_64
run-debug-x86_64:
	$(MAKE) KARCH=x86_64 launch-debug-x86_64

.PHONY: launch-debug-x86_64
launch-debug-x86_64: ovmf/ovmf-code-x86_64.fd ovmf/ovmf-vars-x86_64.fd $(IMAGE_NAME).iso
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-x86_64 $(QEMU_NO_REBOOT) -s -S \
		-M q35 \
		-serial stdio \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-x86_64.fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-x86_64.fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA) | tee qemu.log ; \
	python3 scripts/analyze_crash.py qemu.log $(APPS_TARGET_DIR) || true ; \
	python3 scripts/analyze_crash.py qemu.log boot || true

# ---- AARCH64 ----

.PHONY: run-debug-aarch64
run-debug-aarch64:
	$(MAKE) KARCH=aarch64 launch-debug-aarch64

.PHONY: launch-debug-aarch64
launch-debug-aarch64: ovmf/ovmf-code-aarch64.fd ovmf/ovmf-vars-aarch64.fd $(IMAGE_NAME).iso
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-aarch64 $(QEMU_NO_REBOOT) -s -S \
		-M virt \
		-cpu cortex-a72 \
		-serial stdio \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-bios ovmf/ovmf-code-aarch64.fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA) | tee qemu.log ; \
	python3 scripts/analyze_crash.py qemu.log $(APPS_TARGET_DIR) || true ; \
	python3 scripts/analyze_crash.py qemu.log boot || true

# ---- RISCV64 ----

.PHONY: run-debug-riscv64
run-debug-riscv64:
	$(MAKE) KARCH=riscv64 launch-debug-riscv64

.PHONY: launch-debug-riscv64
launch-debug-riscv64: ovmf/ovmf-code-riscv64.fd ovmf/ovmf-vars-riscv64.fd $(IMAGE_NAME).iso
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-riscv64 $(QEMU_NO_REBOOT) -s -S \
		-M virt \
		-serial stdio \
		-device ramfb \
		-bios ovmf/ovmf-code-riscv64.fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA) | tee qemu.log ; \
	python3 scripts/analyze_crash.py qemu.log $(APPS_TARGET_DIR) || true ; \
	python3 scripts/analyze_crash.py qemu.log boot || true

# ---- LOONGARCH64 ----

.PHONY: run-debug-loongarch64
run-debug-loongarch64:
	$(MAKE) KARCH=loongarch64 launch-debug-loongarch64

.PHONY: launch-debug-loongarch64
launch-debug-loongarch64: ovmf/ovmf-code-loongarch64.fd ovmf/ovmf-vars-loongarch64.fd $(IMAGE_NAME).iso
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-loongarch64 $(QEMU_NO_REBOOT) -s -S \
		-M virt \
		-serial stdio \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-loongarch64.fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-loongarch64.fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA) | tee qemu.log ; \
	python3 scripts/analyze_crash.py qemu.log $(APPS_TARGET_DIR) || true ; \
	python3 scripts/analyze_crash.py qemu.log boot || true

###############################################################################
# RUN TARGETS (HDD VARIANTS)
###############################################################################

# They follow same pattern.
# Keeping your existing HDD rules unchanged.

# Debug HDD targets: start QEMU with GDB server (-s) and paused CPU (-S)
.PHONY: run-debug-hdd-x86_64
run-debug-hdd-x86_64:
	$(MAKE) KARCH=x86_64 launch-debug-hdd-x86_64

.PHONY: launch-debug-hdd-x86_64
launch-debug-hdd-x86_64: ovmf/ovmf-code-x86_64.fd ovmf/ovmf-vars-x86_64.fd $(IMAGE_NAME).hdd
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-x86_64 $(QEMU_NO_REBOOT) -s -S \
		-M q35 \
		-serial stdio \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-x86_64.fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-x86_64.fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA) | tee qemu.log ; \
	python3 scripts/analyze_crash.py qemu.log $(APPS_TARGET_DIR) || true ; \
	python3 scripts/analyze_crash.py qemu.log boot || true

.PHONY: run-debug-hdd-aarch64
run-debug-hdd-aarch64:
	$(MAKE) KARCH=aarch64 launch-debug-hdd-aarch64

.PHONY: launch-debug-hdd-aarch64
launch-debug-hdd-aarch64: ovmf/ovmf-code-aarch64.fd ovmf/ovmf-vars-aarch64.fd $(IMAGE_NAME).hdd
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-aarch64 $(QEMU_NO_REBOOT) -s -S \
		-M virt \
		-cpu cortex-a72 \
		-serial stdio \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-bios ovmf/ovmf-code-aarch64.fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA) | tee qemu.log ; \
	python3 scripts/analyze_crash.py qemu.log $(APPS_TARGET_DIR) || true ; \
	python3 scripts/analyze_crash.py qemu.log boot || true

.PHONY: run-debug-hdd-riscv64
run-debug-hdd-riscv64:
	$(MAKE) KARCH=riscv64 launch-debug-hdd-riscv64

.PHONY: launch-debug-hdd-riscv64
launch-debug-hdd-riscv64: ovmf/ovmf-code-riscv64.fd ovmf/ovmf-vars-riscv64.fd $(IMAGE_NAME).hdd
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-riscv64 $(QEMU_NO_REBOOT) -s -S \
		-M virt \
		-serial stdio \
		-device ramfb \
		-bios ovmf/ovmf-code-riscv64.fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA) | tee qemu.log ; \
	python3 scripts/analyze_crash.py qemu.log $(APPS_TARGET_DIR) || true ; \
	python3 scripts/analyze_crash.py qemu.log boot || true

.PHONY: run-debug-hdd-loongarch64
run-debug-hdd-loongarch64:
	$(MAKE) KARCH=loongarch64 launch-debug-hdd-loongarch64

.PHONY: launch-debug-hdd-loongarch64
launch-debug-hdd-loongarch64: ovmf/ovmf-code-loongarch64.fd ovmf/ovmf-vars-loongarch64.fd $(IMAGE_NAME).hdd
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-loongarch64 $(QEMU_NO_REBOOT) -s -S \
		-M virt \
		-serial stdio \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-loongarch64.fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-loongarch64.fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA) | tee qemu.log ; \
	python3 scripts/analyze_crash.py qemu.log $(APPS_TARGET_DIR) || true ; \
	python3 scripts/analyze_crash.py qemu.log boot || true

.PHONY: run-hdd-x86_64
run-hdd-x86_64:
	$(MAKE) KARCH=x86_64 launch-hdd-x86_64

.PHONY: run-log-hdd-x86_64
run-log-hdd-x86_64:
	$(MAKE) KARCH=x86_64 launch-log-hdd-x86_64

.PHONY: launch-log-hdd-x86_64
launch-log-hdd-x86_64: ovmf/ovmf-code-x86_64.fd ovmf/ovmf-vars-x86_64.fd $(IMAGE_NAME).hdd
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-x86_64 $(QEMU_NO_REBOOT) \
		-M q35 \
		-serial stdio \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-x86_64.fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-x86_64.fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA) | tee qemu.log ; \
	python3 scripts/analyze_crash.py qemu.log $(APPS_TARGET_DIR) || true ; \
	python3 scripts/analyze_crash.py qemu.log boot || true

.PHONY: launch-hdd-x86_64
launch-hdd-x86_64: ovmf/ovmf-code-x86_64.fd ovmf/ovmf-vars-x86_64.fd $(IMAGE_NAME).hdd
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-x86_64 $(QEMU_NO_REBOOT) \
		-M q35 \
		-serial stdio \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-x86_64.fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-x86_64.fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA)


# AARCH64 HDD

.PHONY: run-hdd-aarch64
run-hdd-aarch64:
	$(MAKE) KARCH=aarch64 launch-hdd-aarch64

.PHONY: run-log-hdd-aarch64
run-log-hdd-aarch64:
	$(MAKE) KARCH=aarch64 launch-log-hdd-aarch64

.PHONY: launch-log-hdd-aarch64
launch-log-hdd-aarch64: ovmf/ovmf-code-aarch64.fd ovmf/ovmf-vars-aarch64.fd $(IMAGE_NAME).hdd
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-aarch64 $(QEMU_NO_REBOOT) \
		-M virt \
		-cpu cortex-a72 \
		-serial stdio \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-bios ovmf/ovmf-code-aarch64.fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA) | tee qemu.log ; \
	python3 scripts/analyze_crash.py qemu.log $(APPS_TARGET_DIR) || true ; \
	python3 scripts/analyze_crash.py qemu.log boot || true

.PHONY: launch-hdd-aarch64
launch-hdd-aarch64: ovmf/ovmf-code-aarch64.fd ovmf/ovmf-vars-aarch64.fd $(IMAGE_NAME).hdd
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-aarch64 $(QEMU_NO_REBOOT) \
		-M virt \
        -cpu cortex-a72 \
		-serial stdio \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-bios ovmf/ovmf-code-aarch64.fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA)


# RISCV64 HDD

.PHONY: run-hdd-riscv64
run-hdd-riscv64:
	$(MAKE) KARCH=riscv64 launch-hdd-riscv64

.PHONY: run-log-hdd-riscv64
run-log-hdd-riscv64:
	$(MAKE) KARCH=riscv64 launch-log-hdd-riscv64

.PHONY: launch-log-hdd-riscv64
launch-log-hdd-riscv64: ovmf/ovmf-code-riscv64.fd ovmf/ovmf-vars-riscv64.fd $(IMAGE_NAME).hdd
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-riscv64 $(QEMU_NO_REBOOT) \
		-M virt \
		-serial stdio \
		-device ramfb \
		-bios ovmf/ovmf-code-riscv64.fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA) | tee qemu.log ; \
	python3 scripts/analyze_crash.py qemu.log $(APPS_TARGET_DIR) || true ; \
	python3 scripts/analyze_crash.py qemu.log boot || true

.PHONY: launch-hdd-riscv64
launch-hdd-riscv64: ovmf/ovmf-code-riscv64.fd ovmf/ovmf-vars-riscv64.fd $(IMAGE_NAME).hdd
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-riscv64 $(QEMU_NO_REBOOT) \
		-M virt \
		-serial stdio \
		-device ramfb \
		-bios ovmf/ovmf-code-riscv64.fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA)


# LOONGARCH64 HDD

.PHONY: run-hdd-loongarch64
run-hdd-loongarch64:
	$(MAKE) KARCH=loongarch64 launch-hdd-loongarch64

.PHONY: run-log-hdd-loongarch64
run-log-hdd-loongarch64:
	$(MAKE) KARCH=loongarch64 launch-log-hdd-loongarch64

.PHONY: launch-log-hdd-loongarch64
launch-log-hdd-loongarch64: ovmf/ovmf-code-loongarch64.fd ovmf/ovmf-vars-loongarch64.fd $(IMAGE_NAME).hdd
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-loongarch64 $(QEMU_NO_REBOOT) \
		-M virt \
		-serial stdio \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-loongarch64.fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-loongarch64.fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA) | tee qemu.log ; \
	python3 scripts/analyze_crash.py qemu.log $(APPS_TARGET_DIR) || true ; \
	python3 scripts/analyze_crash.py qemu.log boot || true

.PHONY: launch-hdd-loongarch64
launch-hdd-loongarch64: ovmf/ovmf-code-loongarch64.fd ovmf/ovmf-vars-loongarch64.fd $(IMAGE_NAME).hdd
	$(QEMU_WATCHER) --pattern 'PANIC!|DOUBLE FAULT' -- \
	qemu-system-loongarch64 $(QEMU_NO_REBOOT) \
		-M virt \
		-serial stdio \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-loongarch64.fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-loongarch64.fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS) $(QEMUFLAGS_EXTRA)


###############################################################################
# OVMF FIRMWARE DOWNLOAD RULES  (CORRECT URLs!)
###############################################################################

# ----------------------
# X86_64
# ----------------------

ovmf/ovmf-code-x86_64.fd:
	mkdir -p ovmf
	curl -L -o $@ https://github.com/osdev0/edk2-ovmf-nightly/releases/download/nightly-2025-10-09/ovmf-code-x86_64.fd

ovmf/ovmf-vars-x86_64.fd:
	mkdir -p ovmf
	curl -L -o $@ https://github.com/osdev0/edk2-ovmf-nightly/releases/download/nightly-2025-10-09/ovmf-vars-x86_64.fd


# ----------------------
# AARCH64
# ----------------------

ovmf/ovmf-code-aarch64.fd:
	mkdir -p ovmf
	curl -L -o $@ https://github.com/osdev0/edk2-ovmf-nightly/releases/download/nightly-2025-10-09/ovmf-code-aarch64.fd

ovmf/ovmf-vars-aarch64.fd:
	mkdir -p ovmf
	curl -L -o $@ https://github.com/osdev0/edk2-ovmf-nightly/releases/download/nightly-2025-10-09/ovmf-vars-aarch64.fd


# ----------------------
# RISCV64
# ----------------------

ovmf/ovmf-code-riscv64.fd:
	mkdir -p ovmf
	curl -L -o $@ https://github.com/osdev0/edk2-ovmf-nightly/releases/download/nightly-2025-10-09/ovmf-code-riscv64.fd

ovmf/ovmf-vars-riscv64.fd:
	mkdir -p ovmf
	truncate -s 32M $@


# ----------------------
# LOONGARCH64
# ----------------------

ovmf/ovmf-code-loongarch64.fd:
	mkdir -p ovmf
	curl -L -o $@ https://github.com/osdev0/edk2-ovmf-nightly/releases/download/nightly-2025-10-09/ovmf-code-loongarch64.fd

ovmf/ovmf-vars-loongarch64.fd:
	mkdir -p ovmf
	curl -L -o $@ https://github.com/osdev0/edk2-ovmf-nightly/releases/download/nightly-2025-10-09/ovmf-vars-loongarch64.fd



###############################################################################
# LIMINE + BUILD
###############################################################################

limine/limine:
	rm -rf limine
	git clone https://github.com/limine-bootloader/limine.git --branch=v9.x-binary --depth=1
	$(MAKE) -C limine

.PHONY: user
user:
	RUSTFLAGS="-C relocation-model=static -Awarnings -C link-arg=-e -C link-arg=main" cargo build --target $(RUST_TARGET) --profile $(RUST_PROFILE) $(addprefix -p ,$(APPS))
	RUSTFLAGS="-C relocation-model=static -Awarnings -C link-arg=-e -C link-arg=main" cargo build --target $(RUST_TARGET) --profile release -p init -p init_debug

.PHONY: drivers
drivers:
ifneq ($(strip $(DRIVERS)),)
	RUSTFLAGS="-C relocation-model=static -Awarnings -C link-arg=-e -C link-arg=main" cargo build --target $(RUST_TARGET) --profile $(RUST_PROFILE) $(addprefix -p ,$(DRIVERS))
endif
	RUSTFLAGS="-C relocation-model=static -Awarnings -C link-arg=-e -C link-arg=main" cargo build --target $(RUST_TARGET) --profile release -p pci

.PHONY: kernel
kernel:
	$(MAKE) -C boot FEATURES="$(FEATURES)"


###############################################################################
# ISO BUILD  (UNCHANGED FROM YOUR VERSION)
###############################################################################

ifeq ($(ENABLE_PLATARO_ICONS),1)
.PHONY: icons
icons:
	# tools/icons/build_plataro_icons.sh
	echo "Plataro icons disabled (ENABLE_PLATARO_ICONS!=1)"
else
.PHONY: icons
icons:
	@echo "Plataro icons disabled (ENABLE_PLATARO_ICONS!=1)"
endif

.PHONY: assets
assets:
ifeq ($(ENABLE_PLATARO_ICONS),1)
	mkdir -p assets/tango-raw
	# Download Tango icon theme if not already cached
	if [ ! -f assets/tango.tar.gz ]; then \
		echo "Downloading Tango icons..."; \
		curl -L -o assets/tango.tar.gz http://tango.freedesktop.org/releases/tango-icon-theme-0.8.90.tar.gz; \
	else \
		echo "Using cached Tango icons from assets/tango.tar.gz"; \
	fi
	tar -xzf assets/tango.tar.gz -C assets/tango-raw --strip-components=1
	mkdir -p assets/icons
	# Build icon-gen tool
	cargo build --manifest-path tools/icon-gen/Cargo.toml --release
	# Run icon-gen
	./target/release/icon-gen --input assets/tango-raw/scalable --output assets/icons
else
	@echo "Skipping asset generation (ENABLE_PLATARO_ICONS!=1)"
endif


$(IMAGE_NAME).iso: limine/limine kernel user drivers icons assets
	rm -rf iso_root
	# Prepare ISO root with both BIOS and UEFI directory trees upfront.
	mkdir -p iso_root/boot iso_root/boot/user iso_root/boot/drivers iso_root/boot/limine iso_root/EFI/BOOT
	cp -v boot/kernel iso_root/boot/
	cp -v assets/wallpapers/clouds.bmp iso_root/boot/clouds.bmp
	for app in clock window_demo compositor hello_world geographer debug_alloc debug_thread; do \
		cp -v $(APPS_TARGET_DIR)/$$app iso_root/boot/user/$$app; \
		# objcopy --strip-debug iso_root/boot/user/$$app; \
	done
	cp -v target/$(RUST_TARGET)/release/init iso_root/boot/user/init
	cp -v target/$(RUST_TARGET)/release/init_debug iso_root/boot/user/init_debug

	for drv in framebuffer ps2_keyboard_driver ps2_mouse_driver usb; do \
		cp -v $(APPS_TARGET_DIR)/$$drv iso_root/boot/drivers/$$drv; \
		# objcopy --strip-debug iso_root/boot/drivers/$$drv; \
	done
	cp -v target/$(RUST_TARGET)/release/pci iso_root/boot/drivers/pci
	# Fonts: Only include unifont.hex and HACK_REGULAR.ttf
	# if [ -d $(COMPOSITOR_FONT_DIR) ] && ls $(COMPOSITOR_FONT_DIR)/*.ttf >/dev/null 2>&1; then \
	# 	mkdir -p iso_root/boot/fonts; \
	# 	cp -v $(COMPOSITOR_FONT_DIR)/*.ttf iso_root/boot/fonts/; \
	# fi
	# Copy unified fonts
	# Copy unified fonts
	mkdir -p iso_root/boot/fonts
	cp -v assets/fonts/unifont.hex iso_root/boot/fonts/
	cp -v assets/fonts/HACK_REGULAR.ttf iso_root/boot/fonts/
	cp -v limine.conf iso_root/boot/limine/limine.conf
ifeq ($(ENABLE_geographer),1)
	echo '    module_path: boot():/boot/user/geographer' >> iso_root/boot/limine/limine.conf
	echo '    module_cmdline: program=geographer' >> iso_root/boot/limine/limine.conf
endif
ifeq ($(ENABLE_PLATARO_ICONS),1)
	mkdir -p iso_root/share/icons/tango
	cp -r assets/icons/* iso_root/share/icons/tango/
	# Append icons to limine.conf as modules
	# Icons are no longer added to limine.conf as modules
	# for icon in iso_root/share/icons/tango/*.bmp; do \
	# 	NAME=$$(basename $$icon); \
	# 	echo "    module_path: boot():/share/icons/tango/$$NAME" >> iso_root/boot/limine/limine.conf; \
	# 	echo "    module_cmdline: image=$$NAME" >> iso_root/boot/limine/limine.conf; \
	# done
endif
	cp -v iso_root/boot/limine/limine.conf iso_root/EFI/BOOT/limine.conf
	cp -v iso_root/boot/limine/limine.conf iso_root/limine.conf
ifeq ($(KARCH),x86_64)
	cp -v limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root/boot/limine/
	cp -v limine/BOOTX64.EFI iso_root/EFI/BOOT/
	cp -v limine/BOOTIA32.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o $(IMAGE_NAME).iso
	cp iso_root/boot/drivers/pci pci_debug_dump
	objdump -d -S --start-address=0x206d50 --stop-address=0x206da0 pci_debug_dump > pci_dump_snippet.txt
	./limine/limine bios-install $(IMAGE_NAME).iso
endif
ifeq ($(KARCH),aarch64)
	cp -v limine/limine-uefi-cd.bin iso_root/boot/limine/
	cp -v limine/BOOTAA64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o $(IMAGE_NAME).iso
endif
ifeq ($(KARCH),riscv64)
	cp -v limine/limine-uefi-cd.bin iso_root/boot/limine/
	mcopy -i iso_root/boot/limine/limine-uefi-cd.bin templates/riscv-startup.nsh ::/startup.nsh
	cp -v limine/BOOTRISCV64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o $(IMAGE_NAME).iso
endif
ifeq ($(KARCH),loongarch64)
	cp -v limine/limine-uefi-cd.bin iso_root/boot/limine/
	cp -v limine/BOOTLOONGARCH64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o $(IMAGE_NAME).iso
endif
	rm -rf iso_root


###############################################################################
# HDD IMAGE BUILD
###############################################################################

$(IMAGE_NAME).hdd: limine/limine kernel user
	rm -f $(IMAGE_NAME).hdd
	dd if=/dev/zero bs=1M count=0 seek=128 of=$@

	sgdisk $(IMAGE_NAME).hdd -n 1:2048 -t 1:ef00
ifeq ($(KARCH),x86_64)
	./limine/limine bios-install $(IMAGE_NAME).hdd
endif
	# Use static limine.conf (do not alter dynamically)
	mformat -i $(IMAGE_NAME).hdd@@1M
	mmd -i $(IMAGE_NAME).hdd@@1M ::/EFI ::/EFI/BOOT ::/boot ::/boot/limine ::/boot/user ::/boot/fonts
	mcopy -i $(IMAGE_NAME).hdd@@1M boot/kernel ::/boot
	mcopy -i $(IMAGE_NAME).hdd@@1M clouds.bmp ::/boot
	for app in $(APPS); do \
		mcopy -i $(IMAGE_NAME).hdd@@1M $(APPS_TARGET_DIR)/$$app ::/boot/user; \
	done
	mmd -i $(IMAGE_NAME).hdd@@1M ::/boot/drivers
	for drv in $(DRIVERS); do \
		mcopy -i $(IMAGE_NAME).hdd@@1M $(APPS_TARGET_DIR)/$$drv ::/boot/drivers; \
	done
	if [ -d $(COMPOSITOR_FONT_DIR) ] && ls $(COMPOSITOR_FONT_DIR)/*.ttf >/dev/null 2>&1; then \
		for font in $(COMPOSITOR_FONT_DIR)/*.ttf; do \
			mcopy -i $(IMAGE_NAME).hdd@@1M $$font ::/boot/fonts/; \
		done; \
	fi
	mcopy -i $(IMAGE_NAME).hdd@@1M limine.conf ::/boot/limine
	mcopy -i $(IMAGE_NAME).hdd@@1M limine.conf ::/EFI/BOOT/limine.conf
ifeq ($(KARCH),riscv64)
	mcopy -i $(IMAGE_NAME).hdd@@1M templates/riscv-startup.nsh ::/
endif
ifeq ($(KARCH),x86_64)
	mcopy -i $(IMAGE_NAME).hdd@@1M limine/limine-bios.sys ::/boot/limine
	mcopy -i $(IMAGE_NAME).hdd@@1M limine/BOOTX64.EFI ::/EFI/BOOT
	mcopy -i $(IMAGE_NAME).hdd@@1M limine/BOOTIA32.EFI ::/EFI/BOOT
endif
ifeq ($(KARCH),aarch64)
	mcopy -i $(IMAGE_NAME).hdd@@1M limine/BOOTAA64.EFI ::/EFI/BOOT
endif
ifeq ($(KARCH),riscv64)
	mcopy -i $(IMAGE_NAME).hdd@@1M limine/BOOTRISCV64.EFI ::/EFI/BOOT
endif
ifeq ($(KARCH),loongarch64)
	mcopy -i $(IMAGE_NAME).hdd@@1M limine/BOOTLOONGARCH64.EFI ::/EFI/BOOT
endif


###############################################################################
# CLEANUP
###############################################################################

.PHONY: clean
clean:
	$(MAKE) -C boot clean
	rm -f qemu.log
	rm -rf iso_root $(IMAGE_NAME).iso $(IMAGE_NAME).hdd
	-cargo clean --manifest-path Cargo.toml
	rm -rf target $(COMPOSITOR_FONT_DIR) ovmf/*.fd

.PHONY: distclean
distclean: clean
	$(MAKE) -C boot distclean
	rm -rf limine ovmf

.PHONY: contracts
contracts:
	@mkdir -p docs/contracts
	python3 tools/generate_contracts.py
