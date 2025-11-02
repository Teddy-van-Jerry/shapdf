# shapdf WebAssembly Example

Minimal demo that exposes the `render_script` helper to the browser.

## Prerequisites
- `wasm-pack`
- `npm`

## Build & Serve
```bash
cd examples/wasm
npm install
npm run serve
```
This builds the wasm bundle into `dist/` (including `pkg/`, `index.html`, and `main.js`) and serves it on http://localhost:8080.

## Manual Build
```bash
wasm-pack build --target web
```
Then open `index.html` while serving the directory (e.g. `npx http-server .`).
