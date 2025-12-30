default:
    @just --list

fetch:
    cargo run -p xtask -- fetch

clean:
    cargo run -p xtask -- clean

check:
    cargo fmt --all --check
    cargo clippy --workspace --exclude kernel_x86_64 --exclude kernel_aarch64 --exclude thing_std -- -D warnings
    cargo clippy -p kernel_x86_64 --target targets/x86_64-thingos.json -Z build-std=core,alloc -- -D warnings
    cargo clippy -p kernel_aarch64 --target targets/aarch64-thingos.json -Z build-std=core,alloc -- -D warnings

test:
    cargo test --workspace

build:
    cargo run -p xtask -- build

iso env="x86_64":
    cargo run -p xtask -- iso --env {{env}}

run env="x86_64" port="1234":
    cargo run -p xtask -- run --env {{env}} --gdb-port {{port}}

play env="x86_64" port="1234":
    cargo run -p xtask -- run --env {{env}} --interactive --gdb-port {{port}}

xtask:
    cargo run -p xtask --

inspect env="x86_64" port="1234":
    @gdb -batch -ex "file target/{{env}}-thingos/debug/kernel_{{env}}" -ex "target remote :{{port}}" -ex "set pagination off" -ex "echo \n--- REGISTERS ---\n" -ex "info registers" -ex "echo \n--- BACKTRACE ---\n" -ex "bt" -ex "echo \n--- INSTRUCTIONS ---\n" -ex "x/10i \$pc" -ex "echo \n--- SOURCE ---\n" -ex "list *\$pc" -ex "quit"

die:
    cargo run -p xtask -- kill
