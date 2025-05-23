# Test WASM with Rust

## Command

Build for Web:
```bash
wasm-pack --target web
```

## Run a client

```bash
deno run --allow-net --allow-read https://deno.land/std@0.224.0/http/file_server.ts .
```

Open in a browser: [http://localhost:4507/index.html](http://localhost:4507/index.html)
