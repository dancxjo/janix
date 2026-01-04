# Scenario: Bloom cannot access display bytespace without capability

**Architecture**: `x86_64`  
**Feature**: `Swappable display backends`

[Back to Index](../../../../index.md)

## Steps Summary

| Index | Step | Status | Artifacts |
| :---: | --- | :---: | --- |
| 0 | I boot ThingOS on "x86_64" with display provider "limine_fb" | ⏭️ | 📝  |

## Execution Details

### 1. I boot ThingOS on "x86_64" with display provider "limine_fb" 

```
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
TICK: switch 2 -> 1 sp=0xffff800001798d50
TICK: switch 1 -> 2 sp=0xffff8000017e3d50
```

---

