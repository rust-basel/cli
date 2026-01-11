

install:
    cargo install --path .
    
run *args:
    cargo run {{args}}

watch:
    cargo watch -x run

verify: lint test

test:
    cargo test

lint:
    cargo fmt --all -- --check
    cargo clippy

fmt:
    cargo fmt
    cargo fix --allow-dirty --allow-stageds