---
trigger: always_on
---

If you feel tempted to add cfg(target_arch) in scheduler code, stop and move the logic into BootRuntime.

Kernel’s job is bookkeeping and fairness. BootRuntime’s job is “how the CPU actually moves.”

The only acceptable kernel “arch knowledge” is: a pointer to a trait object that can do the dirty work.