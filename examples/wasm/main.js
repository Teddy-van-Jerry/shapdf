import init, { render_script } from "./pkg/shapdf_wasm_example.js";

const statusEl = document.getElementById("status");
const scriptEl = document.getElementById("script");
const renderBtn = document.getElementById("render");

async function ensureWasm() {
  if (!ensureWasm._initP) {
    statusEl.textContent = "Loading WebAssembly module...";
    ensureWasm._initP = init().catch((err) => {
      statusEl.textContent = "Failed to load wasm: " + err;
      throw err;
    });
    await ensureWasm._initP;
    statusEl.textContent = "Wasm module ready.";
  }
  return ensureWasm._initP;
}

renderBtn.addEventListener("click", async () => {
  try {
    await ensureWasm();
    statusEl.textContent = "Rendering...";
    const script = scriptEl.value;
    const pdfBytes = render_script(script);
    const blob = new Blob([pdfBytes], { type: "application/pdf" });
    const url = URL.createObjectURL(blob);

    const link = document.createElement("a");
    link.href = url;
    link.download = "shapdf-output.pdf";
    link.textContent = "Download generated PDF";

    statusEl.innerHTML = "<strong>Success:</strong> ";
    statusEl.appendChild(link);
  } catch (err) {
    console.error(err);
    statusEl.textContent = "Error: " + err;
  }
});
