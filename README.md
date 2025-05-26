# Test WASM with Rust

Try [this](https://medium.com/@agnislav/wasm-rust-vite-and-pnpm-workspace-db561f77c5ca)

## Command

Build for Web:

```bash
wasm-pack build --target web
```

or

```bash
make wasm
```

## Run a client with Deno

```bash
deno run --allow-net --allow-read https://deno.land/std@0.224.0/http/file_server.ts .
```

or

```bash
make serve-deno
```

Open in a browser: [http://localhost:4507](http://localhost:4507)

## Debugging

### TLDR

The main topics for enabling DWARF debugging are:

- Configure `wasm-pack` to not strip DWARF debug info via the `Cargo.toml`

```toml
[package.metadata.wasm-pack.profile.dev.wasm-bindgen]
dwarf-debug-info = true
```

- Run `wasm-pack` with the `--dev` profile
- Install [Chrome DevTools C++ DWARF debugging](https://chromewebstore.google.com/detail/cc++-devtools-support-dwa/pdcpmagijalfljmkmjngeonclgbbannb)

### Prerequisites

- Install `wasm-pack`: https://rustwasm.github.io/wasm-pack/
- Install [Chrome DevTools C++ DWARF debugging](https://chromewebstore.google.com/detail/cc++-devtools-support-dwa/pdcpmagijalfljmkmjngeonclgbbannb)

### Build and run demo with DWARF debug symbols

Open Chrome DevTools

- Go to "Sources" tab
- Open `file://<your repo path>/src/lib.rs`
