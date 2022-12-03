all: fmt clippy test

fmt: (_cargo "fmt")

clippy: (_cargo "clippy")

test: (_cargo "test")

_cargo command:
  #!/usr/bin/env bash
  set -euxo pipefail
  for d in ./*/; do
    cd "$d"
    if [ -f "Cargo.toml" ]; then
      cargo {{command}}
    fi
    cd ..
  done
