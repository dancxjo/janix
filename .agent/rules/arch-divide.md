---
trigger: always_on
glob:
description:
---

For the love of god, don't put architecture-specific code in the kernel. ArchSpecific code gets abstracted by the BootRuntime in the bran. For userspace, arch-specific code like syscalls should be in the Stem.