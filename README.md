# rust_interop
Template code for `struct` and `func` interop between `Rust` and `C`, `Rust`, `Swift`, `JavaScript` (through binary `dylib`/`WASM`; tested on macOS).

## Prerequisites
- [Rust](https://www.rust-lang.org/learn/get-started)
- `cc`, `swiftc` – for C and Swift interop
- `wasm-pack`: `cargo install wasm-pack` – for in-browser JavaScript interop
    - `python3` – for hosting local Web server

## Build and run
- `./build.sh` – to build Rust lib
    - `cd interop/{c|rust|swift|web}; ./build.sh` – to build interop code
        - `cd interop/web; ./run.sh` – to host local Web server and to open `index.html` in browser
