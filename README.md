# glb-to-webgpu
Example guest component that draws a 3D gimbal/axis control onto screen with camera/mouse support (axis.glb)

## Source
This code is adapated from code in https://github.com/Formlabs/foxtrot.

## Building
cargo build --release --target wasm32-unknown-unknown

wasm-tools component new ./target/wasm32-unknown-unknown/release/glb-to-webgpu.wasm -o ./glb-to-webgpu-component.wasm

## License
© 2021 [Formlabs](https://formlabs.com)

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
