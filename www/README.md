# bevy_gaussian_splatting for web

## wasm support

to build wasm run:

```bash
cargo build --example=minimal --target wasm32-unknown-unknown --release
```

to generate bindings:
> `wasm-bindgen --out-dir ./www/out/ --target web ./target/wasm32-unknown-unknown/release/examples/minimal.wasm`


open a live server of `index.html` and append args: `?my_string=hello_world`
