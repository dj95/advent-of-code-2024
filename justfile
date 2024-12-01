run day part:
    cargo run -r -p {{day}} --bin {{part}}

test day:
    RUST_LOG=trace cargo watch -x "nextest run -p {{day}} --no-capture"

create day:
    cargo generate --path ./daily-template --name {{day}}

lint:
    cargo watch -x "clippy --all-targets --all-features"

bench day:
    cargo bench -p "{{day}}"
