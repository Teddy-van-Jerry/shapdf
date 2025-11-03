# shapdf Web Editor

**Live Demo: [shapdf.wqzhao.org](https://shapdf.wqzhao.org)**

An in-browser PDF shape editor powered by Rust and WebAssembly. Create lines, circles, and rectangles using a simple declarative syntax.

## Features

- 🎨 **Monaco Editor** - VS Code-like editing experience with syntax highlighting
- 📄 **Live Preview** - Real-time PDF rendering with zoom controls (25%-300%)
- 🚀 **WebAssembly** - Fast, efficient rendering using Rust compiled to WASM
- 💾 **Export** - Download both `.shapdf` scripts and generated PDFs
- 📱 **Responsive** - Works on desktop and mobile devices

## Quick Start

```sh
npm install
npm run dev
```

Visit `http://localhost:5173` to start editing.

## Build Commands

- **`npm run dev`** - Start development server
- **`npm run build`** - Build for production (includes WASM rebuild)
- **`npm run build:wasm`** - Rebuild WebAssembly bundle only
- **`npm run preview`** - Preview production build

## WebAssembly

The WASM bundle in `src/wasm/` is auto-generated from the root Rust crate. Run `npm run build:wasm` after modifying Rust sources.

## Deployment

Built artifacts in `dist/` are ready for static hosting (GitHub Pages, Netlify, Vercel, etc.).

---

**Author:** [Wuqiong Zhao](https://wqzhao.org)
**License:** GPL-3.0
