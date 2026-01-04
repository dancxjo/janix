# BDD Test Report Index

## Architecture: x86_64

### Feature: Boot contract and system bring-up

| Scenario | Status | Steps | Report |
| --- | :---: | :---: | :---: |
| Bran hands off a boot contract to the Kernel | ❌ | 5 | [View Report](x86_64/boot_contract_and_system_bring_up/bran_hands_off_a_boot_contract_to_the_kernel/report.md) |

### Feature: Multi-architecture behavioral parity

| Scenario | Status | Steps | Report |
| --- | :---: | :---: | :---: |
| Boot reaches kernel ready on each architecture | ⚠️ | 2 | [View Report](x86_64/multi_architecture_behavioral_parity/boot_reaches_kernel_ready_on_each_architecture/report.md) |
| The syscall ABI returns a structured result on each architecture | ⚠️ | 2 | [View Report](x86_64/multi_architecture_behavioral_parity/the_syscall_abi_returns_a_structured_result_on_each_architecture/report.md) |

### Feature: Swappable display backends

| Scenario | Status | Steps | Report |
| --- | :---: | :---: | :---: |
| Bloom can paint a solid color on any provider | ⚠️ | 2 | [View Report](x86_64/swappable_display_backends/bloom_can_paint_a_solid_color_on_any_provider/report.md) |
| Bloom cannot access display bytespace without capability | ⚠️ | 1 | [View Report](x86_64/swappable_display_backends/bloom_cannot_access_display_bytespace_without_capability/report.md) |
| Limine framebuffer provider exposes a primary display bytespace | ⚠️ | 1 | [View Report](x86_64/swappable_display_backends/limine_framebuffer_provider_exposes_a_primary_display_bytespace/report.md) |

