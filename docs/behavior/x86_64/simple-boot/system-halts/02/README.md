# ✅ Then I should see that the machine has halted

**Result:** passed | **Duration:** 5710ms

<details open>
<summary>Serial Output</summary>

```
[13595136060] [INFO] frame_alloc: total=516638 free=511667 used=4971
[13597676433] [INFO] Running frame_alloc sanity check...
[13601281815] [INFO] frame_alloc: sanity: single ok
[13607189772] [INFO] frame_alloc: sanity: contig(8) ok
[13609795122] [INFO] Testing paging subsystem...
[13618731357] [INFO] Switched to new address space
[13621994793] [INFO] Paging subsystem test passed
[13622467551] [INFO] Paging subsystem test passed
[13622902821] [INFO] Initializing Kernel Heap...
[13630133550] [INFO] kheap: grew by 64 pages (phys=0x3f000, virt=0xffffa00000000000)
[13635525453] [INFO] global_alloc: switched to kernel heap
[13636582773] [INFO] Running heap sanity check...
[13655852529] [INFO] kheap: sanity ok
[13658620734] [INFO] kheap: forcing growth...
[13661526780] [INFO] kheap: grew by 16 pages (phys=0x82000, virt=0xffffa00000040000)
[13686692976] [INFO] kheap: big allocation ok (len=307200)
[13699595877] [INFO] kheap: reserved=268435456 committed=327680
[13701231258] [INFO] System halted

```
</details>
