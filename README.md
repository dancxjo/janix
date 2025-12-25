# ThingOS

ThingOS is a graph-first operating system. This repo is intentionally structured to keep the kernel core portable across:
- hosted (normal process)
- x86_64 bare metal (UEFI via OVMF + Limine)
- aarch64 bare metal (UEFI via OVMF + Limine)

## Quickstart

```sh
just fetch
just check
just test
just run env=hosted
just iso env=x86_64
just run env=x86_64
just iso env=aarch64
just run env=aarch64
```

> xtask owns the real build/run logic.
