cd $(dirname $0)
set -eu
rm schema.json || true
wget https://xtp.dylibso.com/assets/wasm/schema.json
cargo install cargo-typify
cargo typify ./schema.json --output ./src/schema.rs