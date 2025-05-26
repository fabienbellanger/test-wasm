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
