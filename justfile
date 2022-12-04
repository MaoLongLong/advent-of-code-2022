alias lint := clippy

all: fmt clippy test

fmt: (_cargo "fmt")

clippy: (_cargo "clippy")

build: (_cargo "build")

test: build
  just _cargo test --verbose --all

_cargo command *flags='':
  #!/usr/bin/env bash
  set -euo pipefail
  for d in ./*/; do
    cd "$d"
    if [ -f "Cargo.toml" ]; then
      cargo {{command}} {{flags}}
    fi
    cd ..
  done
