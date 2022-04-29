# Roguelike
implementing [this](https://bfnightly.bracketproductions.com/rustbook/chapter_0.html) tutorial to learn more about ECS



# Build 

## Wasm 
```
cargo build --release --target wasm32-unknown-unknown && wasm-bindgen target\wasm32-unknown-unknown\release\roguelike.wasm --out-dir wasm --no-modules --no-typescript
```
