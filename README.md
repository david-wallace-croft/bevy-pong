# Bevy Pong

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/bevy-pong/blob/main/LICENSE.txt

- Code adapted from
  - "Bevy Tutorial: Pong" by Nolan Tait
    - https://taintedcoders.com/bevy/tutorials/pong-tutorial
  - "How to Export your Bevy project to the Web with WASM" by Biped Potato
    - https://youtu.be/VjXiREbPtJs
  - "Unofficial Bevy Cheat Book" by Ida Borisova  
    - https://bevy-cheatbook.github.io/platforms/wasm.html  
  - "Bevy Examples / Window / Window Settings"  
    - https://bevy.org/examples/window/window-settings/  

## Usage

### Run in a Desktop Window

- cargo run

### Run in a Browser Window

- cargo build --release --target wasm32-unknown-unknown
- wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/cargo-pong.wasm
- http-server -o

## History

- 2025-08-10: Project started
- 2025-10-16: Updated from Bevy v0.16 to v0.17
