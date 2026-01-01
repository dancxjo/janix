# Nuke built-in rules and variables.
MAKEFLAGS += -rR
.SUFFIXES:

# Convenience macro to reliably declare user overridable variables.
override USER_VARIABLE = $(if $(filter $(origin $(1)),default undefined),$(eval override $(1) := $(2)))

# Target architecture to build for. Default to x86_64.
$(call USER_VARIABLE,KARCH,x86_64)

# Default user QEMU flags. These are appended to the QEMU command calls.
$(call USER_VARIABLE,QEMUFLAGS,-m 2G -serial stdio)

# Stable gdb stub ports by architecture so inspect targets can auto-connect.
GDB_PORT_x86_64 ?= 1234
GDB_PORT_aarch64 ?= 2234
GDB_PORT_riscv64 ?= 3234
GDB_PORT_loongarch64 ?= 4234
GDB_PORT = $(if $(GDB_PORT_$(KARCH)),$(GDB_PORT_$(KARCH)),1234)

# Selectable gdb binary (override GDB_BIN or GDB_BIN_<arch> if needed).
GDB_BIN ?= gdb
GDB_BIN_x86_64 ?= $(GDB_BIN)
GDB_BIN_aarch64 ?= gdb-multiarch
GDB_BIN_riscv64 ?= gdb-multiarch
GDB_BIN_loongarch64 ?= gdb-multiarch
GDB_TOOL = $(if $(GDB_BIN_$(KARCH)),$(GDB_BIN_$(KARCH)),$(GDB_BIN))

# Optional architecture hint for gdb.
GDB_ARCH_x86_64 := i386:x86-64
GDB_ARCH_aarch64 := aarch64
GDB_ARCH_riscv64 := riscv:rv64
GDB_ARCH_loongarch64 := loongarch64
GDB_ARCH = $(GDB_ARCH_$(KARCH))

# Opt-in gdb stub for QEMU. WITH_GDB=1 appends -gdb tcp::<port>; set GDB_PAUSE=1 to also freeze with -S.
WITH_GDB ?= 0
GDB_PAUSE ?= 0
QEMU_GDB_FLAGS = -gdb tcp::$(GDB_PORT)
ifeq ($(GDB_PAUSE),1)
QEMU_GDB_FLAGS += -S
endif

ifeq ($(WITH_GDB),1)
override QEMUFLAGS += $(QEMU_GDB_FLAGS)
endif

override IMAGE_NAME := template-$(KARCH)
override BRAN_BIN = crates/bran/bin-$(KARCH)/kernel
override SPROUT_BIN = crates/sprout/bin-$(KARCH)/sprout

.PHONY: all
all: $(IMAGE_NAME).iso

.PHONY: all-hdd
all-hdd: $(IMAGE_NAME).hdd

.PHONY: run
run: run-$(KARCH)

.PHONY: run-hdd
run-hdd: run-hdd-$(KARCH)

.PHONY: run-x86_64
run-x86_64: ovmf/ovmf-code-$(KARCH).fd ovmf/ovmf-vars-$(KARCH).fd $(IMAGE_NAME).iso
	qemu-system-$(KARCH) \
		-M q35 \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-$(KARCH).fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-$(KARCH).fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS)

.PHONY: run-hdd-x86_64
run-hdd-x86_64: ovmf/ovmf-code-$(KARCH).fd ovmf/ovmf-vars-$(KARCH).fd $(IMAGE_NAME).hdd
	qemu-system-$(KARCH) \
		-M q35 \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-$(KARCH).fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-$(KARCH).fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS)

.PHONY: launch-aarch64
launch-aarch64: ovmf/ovmf-code-$(KARCH).fd ovmf/ovmf-vars-$(KARCH).fd $(IMAGE_NAME).iso
	qemu-system-$(KARCH) \
		-M virt \
		-cpu cortex-a72 \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-$(KARCH).fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-$(KARCH).fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS)

.PHONY: run-aarch64
run-aarch64:
	$(MAKE) launch-aarch64 KARCH=aarch64

