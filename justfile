cargo command:
  #!/usr/bin/env bash
  set -euxo pipefail
  for d in ./*/; do
    cd "$d"
    if [ -f "Cargo.toml" ]; then
      cargo {{command}}
    fi
    cd ..
  done
