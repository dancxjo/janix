# Nuke built-in rules and variables.
MAKEFLAGS += -rR
.SUFFIXES:

# Convenience macro to reliably declare user overridable variables.
override USER_VARIABLE = $(if $(filter $(origin $(1)),default undefined),$(eval override $(1) := $(2)))

# Target architecture to build for. Default to x86_64.
$(call USER_VARIABLE,KARCH,x86_64)

# Determine Rust target/profile for artifacts.
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

ENABLE_ROOTFS ?= 0
APPS := init thread_dashboard clock_demo alarm_demo ps2_keyboard_driver ps2_mouse_driver input_logger_demo framebuffer_driver compositor window_demo
ifeq ($(ENABLE_ROOTFS),1)
APPS := rootfs $(APPS)
endif
APPS_TARGET_DIR := target/$(RUST_TARGET)/$(RUST_PROFILE_SUBDIR)
COMPOSITOR_FONT_DIR := target/compositor-fonts

# Default user QEMU flags. These are appended to the QEMU command calls.
$(call USER_VARIABLE,QEMUFLAGS,-m 2G)
# Force QEMU to exit instead of rebooting when the guest halts/crashes.
$(call USER_VARIABLE,QEMU_NO_REBOOT,-no-reboot)

# External watcher wrapper that runs QEMU and exits when a crash/halt pattern
# is observed on QEMU's serial/stdout. Can be overridden by users.
$(call USER_VARIABLE,QEMU_WATCHER,scripts/qemu-watcher.sh)

override IMAGE_NAME := template-$(KARCH)
FEATURES ?=
ifneq ($(strip $(FEATURES)),)
FEATURES_ARG := --features "$(strip $(FEATURES))"
endif

.PHONY: all
all: $(IMAGE_NAME).iso

.PHONY: all-hdd
all-hdd: $(IMAGE_NAME).hdd

.PHONY: test
test:
	@echo "=== Running ThingOS test suite ==="
	cargo test --workspace --exclude boot

.PHONY: run
run: run-$(KARCH)

.PHONY: run-hdd
run-hdd: run-hdd-$(KARCH)

.PHONY: smoke
smoke:
	@echo "=== Running Smoke Tests (QEMU) ==="
	THINGOS_QEMU_SMOKE=1 cargo test --package smoke_tests --test qemu_smoke -- --ignored --test-threads=1

.PHONY: run-x86_64
run-x86_64:
	$(MAKE) KARCH=x86_64 launch-x86_64

.PHONY: launch-x86_64
launch-x86_64: ovmf/ovmf-code-$(KARCH).fd ovmf/ovmf-vars-$(KARCH).fd $(IMAGE_NAME).iso
	$(QEMU_WATCHER) --pattern 'PANIC!|No runnable threads' -- qemu-system-$(KARCH) $(QEMU_NO_REBOOT) \
		-M q35 \
		-serial stdio \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-$(KARCH).fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-$(KARCH).fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS)

.PHONY: run-hdd-x86_64
run-hdd-x86_64:
	$(MAKE) KARCH=x86_64 launch-hdd-x86_64

.PHONY: launch-hdd-x86_64
launch-hdd-x86_64: ovmf/ovmf-code-$(KARCH).fd ovmf/ovmf-vars-$(KARCH).fd $(IMAGE_NAME).hdd
	$(QEMU_WATCHER) --pattern 'PANIC!|No runnable threads' -- qemu-system-$(KARCH) $(QEMU_NO_REBOOT) \
		-M q35 \
		-serial stdio \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-$(KARCH).fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-$(KARCH).fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS)

.PHONY: run-aarch64
run-aarch64:
	$(MAKE) KARCH=aarch64 launch-aarch64

.PHONY: launch-aarch64
launch-aarch64: ovmf/ovmf-code-$(KARCH).fd ovmf/ovmf-vars-$(KARCH).fd $(IMAGE_NAME).iso
	$(QEMU_WATCHER) --pattern 'PANIC!|No runnable threads' -- qemu-system-$(KARCH) $(QEMU_NO_REBOOT) \
		-M virt \
		-cpu cortex-a72 \
		-serial stdio \
		-semihosting \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-$(KARCH).fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-$(KARCH).fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS)

.PHONY: run-hdd-aarch64
run-hdd-aarch64:
	$(MAKE) KARCH=aarch64 launch-hdd-aarch64

.PHONY: launch-hdd-aarch64
launch-hdd-aarch64: ovmf/ovmf-code-$(KARCH).fd ovmf/ovmf-vars-$(KARCH).fd $(IMAGE_NAME).hdd
	$(QEMU_WATCHER) --pattern 'PANIC!|No runnable threads' -- qemu-system-$(KARCH) $(QEMU_NO_REBOOT) \
		-M virt \
		-cpu cortex-a72 \
		-serial stdio \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-$(KARCH).fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-$(KARCH).fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS)

.PHONY: run-riscv64
run-riscv64:
	$(MAKE) KARCH=riscv64 launch-riscv64

