cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/debug/thermo.wasm dist
cp index.html dist