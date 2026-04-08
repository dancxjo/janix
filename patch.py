import sys

with open("kernel/src/simd.rs", "r") as f:
    text = f.read()

new_text = text.replace(
    "unsafe { rt.simd_save(self.buffer) };",
    "crate::kinfo!(\"SIMD_SAVE: dst=0x{:x}\", self.buffer as usize);\n            unsafe { rt.simd_save(self.buffer) };"
)

with open("kernel/src/simd.rs", "w") as f:
    f.write(new_text)

