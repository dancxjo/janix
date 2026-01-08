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

update-docs:
    cargo run -p xtask -- update-docs

test +args="":
    cargo run -p xtask -- test {{args}}

test-all:
    @just test-suite core
    @just test-suite input
    @just test-suite bloom
    @just test-suite apps

smoke arch="all" feature="all":
    cargo run -p xtask -- test --arch {{arch}} --smoke --feature {{feature}}

test-suite suite arch="all":
    cargo run -p xtask -- test --arch {{arch}} --suite {{suite}}
    cargo run -p xtask -- update-docs

build:
    cargo run -p xtask -- build

iso env="x86_64":
    cargo run -p xtask -- iso --env {{env}}

run env="x86_64":
    cargo run -p xtask -- run --env {{env}} --interactive --gdb

run-all:
    @echo "Starting all architectures in screen..."
    # Create detached session with first window (x86_64)
    screen -d -m -S thingos-all -t x86_64 just run x86_64
    # Create other windows
    screen -S thingos-all -X screen -t aarch64 just run aarch64
    screen -S thingos-all -X screen -t riscv64 just run riscv64
    screen -S thingos-all -X screen -t loongarch64 just run loongarch64
    # Split vertically (Left | Right)
    screen -S thingos-all -X split -v
    # Focus Right
    screen -S thingos-all -X focus
    # Split Right Horizontally (Top Right / Bot Right)
    screen -S thingos-all -X split
    # Focus Bot Right, Select loongarch64 (3)
    screen -S thingos-all -X focus
    screen -S thingos-all -X select 3
    # Focus Top Right, Select riscv64 (2)
    screen -S thingos-all -X focus
    screen -S thingos-all -X select 2
    # Focus Left
    screen -S thingos-all -X focus
    # Split Left Horizontally (Top Left / Bot Left)
    screen -S thingos-all -X split
    # Focus Bot Left, Select aarch64 (1)
    screen -S thingos-all -X focus
    screen -S thingos-all -X select 1
    # Focus Top Left, Select x86_64 (0)
    screen -S thingos-all -X focus
    screen -S thingos-all -X select 0
    # Attach
    screen -r thingos-all

run-headless env="x86_64":
    cargo run -p xtask -- run --env {{env}} --gdb

debug env="x86_64":
    cargo run -p xtask -- run --env {{env}} --debug

debug-console:
    socat -,raw,echo=0 unix-connect:/tmp/thingos-serial.sock

inspect env="x86_64" port="0":
    cargo run -p xtask -- inspect --env {{env}} --port {{port}}

# Primary architecture aliases
intel:
    @just run x86_64

arm:
    @just run aarch64

risc:
    @just run riscv64

loong:
    @just run loongarch64

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
    @just test --arch x86_64
test-aarch64:
    @just test --arch aarch64
test-riscv64:
    @just test --arch riscv64
test-loongarch64:
    @just test --arch loongarch64

# smoke aliases
smoke-x86_64:
    @just smoke x86_64
smoke-aarch64:
    @just smoke aarch64
smoke-riscv64:
    @just smoke riscv64
smoke-loongarch64:
    @just smoke loongarch64

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

# CI tripwire for graph usage in memory modules
check-memory:
    @bash tools/ci/no_graph_in_memory.sh

# CI target to run all checks
ci: fetch check-memory build
    @echo "CI passed."
