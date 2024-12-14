alias b := build
alias c := clean
alias d := develop
alias f := format
alias bc := build-cv
alias dc := develop-cv

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
    cargo fmt
    leptosfmt **/*.rs

# Clean
clean:
    cargo clean --verbose && rm -vr ./dist ./style/style.css

# Build the CV using `rendercv` and open it using `xdg-open`
build-cv:
    rendercv render CV.yaml --dont-generate-markdown --dont-generate-png
    cp rendercv_output/Rolv_Apneseth_CV.pdf public/rolvApnesethCV.pdf
    xdg-open public/rolvApnesethCV.pdf

# Re-build and re-open the CV upon any change to `CV.yaml`
develop-cv:
    echo CV.yaml | entr just build-cv
