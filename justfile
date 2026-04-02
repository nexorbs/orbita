prepare:
    cargo install dioxus-cli

dev:
    dx serve --hot-patch

f:
    cargo fmt --all
    cargo clippy --all -- -D warnings