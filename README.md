# cfw-wasi

WASI sample

## Build

```
cargo build --target wasm32-wasi --release
```

## Run

### Use wasmtime

```
echo '{"left":{"x":100, "y":100}, "ope":"Mul" , "right":{"x":150, "y":2}}' | wasmtime target/wasm32-wasi/release/cfw-wasi.wasm
```

### Use Cloudflare Workers

! This sample size is over 1MiB !

Run worker

```
npx wrangler@wasm dev target/wasm32-wasi/release/cfw-wasi.wasm
```

Call worker

```
echo '{"left":{"x":100, "y":100}, "ope":"Plus" , "right":{"x":150, "y":70}}' | curl -X POST --data-binary @- http://localhost:8787
```
