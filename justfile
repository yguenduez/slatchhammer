deploy:
  cargo build --release --target wasm32-unknown-unknown
  wasm-bindgen --out-dir out --target web target/wasm32-unknown-unknown/release/slatchhammer.wasm --no-typescript
  wasm-opt -Oz -o out/slatchhammer_bg.wasm out/slatchhammer_bg.wasm
