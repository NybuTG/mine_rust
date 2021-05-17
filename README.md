# mine_rust
This project is a clone of a very popular game, made for fun.

I'm using: Macroquad for game stuff like drawing and Rapier for physics


# IDEAS:
- WASM + FFI for handling mods. Would allow other coding languages for mods too! How cool

- Resources for the above:
-https://medium.com/wasmer/executing-webassembly-in-your-rust-application-d5cd32e8ce46

# TODO
- Replace ron-rs with https://github.com/a1phyr/assets_manager. 
- Why: Can open RON files directly and loads into cache without other code. Besides that it can also load images
and hot reload them.
  
- **Steps**:
1. Load all cube defs into the manager
2. Get all texture files
3. Convert the pngs to macroquad textures
4. Insert textures into manager