.PHONY: launch-riscv64
launch-riscv64: ovmf/ovmf-code-$(KARCH).fd ovmf/ovmf-vars-$(KARCH).fd $(IMAGE_NAME).iso
	$(QEMU_WATCHER) --pattern 'PANIC!|No runnable threads' -- qemu-system-$(KARCH) $(QEMU_NO_REBOOT) \
		-M virt \
		-cpu rv64 \
		-serial stdio \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-$(KARCH).fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-$(KARCH).fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS)

.PHONY: run-hdd-riscv64
run-hdd-riscv64:
	$(MAKE) KARCH=riscv64 launch-hdd-riscv64

.PHONY: launch-hdd-riscv64
launch-hdd-riscv64: ovmf/ovmf-code-$(KARCH).fd ovmf/ovmf-vars-$(KARCH).fd $(IMAGE_NAME).hdd
	$(QEMU_WATCHER) --pattern 'PANIC!|No runnable threads' -- qemu-system-$(KARCH) $(QEMU_NO_REBOOT) \
		-M virt \
		-cpu rv64 \
		-serial stdio \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-$(KARCH).fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-$(KARCH).fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS)

.PHONY: run-loongarch64
run-loongarch64:
	$(MAKE) KARCH=loongarch64 launch-loongarch64

.PHONY: launch-loongarch64
launch-loongarch64: ovmf/ovmf-code-$(KARCH).fd ovmf/ovmf-vars-$(KARCH).fd $(IMAGE_NAME).iso
	$(QEMU_WATCHER) --pattern 'PANIC!|No runnable threads' -- qemu-system-$(KARCH) $(QEMU_NO_REBOOT) \
		-M virt \
		-cpu la464 \
		-serial stdio \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-$(KARCH).fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-$(KARCH).fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS)

.PHONY: run-hdd-loongarch64
run-hdd-loongarch64:
	$(MAKE) KARCH=loongarch64 launch-hdd-loongarch64

.PHONY: launch-hdd-loongarch64
launch-hdd-loongarch64: ovmf/ovmf-code-$(KARCH).fd ovmf/ovmf-vars-$(KARCH).fd $(IMAGE_NAME).hdd
	$(QEMU_WATCHER) --pattern 'PANIC!|No runnable threads' -- qemu-system-$(KARCH) $(QEMU_NO_REBOOT) \
		-M virt \
		-cpu la464 \
		-serial stdio \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-$(KARCH).fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-$(KARCH).fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS)


.PHONY: run-bios
run-bios: $(IMAGE_NAME).iso
	$(QEMU_WATCHER) --pattern 'PANIC!' -- qemu-system-$(KARCH) $(QEMU_NO_REBOOT) \
		-M q35 \
		-serial stdio \
		-cdrom $(IMAGE_NAME).iso \
		-boot d \
		$(QEMUFLAGS)

.PHONY: run-hdd-bios
run-hdd-bios: $(IMAGE_NAME).hdd
	$(QEMU_WATCHER) --pattern 'PANIC!' -- qemu-system-$(KARCH) $(QEMU_NO_REBOOT) \
		-M q35 \
		-serial stdio \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS)

ovmf/edk2-ovmf.tar.gz:
	mkdir -p ovmf
	curl -Lo $@ https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/edk2-ovmf.tar.gz

ovmf/ovmf-code-$(KARCH).fd: ovmf/edk2-ovmf.tar.gz
	tar -xzf $< -C ovmf --strip-components=1 edk2-ovmf/ovmf-code-$(KARCH).fd
	case "$(KARCH)" in \
		aarch64) dd if=/dev/zero of=$@ bs=1 count=0 seek=67108864 2>/dev/null;; \
		loongarch64) dd if=/dev/zero of=$@ bs=1 count=0 seek=5242880 2>/dev/null;; \
		riscv64) dd if=/dev/zero of=$@ bs=1 count=0 seek=33554432 2>/dev/null;; \
	esac

ovmf/ovmf-vars-$(KARCH).fd: ovmf/edk2-ovmf.tar.gz
	tar -xzf $< -C ovmf --strip-components=1 edk2-ovmf/ovmf-vars-$(KARCH).fd
	case "$(KARCH)" in \
		aarch64) dd if=/dev/zero of=$@ bs=1 count=0 seek=67108864 2>/dev/null;; \
		loongarch64) dd if=/dev/zero of=$@ bs=1 count=0 seek=5242880 2>/dev/null;; \
		riscv64) dd if=/dev/zero of=$@ bs=1 count=0 seek=33554432 2>/dev/null;; \
	esac

limine/limine:
	rm -rf limine
	git clone https://github.com/limine-bootloader/limine.git --branch=v9.x-binary --depth=1
	$(MAKE) -C limine

.PHONY: apps
apps:
	RUSTFLAGS="-C relocation-model=static" cargo build --target $(RUST_TARGET) --profile $(RUST_PROFILE) $(FEATURES_ARG) $(addprefix -p ,$(APPS))

.PHONY: kernel
kernel:
	$(MAKE) -C boot FEATURES="$(FEATURES)"

