default:
    @just --list

fetch:
    cargo run -p xtask -- fetch

check:
    cargo fmt --all --check
    cargo clippy --all-targets -- -D warnings

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
