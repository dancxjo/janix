# ✅ Then I should see that the machine has halted

**Result:** passed | **Duration:** 5701ms

<details open>
<summary>Serial Output</summary>

```
[13888578627] [INFO] frame_alloc: total=516638 free=450333 used=66305
[13889507445] [INFO] Running frame_alloc sanity check...
[13946279292] [INFO] frame_alloc: sanity: single ok
[13954375479] [INFO] frame_alloc: sanity: contig(8) ok
[13957725375] [INFO] Testing paging subsystem...
[13966736817] [INFO] Switched to new address space
[13971207591] [INFO] Paging subsystem test passed
[13973482380] [INFO] Paging subsystem test passed
[13973895111] [INFO] Initializing Kernel Heap...
[13981280841] [INFO] kheap: grew by 64 pages (phys=0x1003f000, virt=0xffffa00000000000)
[13988914698] [INFO] global_alloc: switched to kernel heap
[13989801606] [INFO] Running heap sanity check...
[14010171417] [INFO] kheap: sanity ok
[14013906192] [INFO] kheap: forcing growth...
[14020158867] [INFO] kheap: grew by 16 pages (phys=0x10082000, virt=0xffffa00000040000)
[14047444323] [INFO] kheap: big allocation ok (len=307200)
[14062098765] [INFO] kheap: reserved=268435456 committed=327680
[14063009367] [INFO] System halted

```
</details>
