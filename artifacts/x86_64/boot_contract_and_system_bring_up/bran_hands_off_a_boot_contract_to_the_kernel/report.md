# Scenario: Bran hands off a boot contract to the Kernel

**Architecture**: `x86_64`  
**Feature**: `Boot contract and system bring-up`

[Back to Index](../../../../index.md)

## Steps Summary

| Index | Step | Status | Artifacts |
| :---: | --- | :---: | --- |
| 0 | I boot ThingOS on "x86_64" | ✅ | 📸 📝  |
| 1 | the serial output contains "BOOT:" | ❌ | 📸 📝  |
| 2 | the serial output contains "TICK: switch" | ❌ | 📸 📝  |
| 3 | the serial output contains "SPROUT: I am alive" | ✅ | 📸 📝  |
| 4 | it does not panic | ✅ | 📸 📝  |

## Execution Details

### 1. I boot ThingOS on "x86_64" ✅

![Screenshot](steps/000_i_boot_thingos_on__x86_64_/screen.png)

```

```

---

### 2. the serial output contains "BOOT:" ❌

![Screenshot](steps/001_the_serial_output_contains__boot__/screen.png)

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

### 3. the serial output contains "TICK: switch" ❌

![Screenshot](steps/002_the_serial_output_contains__tick__switch_/screen.png)

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

### 4. the serial output contains "SPROUT: I am alive" ✅

![Screenshot](steps/003_the_serial_output_contains__sprout__i_am_alive_/screen.png)

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

### 5. it does not panic ✅

![Screenshot](steps/004_it_does_not_panic/screen.png)

```
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
TICK: switch 2 -> 1 sp=0xffff800001798d50
```

---

