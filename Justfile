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

run env="x86_64":
    cargo run -p xtask -- run --env {{env}} --interactive --gdb

run-headless env="x86_64":
    cargo run -p xtask -- run --env {{env}} --gdb

debug env="x86_64" port="0":
    cargo run -p xtask -- run --env {{env}} --gdb --gdb-port {{port}} --interactive --frozen

inspect env="x86_64" port="0":
    cargo run -p xtask -- inspect --env {{env}} --port {{port}}