.PHONY: launch-hdd-aarch64
launch-hdd-aarch64: ovmf/ovmf-code-$(KARCH).fd ovmf/ovmf-vars-$(KARCH).fd $(IMAGE_NAME).hdd
	qemu-system-$(KARCH) \
		-M virt \
		-cpu cortex-a72 \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-$(KARCH).fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-$(KARCH).fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS)

.PHONY: run-hdd-aarch64
run-hdd-aarch64:
	$(MAKE) launch-hdd-aarch64 KARCH=aarch64

.PHONY: launch-riscv64
launch-riscv64: ovmf/ovmf-code-$(KARCH).fd ovmf/ovmf-vars-$(KARCH).fd $(IMAGE_NAME).iso
	qemu-system-$(KARCH) \
		-M virt \
		-cpu rv64 \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-$(KARCH).fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-$(KARCH).fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS)

.PHONY: run-riscv64
run-riscv64:
	$(MAKE) launch-riscv64 KARCH=riscv64

.PHONY: launch-hdd-riscv64
launch-hdd-riscv64: ovmf/ovmf-code-$(KARCH).fd ovmf/ovmf-vars-$(KARCH).fd $(IMAGE_NAME).hdd
	qemu-system-$(KARCH) \
		-M virt \
		-cpu rv64 \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-$(KARCH).fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-$(KARCH).fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS)

.PHONY: run-hdd-riscv64
run-hdd-riscv64:
	$(MAKE) launch-hdd-riscv64 KARCH=riscv64

.PHONY: launch-loongarch64
launch-loongarch64: ovmf/ovmf-code-$(KARCH).fd ovmf/ovmf-vars-$(KARCH).fd $(IMAGE_NAME).iso
	qemu-system-$(KARCH) \
		-M virt \
		-cpu la464 \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-$(KARCH).fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-$(KARCH).fd \
		-cdrom $(IMAGE_NAME).iso \
		$(QEMUFLAGS)

.PHONY: run-loongarch64
run-loongarch64:
	$(MAKE) launch-loongarch64 KARCH=loongarch64

.PHONY: launch-hdd-loongarch64
launch-hdd-loongarch64: ovmf/ovmf-code-$(KARCH).fd ovmf/ovmf-vars-$(KARCH).fd $(IMAGE_NAME).hdd
	qemu-system-$(KARCH) \
		-M virt \
		-cpu la464 \
		-device ramfb \
		-device qemu-xhci \
		-device usb-kbd \
		-device usb-mouse \
		-drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-$(KARCH).fd,readonly=on \
		-drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-$(KARCH).fd \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS)

.PHONY: run-hdd-loongarch64
run-hdd-loongarch64:
	$(MAKE) launch-hdd-loongarch64 KARCH=loongarch64

.PHONY: run-gdb
run-gdb:
	$(MAKE) run KARCH=$(KARCH) WITH_GDB=1

.PHONY: run-gdb-x86_64
run-gdb-x86_64:
	$(MAKE) run-x86_64 KARCH=x86_64 WITH_GDB=1

.PHONY: run-gdb-aarch64
run-gdb-aarch64:
	$(MAKE) run-aarch64 KARCH=aarch64 WITH_GDB=1

.PHONY: run-gdb-riscv64
run-gdb-riscv64:
	$(MAKE) run-riscv64 KARCH=riscv64 WITH_GDB=1

.PHONY: run-gdb-loongarch64
run-gdb-loongarch64:
	$(MAKE) run-loongarch64 KARCH=loongarch64 WITH_GDB=1


.PHONY: run-bios
run-bios: $(IMAGE_NAME).iso
	qemu-system-$(KARCH) \
		-M q35 \
		-cdrom $(IMAGE_NAME).iso \
		-boot d \
		$(QEMUFLAGS)

.PHONY: run-hdd-bios
run-hdd-bios: $(IMAGE_NAME).hdd
	qemu-system-$(KARCH) \
		-M q35 \
		-hda $(IMAGE_NAME).hdd \
		$(QEMUFLAGS)

