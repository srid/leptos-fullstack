default:
    @just --list

# Auto-format the source tree
fmt:
    treefmt

# Run the project locally
watch *ARGS:
    rm -rf ./frontend/dist
    watch-project