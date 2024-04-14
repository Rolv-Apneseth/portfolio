alias b := build
alias c := clean
alias d := develop
alias f := format

# List commands
default:
    @just --list

# Build
build:
    trunk build --release

# Serve web app for development
develop:
    trunk serve --open

# Format
format:
    leptosfmt

# Clean
clean:
    cargo clean --verbose && rm -vr ./dist ./style/style.css
