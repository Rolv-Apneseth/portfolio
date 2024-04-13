alias b := build
alias d := develop

# List commands
default:
    @just --list

# Build
build:
    cargo build --release

# Serve web app for development
develop:
    trunk serve --open

format:
    leptosfmt

# Clean
clean:
    cargo clean --verbose
