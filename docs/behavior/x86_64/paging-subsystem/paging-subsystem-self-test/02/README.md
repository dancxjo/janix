# ✅ Then the serial output should contain "Testing paging subsystem..."

**Result:** passed | **Duration:** 5499ms

## Screenshots

### After
![After](./after.png)

## Registers

```
{"return": {}}

```

<details open>
<summary>Serial Output</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[16673765460] [INFO] System booted
[16687709379] [INFO] boot: phys ranges=35 modules=0
[16689509661] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[16691502465] [INFO] Initializing Real Frame Allocator...
[16696595124] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[17401026126] [INFO] frame_alloc: total=516638 free=511720 used=4918
[17403861618] [INFO] Running frame_alloc sanity check...
[17410756836] [INFO] frame_alloc: sanity: single ok
[17415410859] [INFO] frame_alloc: sanity: contig(8) ok
[17417425641] [INFO] Testing paging subsystem...
[17426789160] [INFO] Switched to new address space
[17430112689] [INFO] Paging subsystem test passed
[17430589968] [INFO] System halted

```
</details>
