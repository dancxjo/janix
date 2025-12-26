default:
    @just --list

fetch:
    cargo run -p xtask -- fetch

check:
    cargo fmt --all --check
    cargo clippy --workspace --exclude kernel_x86_64 --exclude kernel_aarch64 -- -D warnings
    cargo clippy -p kernel_x86_64 --target targets/x86_64-thingos.json -Z build-std=core,alloc -- -D warnings
    cargo clippy -p kernel_aarch64 --target targets/aarch64-thingos.json -Z build-std=core,alloc -- -D warnings

test:
    cargo test --workspace

build:
    cargo run -p xtask -- build

iso env="x86_64":
    cargo run -p xtask -- iso --env {{env}}

run env="hosted":
    cargo run -p xtask -- run --env {{env}}

xtask:
    cargo run -p xtask --

inspect:
    @gdb -batch -ex "target remote :1234" -ex "set pagination off" -ex "echo \n--- REGISTERS ---\n" -ex "info registers" -ex "echo \n--- BACKTRACE ---\n" -ex "bt" -ex "echo \n--- INSTRUCTIONS ---\n" -ex "x/10i \$pc" -ex "quit"

die:
    cargo run -p xtask -- kill
