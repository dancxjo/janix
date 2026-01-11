# Thing-OS BDD Artifacts

This directory holds the BDD feature files and generated test artifacts.

## Architectures

Results are organized by target architecture:

- [x86_64](./results/x86_64/)
- [aarch64](./results/aarch64/)
- [riscv64](./results/riscv64/)
- [loongarch64](./results/loongarch64/)

## Features

Feature files live in `docs/behave/features/`:

- [boot](./features/boot.feature)
- [kernel_heap](./features/kernel_heap.feature)
- [memory](./features/memory.feature)
- [memory_real](./features/memory_real.feature)
- [paging](./features/paging.feature)
- [simple-boot](./features/simple-boot.feature)
- [threading](./features/threading.feature)
- [time](./features/time.feature)

## Notes

- Run `just behave` to generate results under `docs/behave/results/${ARCH}`.
- Scenario READMEs include inline logs and screenshots for each step.
