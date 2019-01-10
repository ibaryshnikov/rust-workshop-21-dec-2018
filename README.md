## Rust + WebAssembly workshop
Simple math and DOM examples

### Building
```bash
wasm-pack build --target no-modules
```

### Running
Use your favourite static server

In case of Rust it's also possible to use [https](https://crates.io/crates/https)
```bash
cargo install https
```
Then navigate to the root of this repo and run
```bash
http
```
and then open [localhost:8000](http://localhost:8000) in your browser