# OVMF pinned release - the latest drops occasionally regress
OVMF_RELEASE ?= edk2-stable202508-r1
OVMF_ARCHIVE = $(OVMF_RELEASE)-bin.tar.xz
OVMF_URL = https://github.com/rust-osdev/ovmf-prebuilt/releases/download/$(OVMF_RELEASE)/$(OVMF_ARCHIVE)

ovmf/$(OVMF_ARCHIVE):
	mkdir -p ovmf
	curl -fLo $@ $(OVMF_URL)

ovmf/ovmf-code-x86_64.fd ovmf/ovmf-vars-x86_64.fd: ovmf/$(OVMF_ARCHIVE)
	mkdir -p ovmf
	tar -xJf ovmf/$(OVMF_ARCHIVE) -C ovmf --strip-components=1 $(OVMF_RELEASE)-bin/x64/code.fd $(OVMF_RELEASE)-bin/x64/vars.fd
	mv ovmf/x64/code.fd ovmf/ovmf-code-x86_64.fd
	mv ovmf/x64/vars.fd ovmf/ovmf-vars-x86_64.fd
	rmdir ovmf/x64

ovmf/ovmf-code-aarch64.fd ovmf/ovmf-vars-aarch64.fd: ovmf/$(OVMF_ARCHIVE)
	mkdir -p ovmf
	tar -xJf ovmf/$(OVMF_ARCHIVE) -C ovmf --strip-components=1 $(OVMF_RELEASE)-bin/aarch64/code.fd $(OVMF_RELEASE)-bin/aarch64/vars.fd
	mv ovmf/aarch64/code.fd ovmf/ovmf-code-aarch64.fd
	mv ovmf/aarch64/vars.fd ovmf/ovmf-vars-aarch64.fd
	rmdir ovmf/aarch64

ovmf/ovmf-code-riscv64.fd ovmf/ovmf-vars-riscv64.fd: ovmf/$(OVMF_ARCHIVE)
	mkdir -p ovmf
	tar -xJf ovmf/$(OVMF_ARCHIVE) -C ovmf --strip-components=1 $(OVMF_RELEASE)-bin/riscv64/code.fd $(OVMF_RELEASE)-bin/riscv64/vars.fd
	mv ovmf/riscv64/code.fd ovmf/ovmf-code-riscv64.fd
	mv ovmf/riscv64/vars.fd ovmf/ovmf-vars-riscv64.fd
	rmdir ovmf/riscv64

ovmf/ovmf-code-loongarch64.fd ovmf/ovmf-vars-loongarch64.fd: ovmf/$(OVMF_ARCHIVE)
	mkdir -p ovmf
	tar -xJf ovmf/$(OVMF_ARCHIVE) -C ovmf --strip-components=1 $(OVMF_RELEASE)-bin/loongarch64/code.fd $(OVMF_RELEASE)-bin/loongarch64/vars.fd
	mv ovmf/loongarch64/code.fd ovmf/ovmf-code-loongarch64.fd
	mv ovmf/loongarch64/vars.fd ovmf/ovmf-vars-loongarch64.fd
	rmdir ovmf/loongarch64


limine/limine:
	rm -rf limine
	git clone https://github.com/limine-bootloader/limine.git --branch=v10.x-binary --depth=1
	$(MAKE) -C limine

.PHONY: bran
bran:
	$(MAKE) -C crates/bran

.PHONY: sprout
sprout:
	$(MAKE) -C crates/sprout

$(IMAGE_NAME).iso: limine/limine bran sprout
	rm -rf iso_root_$(KARCH)
	mkdir -p iso_root_$(KARCH)/boot
	mkdir -p iso_root_$(KARCH)/boot/modules
	cp -v $(BRAN_BIN) iso_root_$(KARCH)/boot/
	cp -v $(SPROUT_BIN) iso_root_$(KARCH)/boot/modules/
	mkdir -p iso_root_$(KARCH)/boot/limine
	cp -v limine.conf iso_root_$(KARCH)/boot/limine/
	mkdir -p iso_root_$(KARCH)/EFI/BOOT
