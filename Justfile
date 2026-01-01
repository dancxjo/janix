default:
    @just --list

fetch:
    cargo run -p xtask -- fetch

clean:
    cargo run -p xtask -- clean

die:
    @echo "Killing all QEMU and xtask instances..."
    -pkill -f "[q]emu-system-"
    -pkill -f "target/debug/[x]task"
    @echo "Resetting terminal..."
    reset

test arch="all":
    cargo run -p xtask -- test --arch {{arch}}

build:
    cargo run -p xtask -- build

iso env="x86_64":
    cargo run -p xtask -- iso --env {{env}}

run env="x86_64":
    cargo run -p xtask -- run --env {{env}} --interactive --gdb

run-headless env="x86_64":
    cargo run -p xtask -- run --env {{env}} --gdb

debug env="x86_64":
    cargo run -p xtask -- run --env {{env}} --debug

debug-console:
    socat -,raw,echo=0 unix-connect:/tmp/thingos-serial.sock

inspect env="x86_64" port="0":
    cargo run -p xtask -- inspect --env {{env}} --port {{port}}

# Aliases for tab completion

# run aliases
run-x86_64:
    @just run x86_64
run-aarch64:
    @just run aarch64
run-riscv64:
    @just run riscv64
run-loongarch64:
    @just run loongarch64

# test aliases
test-x86_64:
    @just test x86_64
test-aarch64:
    @just test aarch64
test-riscv64:
    @just test riscv64
test-loongarch64:
    @just test loongarch64

# debug aliases
debug-x86_64:
    @just debug x86_64
debug-aarch64:
    @just debug aarch64
debug-riscv64:
    @just debug riscv64
debug-loongarch64:
    @just debug loongarch64

# inspect aliases
inspect-x86_64:
    @just inspect x86_64
inspect-aarch64:
    @just inspect aarch64
inspect-riscv64:
    @just inspect riscv64
inspect-loongarch64:
    @just inspect loongarch64
