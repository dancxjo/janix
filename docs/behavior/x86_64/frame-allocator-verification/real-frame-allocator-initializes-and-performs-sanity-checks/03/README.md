# ✅ And the serial output should contain "frame_alloc: base="

**Result:** passed | **Duration:** 23ms

## Registers

```
{"return": {}}

```

<details open>
<summary>Serial Output</summary>

```
[16413300948] [INFO] frame_alloc: total=516638 free=450333 used=66305
[16414339689] [INFO] Running frame_alloc sanity check...
[16551406641] [INFO] frame_alloc: sanity: single ok
[16555573782] [INFO] frame_alloc: sanity: contig(8) ok
[16556210616] [INFO] Testing paging subsystem...
[16575993753] [INFO] Switched to new address space
[16576621908] [INFO] Paging subsystem test passed
[16577207922] [INFO] Paging subsystem test passed
[16577778327] [INFO] Initializing Kernel Heap...
[16591974861] [INFO] kheap: grew by 64 pages (phys=0x1003f000, virt=0xffffa00000000000)
[16593431712] [INFO] global_alloc: switched to kernel heap
[16600404843] [INFO] Running heap sanity check...
[16631811405] [INFO] kheap: sanity ok
[16639799781] [INFO] kheap: forcing growth...
[16646155317] [INFO] kheap: grew by 16 pages (phys=0x10082000, virt=0xffffa00000040000)
[16677230262] [INFO] kheap: big allocation ok (len=307200)
[16691939187] [INFO] kheap: reserved=268435456 committed=327680
[16695465732] [INFO] System halted

```
</details>