ifeq ($(KARCH),x86_64)
	cp -v limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root_$(KARCH)/boot/limine/
	cp -v limine/BOOTX64.EFI iso_root_$(KARCH)/EFI/BOOT/
	cp -v limine/BOOTIA32.EFI iso_root_$(KARCH)/EFI/BOOT/
	xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root_$(KARCH) -o $(IMAGE_NAME).iso
	./limine/limine bios-install $(IMAGE_NAME).iso
endif
ifeq ($(KARCH),aarch64)
	cp -v limine/limine-uefi-cd.bin iso_root_$(KARCH)/boot/limine/
	cp -v limine/BOOTAA64.EFI iso_root_$(KARCH)/EFI/BOOT/
	xorriso -as mkisofs \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root_$(KARCH) -o $(IMAGE_NAME).iso
endif
ifeq ($(KARCH),riscv64)
	cp -v limine/limine-uefi-cd.bin iso_root_$(KARCH)/boot/limine/
	cp -v limine/BOOTRISCV64.EFI iso_root_$(KARCH)/EFI/BOOT/
	xorriso -as mkisofs \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root_$(KARCH) -o $(IMAGE_NAME).iso
endif
ifeq ($(KARCH),loongarch64)
	cp -v limine/limine-uefi-cd.bin iso_root_$(KARCH)/boot/limine/
	cp -v limine/BOOTLOONGARCH64.EFI iso_root_$(KARCH)/EFI/BOOT/
	xorriso -as mkisofs \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root_$(KARCH) -o $(IMAGE_NAME).iso
endif
	rm -rf iso_root_$(KARCH)

$(IMAGE_NAME).hdd: limine/limine bran
	rm -f $(IMAGE_NAME).hdd
	dd if=/dev/zero bs=1M count=0 seek=64 of=$(IMAGE_NAME).hdd
	sgdisk $(IMAGE_NAME).hdd -n 1:2048 -t 1:ef00
ifeq ($(KARCH),x86_64)
	./limine/limine bios-install $(IMAGE_NAME).hdd
endif
	mformat -i $(IMAGE_NAME).hdd@@1M
	mmd -i $(IMAGE_NAME).hdd@@1M ::/EFI ::/EFI/BOOT ::/boot ::/boot/limine
	mcopy -i $(IMAGE_NAME).hdd@@1M $(BRAN_BIN) ::/boot
	mcopy -i $(IMAGE_NAME).hdd@@1M limine.conf ::/boot/limine
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
	$(MAKE) -C crates/bran clean
	$(MAKE) -C crates/sprout clean
	rm -rf iso_root_* $(IMAGE_NAME).iso $(IMAGE_NAME).hdd

.PHONY: distclean
distclean: clean
	$(MAKE) -C crates/bran distclean
	$(MAKE) -C crates/sprout distclean
	rm -rf limine ovmf

.PHONY: bdd
bdd:
	ARCH=$(if $(ARCH),$(ARCH),all) cargo run -p ci-bdd-runner

.PHONY: inspect
inspect:
	$(MAKE) inspect-$(KARCH) KARCH=$(KARCH)

.PHONY: inspect-%
inspect-%: KARCH=$*
inspect-%:
	$(MAKE) bran KARCH=$(KARCH)
	@$(GDB_TOOL) -batch \
		$(if $(GDB_ARCH),-ex "set architecture $(GDB_ARCH)") \
		-ex "file $(BRAN_BIN)" \
		-ex "target remote :$(GDB_PORT)" \
		-ex "set pagination off" \
		-ex "echo \n--- REGISTERS ---\n" \
		-ex "info registers" \
		-ex "echo \n--- BACKTRACE ---\n" \
		-ex "bt" \
		-ex "echo \n--- INSTRUCTIONS ---\n" \
		-ex 'x/10i $$pc' \
		-ex "echo \n--- SOURCE ---\n" \
		-ex 'list *$$pc' \
		-ex "quit"
