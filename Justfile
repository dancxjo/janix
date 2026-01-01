default:
    @just --list

fetch:
    cargo run -p xtask -- fetch

clean:
    cargo run -p xtask -- clean

test arch="all":
    cargo run -p xtask -- test --arch {{arch}}

build:
    cargo run -p xtask -- build

iso env="x86_64":
    cargo run -p xtask -- iso --env {{env}}

run env="x86_64" port="1234":
    cargo run -p xtask -- run --env {{env}} --gdb-port {{port}} --interactive

run-headless env="x86_64" port="1234":
    cargo run -p xtask -- run --env {{env}} --gdb-port {{port}}

debug env="x86_64" port="1234":
    cargo run -p xtask -- run --env {{env}} --gdb --gdb-port {{port}} --interactive

inspect env="x86_64" port="1234":
    gdb -batch -ex "file target/x86_64-unknown-none/debug/bran" -ex "target remote :{{port}}" -ex "bt"
