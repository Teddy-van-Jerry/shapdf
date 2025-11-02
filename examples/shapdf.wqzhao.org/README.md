# shapdf.wqzhao.org frontend

This React/Vite workspace hosts the in-browser shapdf editor.

## Regenerating the WebAssembly bundle

The contents of `src/wasm` are produced from the root Rust crate via `wasm-pack`.

From this directory run:

```sh
npm run build:wasm
```

This wraps `wasm-pack build ../../ --target web --out-dir src/wasm --out-name shapdf_wasm_example`, rebuilding the JavaScript glue code, type definitions, and `.wasm` binary in `src/wasm/`. Re-run this command whenever the Rust sources change.

## Development

```sh
npm install
npm run dev
```

## Production build

```sh
npm run build
```

The output is emitted to `dist/`, ready for GitHub Pages deployment.
