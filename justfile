default:
    @just --list

# Auto-format the source tree
fmt:
    treefmt

# Run the project locally
watch $RUST_BACKTRACE="1":
    cargo leptos watch

# Run cargo in release mode (prints red panic)
watch-release:
    cargo leptos watch --release

# Run tests (backend & frontend)
test:
    cargo watch -- cargo leptos test

