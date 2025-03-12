cargo build --release --target wasm32-unknown-unknown

wasm-tools component new ./target/wasm32-unknown-unknown/release/glb-to-webgpu.wasm -o ./glb-to-webgpu-component.wasm