$(IMAGE_NAME).iso: limine/limine kernel apps
	rm -rf iso_root
	mkdir -p iso_root/boot
	cp -v boot/kernel iso_root/boot/
	mkdir -p iso_root/boot/apps
	for app in $(APPS); do \
		cp -v $(APPS_TARGET_DIR)/$$app iso_root/boot/apps/$$app; \
	done
	if [ -d $(COMPOSITOR_FONT_DIR) ] && ls $(COMPOSITOR_FONT_DIR)/*.ttf >/dev/null 2>&1; then \
		mkdir -p iso_root/boot/fonts; \
		cp -v $(COMPOSITOR_FONT_DIR)/*.ttf iso_root/boot/fonts/; \
	fi
	mkdir -p iso_root/boot/limine
	cp -v clouds.bmp iso_root/boot/clouds.bmp
	rm -f limine.conf.tmp
	cp limine.conf limine.conf.tmp
ifeq ($(ENABLE_ROOTFS),1)
	echo "    module_path: boot():/boot/apps/rootfs" >> limine.conf.tmp
	echo "    module_cmdline: program=rootfs" >> limine.conf.tmp
endif
	echo "    module_path: boot():/boot/clouds.bmp" >> limine.conf.tmp
	echo "    module_cmdline: image=clouds.bmp" >> limine.conf.tmp
	if [ -d $(COMPOSITOR_FONT_DIR) ] && ls $(COMPOSITOR_FONT_DIR)/*.ttf >/dev/null 2>&1; then \
		for font in $(COMPOSITOR_FONT_DIR)/*.ttf; do \
			name=$$(basename $$font); \
			base=$${name%.ttf}; \
			echo "    module_path: boot():/boot/fonts/$$name" >> limine.conf.tmp; \
			echo "    module_cmdline: font=$$base" >> limine.conf.tmp; \
		done; \
	fi
	cp -v limine.conf.tmp iso_root/boot/limine/limine.conf
ifeq ($(KARCH),riscv64)
	cp templates/riscv-startup.nsh iso_root/startup.nsh
endif
	mkdir -p iso_root/EFI/BOOT
ifeq ($(KARCH),x86_64)
	cp -v limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root/boot/limine/
	cp -v limine/BOOTX64.EFI iso_root/EFI/BOOT/
	cp -v limine/BOOTIA32.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o $(IMAGE_NAME).iso
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
	cp -v limine.conf.tmp iso_root/EFI/BOOT/limine.conf
	cp -v limine.conf.tmp iso_root/limine.conf
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

$(IMAGE_NAME).hdd: limine/limine kernel apps
	rm -f $(IMAGE_NAME).hdd
	dd if=/dev/zero bs=1M count=0 seek=128 of=$@

	sgdisk $(IMAGE_NAME).hdd -n 1:2048 -t 1:ef00
ifeq ($(KARCH),x86_64)
	./limine/limine bios-install $(IMAGE_NAME).hdd
endif
	rm -f limine.conf.tmp
	cp limine.conf limine.conf.tmp
ifeq ($(ENABLE_ROOTFS),1)
	echo "    module_path: boot():/boot/apps/rootfs" >> limine.conf.tmp
	echo "    module_cmdline: program=rootfs" >> limine.conf.tmp
endif
	if [ -d $(COMPOSITOR_FONT_DIR) ] && ls $(COMPOSITOR_FONT_DIR)/*.ttf >/dev/null 2>&1; then \
		for font in $(COMPOSITOR_FONT_DIR)/*.ttf; do \
			name=$$(basename $$font); \
			base=$${name%.ttf}; \
			echo "    module_path: boot():/boot/fonts/$$name" >> limine.conf.tmp; \
			echo "    module_cmdline: font=$$base" >> limine.conf.tmp; \
		done; \
	fi
	mformat -i $(IMAGE_NAME).hdd@@1M
	mmd -i $(IMAGE_NAME).hdd@@1M ::/EFI ::/EFI/BOOT ::/boot ::/boot/limine ::/boot/apps ::/boot/fonts
	mcopy -i $(IMAGE_NAME).hdd@@1M boot/kernel ::/boot
	for app in $(APPS); do \
		mcopy -i $(IMAGE_NAME).hdd@@1M $(APPS_TARGET_DIR)/$$app ::/boot/apps; \
	done
	if [ -d $(COMPOSITOR_FONT_DIR) ] && ls $(COMPOSITOR_FONT_DIR)/*.ttf >/dev/null 2>&1; then \
		for font in $(COMPOSITOR_FONT_DIR)/*.ttf; do \
			mcopy -i $(IMAGE_NAME).hdd@@1M $$font ::/boot/fonts/; \
		done; \
	fi
	mcopy -i $(IMAGE_NAME).hdd@@1M limine.conf.tmp ::/boot/limine
ifeq ($(KARCH),riscv64)
	mcopy -i $(IMAGE_NAME).hdd@@1M limine.conf.tmp ::/EFI/BOOT/limine.conf
endif
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

.PHONY: clean
clean:
	$(MAKE) -C boot clean
	rm -f limine.conf.tmp
	rm -rf iso_root $(IMAGE_NAME).iso $(IMAGE_NAME).hdd

.PHONY: distclean
distclean: clean
	$(MAKE) -C boot distclean
	rm -rf limine ovmf
