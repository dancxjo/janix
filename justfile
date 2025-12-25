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

iso:
    cargo run -p xtask -- iso {{arg(env, "x86_64")}}

run:
    cargo run -p xtask -- run {{arg(env, "hosted")}}

xtask:
    cargo run -p xtask --
