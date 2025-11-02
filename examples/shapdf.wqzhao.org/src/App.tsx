import {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
  type ChangeEvent,
  type KeyboardEvent,
} from "react";
import initWasm, { render_script } from "./wasm/shapdf_wasm_example.js";

type PdfJsModule = typeof import("pdfjs-dist");
type PDFDocumentProxy = import("pdfjs-dist").PDFDocumentProxy;

let pdfjsModulePromise: Promise<PdfJsModule> | null = null;

async function loadPdfJs(): Promise<PdfJsModule> {
  if (!pdfjsModulePromise) {
    pdfjsModulePromise = Promise.all([
      import("pdfjs-dist"),
      import("pdfjs-dist/build/pdf.worker?url"),
    ]).then(([pdfjs, worker]) => {
      if (pdfjs.GlobalWorkerOptions.workerSrc !== worker.default) {
        pdfjs.GlobalWorkerOptions.workerSrc = worker.default as unknown as string;
      }
      return pdfjs;
    });
  }
  return pdfjsModulePromise;
}

type SampleMeta = {
  id: string;
  label: string;
  file: string;
  description: string;
};

const SAMPLES: SampleMeta[] = [
  {
    id: "intro",
    label: "Intro: Simple Geometry",
    file: "sample_shapes.shapdf",
    description: "Starter composition featuring lines, circles, and rectangles.",
  },
  {
    id: "roadmap",
    label: "Product Roadmap Timeline",
    file: "product_roadmap.shapdf",
    description: "Landscape timeline with quarterly milestones and connectors.",
  },
  {
    id: "dashboard",
    label: "Analytics Dashboard Layout",
    file: "analytics_dashboard.shapdf",
    description: "Panel-based dashboard layout with trend lines and cards.",
  },
  {
    id: "notebook",
    label: "Designer Notebook Page",
    file: "notebook_sketchbook.shapdf",
    description: "Notebook-inspired canvas with grid guides and sticky notes.",
  },
];

type StatusTone = "idle" | "info" | "success" | "error";

const STATUS_TONE_CLASSES: Record<StatusTone, string> = {
  idle: "text-slate-400",
  info: "text-slate-300",
  success: "text-shapdf-200",
  error: "text-rose-300 font-medium",
};

const DEFAULT_SAMPLE_ID = SAMPLES[0]?.id ?? "";

export default function App() {
  const [selectedSampleId, setSelectedSampleId] = useState<string>("");
  const [script, setScript] = useState("");
  const [projectTitle, setProjectTitle] = useState("shapdf-output");
  const [isTitleDirty, setIsTitleDirty] = useState(false);
  const [autoRender, setAutoRender] = useState(true);
  const [fitToWidth, setFitToWidth] = useState(true);
  const [softWrap, setSoftWrap] = useState(false);
  const [pageCount, setPageCount] = useState(0);
  const [isRendering, setIsRendering] = useState(false);
  const [downloadUrl, setDownloadUrl] = useState<string | null>(null);
  const [status, setStatus] = useState<{ message: string; tone: StatusTone }>({
    message: "WebAssembly module not initialised yet.",
    tone: "idle",
  });

  const previewRef = useRef<HTMLDivElement>(null);
  const wasmInitRef = useRef<Promise<void> | null>(null);
  const lastPdfRef = useRef<Uint8Array | null>(null);
  const isRenderingRef = useRef(false);
  const queuedRenderRef = useRef(false);
  const autoRenderTimerRef = useRef<number | null>(null);
  const fileInputRef = useRef<HTMLInputElement | null>(null);

  const setStatusMessage = useCallback((message: string, tone: StatusTone = "idle") => {
    setStatus({ message, tone });
  }, []);

  const ensureWasm = useCallback(async () => {
    if (!wasmInitRef.current) {
      wasmInitRef.current = initWasm()
        .then(() => {
          setStatusMessage("WASM module ready. Render to preview.", "success");
        })
        .catch((error: unknown) => {
          const message = error instanceof Error ? error.message : String(error);
          setStatusMessage(`Failed to load WebAssembly: ${message}`, "error");
          wasmInitRef.current = null;
          throw error;
        });
    }

    await wasmInitRef.current;
  }, [setStatusMessage]);

  const revokeDownloadUrl = useCallback(() => {
    setDownloadUrl((current) => {
      if (current) {
        URL.revokeObjectURL(current);
      }
      return null;
    });
  }, []);

  useEffect(() => () => revokeDownloadUrl(), [revokeDownloadUrl]);

  const sanitizedTitle = useMemo(() => {
    const trimmed = projectTitle.trim();
    if (!trimmed) {
      return "shapdf-output";
    }
    return trimmed.replace(/[^\w.-]+/g, "_").replace(/^[_\s]+|[_\s]+$/g, "") || "shapdf-output";
  }, [projectTitle]);

const currentYear = useMemo(() => new Date().getFullYear(), []);

  const displayPdf = useCallback(
    async (bytes: Uint8Array) => {
      const container = previewRef.current;
      if (!container) {
        return;
      }

      container.innerHTML = "";

      try {
        const { getDocument } = await loadPdfJs();
        const pdfDoc: PDFDocumentProxy = await getDocument({ data: bytes.slice() }).promise;
        setPageCount(pdfDoc.numPages);

        const containerWidth = container.clientWidth || 800;

        for (let pageNumber = 1; pageNumber <= pdfDoc.numPages; pageNumber += 1) {
          const page = await pdfDoc.getPage(pageNumber);
          const baseViewport = page.getViewport({ scale: 1 });
          const targetWidth = fitToWidth
            ? Math.max(containerWidth - 48, 320)
            : baseViewport.width * 1.15;
          const scale = fitToWidth ? Math.min(targetWidth / baseViewport.width, 2.6) : 1.2;
          const viewport = page.getViewport({ scale });

          const canvas = document.createElement("canvas");
          canvas.width = viewport.width;
          canvas.height = viewport.height;
          canvas.className = "w-full border border-slate-800 bg-white";

          const ctx = canvas.getContext("2d");
          if (!ctx) {
            throw new Error("Unable to obtain canvas context.");
          }

          await page.render({ canvasContext: ctx, viewport, canvas }).promise;
          container.appendChild(canvas);
        }
      } catch (error: unknown) {
        console.error(error);
        const message = error instanceof Error ? error.message : String(error);
        setStatusMessage(`Unable to display PDF: ${message}`, "error");
        setPageCount(0);
      }
    },
    [fitToWidth, setStatusMessage],
  );

  const handleRender = useCallback(async () => {
    if (isRenderingRef.current) {
      queuedRenderRef.current = true;
      return;
    }

    isRenderingRef.current = true;
    setIsRendering(true);
    queuedRenderRef.current = false;

    try {
      await ensureWasm();
      setStatusMessage("Rendering via WebAssembly…", "info");

      const rendered = render_script(script);
      const bytes = rendered instanceof Uint8Array ? rendered : new Uint8Array(rendered);
      const downloadCopy = bytes.slice();
      lastPdfRef.current = downloadCopy;

      await displayPdf(downloadCopy);

      revokeDownloadUrl();
      const blobUrl = URL.createObjectURL(new Blob([downloadCopy.slice()], { type: "application/pdf" }));
      setDownloadUrl(blobUrl);

      const timestamp = new Date().toLocaleTimeString([], {
        hour: "2-digit",
        minute: "2-digit",
      });
      setStatusMessage(`Rendered successfully at ${timestamp}.`, "success");
    } catch (error: unknown) {
      console.error(error);
      const message = error instanceof Error ? error.message : String(error);
      revokeDownloadUrl();
      setStatusMessage(`Render failed: ${message}`, "error");
      setPageCount(0);
    } finally {
      setIsRendering(false);
      isRenderingRef.current = false;

      if (queuedRenderRef.current) {
        queuedRenderRef.current = false;
        window.setTimeout(() => {
          void handleRender();
        }, 0);
      }
    }
  }, [displayPdf, ensureWasm, revokeDownloadUrl, script, setStatusMessage]);

  const loadSample = useCallback(
    async (id: string) => {
      const sample = SAMPLES.find((item) => item.id === id);
      if (!sample) {
        setStatusMessage("Unknown sample selection.", "error");
        return;
      }

      try {
        setStatusMessage(`Loading “${sample.label}”…`, "info");
        revokeDownloadUrl();
        lastPdfRef.current = null;
        setPageCount(0);

        if (previewRef.current) {
          previewRef.current.innerHTML = "";
        }

        const response = await fetch(
          `${import.meta.env.BASE_URL}samples/${sample.file}`,
        );
        if (!response.ok) {
          throw new Error(`HTTP ${response.status}`);
        }

        const text = await response.text();
        setScript(text);
        if (!isTitleDirty) {
          setProjectTitle(sample.label);
        }
        setSelectedSampleId("");
        setStatusMessage(`Loaded template “${sample.label}”.`, "success");
      } catch (error: unknown) {
        console.error(error);
        const message = error instanceof Error ? error.message : String(error);
        setStatusMessage(`Could not load template: ${message}`, "error");
      }
    },
    [isTitleDirty, revokeDownloadUrl, setStatusMessage],
  );

  useEffect(() => {
    if (DEFAULT_SAMPLE_ID) {
      void loadSample(DEFAULT_SAMPLE_ID);
    }
  }, [loadSample]);

  useEffect(() => {
    if (!autoRender) {
      if (autoRenderTimerRef.current) {
        window.clearTimeout(autoRenderTimerRef.current);
        autoRenderTimerRef.current = null;
      }
      return;
    }

    if (!script.trim()) {
      return;
    }

    if (autoRenderTimerRef.current) {
      window.clearTimeout(autoRenderTimerRef.current);
    }

    autoRenderTimerRef.current = window.setTimeout(() => {
      void handleRender();
    }, 420);

    return () => {
      if (autoRenderTimerRef.current) {
        window.clearTimeout(autoRenderTimerRef.current);
        autoRenderTimerRef.current = null;
      }
    };
  }, [autoRender, handleRender, script]);

  useEffect(() => {
    if (fitToWidth && lastPdfRef.current) {
      void displayPdf(lastPdfRef.current);
    }
  }, [displayPdf, fitToWidth]);

  useEffect(() => {
    const handleResize = () => {
      if (fitToWidth && lastPdfRef.current) {
        void displayPdf(lastPdfRef.current);
      }
    };

    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, [displayPdf, fitToWidth]);

  const pageSummary = useMemo(() => {
    if (pageCount <= 0) {
      return "0 pages";
    }
    return pageCount === 1 ? "1 page" : `${pageCount} pages`;
  }, [pageCount]);

  const downloadDisabled = !downloadUrl || !lastPdfRef.current || isRendering;
  const sourceDownloadDisabled = script.trim().length === 0;

  const textareaClassNames = [
    "h-full w-full flex-1 rounded-xl border border-slate-800 bg-slate-950/80 p-5 font-mono text-sm leading-6 text-slate-100",
    "shadow-inner focus:border-shapdf-400 focus:outline-none focus:ring-2 focus:ring-shapdf-300",
    "overflow-auto resize-none",
    softWrap ? "whitespace-pre-wrap" : "whitespace-pre",
  ].join(" ");

  const handleScriptChange = (event: ChangeEvent<HTMLTextAreaElement>) => {
    setScript(event.target.value);
  };

  const handleProjectTitleChange = (event: ChangeEvent<HTMLInputElement>) => {
    setProjectTitle(event.target.value);
    setIsTitleDirty(true);
  };

  const handleScriptKeyDown = (event: KeyboardEvent<HTMLTextAreaElement>) => {
    if ((event.metaKey || event.ctrlKey) && event.key === "Enter") {
      event.preventDefault();
      void handleRender();
    }
  };

  const handleSampleChange = (event: ChangeEvent<HTMLSelectElement>) => {
    const id = event.target.value;
    setSelectedSampleId(id);
    if (id) {
      void loadSample(id);
    }
  };

  const handleUploadButton = () => {
    fileInputRef.current?.click();
  };

  const handleFileUpload = async (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) {
      return;
    }
    try {
      setStatusMessage(`Loading local file “${file.name}”…`, "info");
      revokeDownloadUrl();
      lastPdfRef.current = null;
      setPageCount(0);
      const text = await file.text();
      setScript(text);
      setSelectedSampleId("");
      const baseName = file.name.replace(/\.[^.]+$/, "");
      setProjectTitle(baseName || "shapdf-output");
      setIsTitleDirty(true);
      setStatusMessage(`Loaded local template “${file.name}”.`, "success");
    } catch (error: unknown) {
      console.error(error);
      const message = error instanceof Error ? error.message : String(error);
      setStatusMessage(`Failed to read file: ${message}`, "error");
    } finally {
      event.target.value = "";
    }
  };

  const handleSaveSource = () => {
    if (!script.trim()) {
      return;
    }
    const blob = new Blob([script], { type: "text/plain;charset=utf-8" });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = `${sanitizedTitle}.shapdf`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
  };

  return (
    <div className="flex h-screen flex-col overflow-hidden bg-slate-950 text-slate-100">
      <header className="border-b border-slate-900/70 bg-slate-950/80 backdrop-blur">
        <div className="grid w-full grid-cols-1 items-center gap-3 px-4 py-3 sm:grid-cols-[auto_minmax(0,1fr)_auto] sm:px-6 lg:px-8">
          <div className="flex items-center gap-2">
            <div className="rounded-md bg-shapdf-600/15 p-1.5 shadow-glow">
              <svg
                className="h-5 w-5 text-shapdf-400"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="1.6"
              >
                <path strokeLinecap="round" strokeLinejoin="round" d="M4 7h6m-6 5h16m-16 5h10" />
                <circle cx="17" cy="7" r="2" />
                <circle cx="12" cy="17" r="2" />
              </svg>
            </div>
            <span className="text-base font-semibold text-white">
              shapdf <span className="font-light text-slate-300">editor</span>
            </span>
          </div>
          <div className="flex w-full items-center justify-center">
            <input
              type="text"
              value={projectTitle}
              onChange={handleProjectTitleChange}
              placeholder="Project title"
              className="w-full max-w-xs rounded-lg border border-slate-700 bg-slate-900/70 px-3 py-1.5 text-center text-sm text-slate-100 shadow-inner focus:border-shapdf-400 focus:outline-none focus:ring-2 focus:ring-shapdf-300"
            />
          </div>
          <div className="flex justify-start sm:justify-end">
            <a
              href="https://github.com/Teddy-van-Jerry/shapdf"
              target="_blank"
              rel="noreferrer"
              className="inline-flex items-center gap-2 rounded-lg border border-slate-700 px-3 py-1.5 text-sm text-slate-200 transition hover:border-shapdf-400 hover:text-shapdf-200"
            >
              <svg
                className="h-4 w-4"
                viewBox="0 0 24 24"
                fill="currentColor"
                aria-hidden
              >
                <path
                  fillRule="evenodd"
                  d="M12.026 2c-5.509 0-9.974 4.468-9.974 9.98 0 4.41 2.865 8.15 6.839 9.471.5.09.682-.217.682-.482 0-.237-.009-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.153-1.11-1.46-1.11-1.46-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.087 2.91.832.091-.647.35-1.087.636-1.337-2.22-.253-4.555-1.112-4.555-4.946 0-1.092.39-1.987 1.029-2.688-.103-.254-.446-1.274.098-2.655 0 0 .84-.27 2.75 1.026a9.564 9.564 0 0 1 2.504-.337 9.56 9.56 0 0 1 2.503.337c1.91-1.296 2.748-1.026 2.748-1.026.546 1.381.202 2.401.1 2.655.64.701 1.028 1.596 1.028 2.688 0 3.842-2.339 4.69-4.566 4.938.359.31.678.922.678 1.857 0 1.34-.012 2.421-.012 2.749 0 .268.18.576.688.478C19.144 20.126 22 16.387 22 11.978 22 6.468 17.535 2 12.026 2Z"
                  clipRule="evenodd"
                />
              </svg>
              <span>GitHub</span>
            </a>
          </div>
        </div>
      </header>

      <input
        ref={fileInputRef}
        type="file"
        accept=".shapdf,.txt"
        className="hidden"
        onChange={handleFileUpload}
      />

      <main className="flex flex-1 flex-col overflow-hidden lg:flex-row">
        <section className="flex flex-1 flex-col overflow-hidden border-b border-slate-900/60 lg:border-b-0 lg:border-r">
          <div className="shrink-0 border-b border-slate-900/50 bg-slate-900/60">
            <div className="flex w-full flex-col gap-4 px-4 py-4 sm:px-6 lg:px-8">
              <div className="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
                <div className="flex flex-col gap-2 md:flex-1">
                  <label
                    htmlFor="sampleSelect"
                    className="text-xs uppercase tracking-wider text-slate-400"
                  >
                    Workspace
                  </label>
                  <div className="flex flex-col gap-2 sm:flex-row sm:items-center sm:gap-3">
                    <div className="flex flex-wrap items-center gap-2">
                      <button
                        type="button"
                        onClick={handleUploadButton}
                        className="inline-flex items-center justify-center gap-2 rounded-lg border border-slate-700 px-4 py-2 text-sm font-medium text-slate-200 transition hover:border-shapdf-400 hover:text-shapdf-200"
                      >
                        <svg
                          className="h-4 w-4"
                          viewBox="0 0 24 24"
                          fill="none"
                          stroke="currentColor"
                          strokeWidth="1.6"
                        >
                          <path strokeLinecap="round" strokeLinejoin="round" d="M5 12h14M12 5l7 7-7 7" />
                        </svg>
                        <span>Upload</span>
                      </button>
                      <button
                        type="button"
                        onClick={handleSaveSource}
                        disabled={sourceDownloadDisabled}
                        className={`inline-flex items-center justify-center gap-2 rounded-lg border px-4 py-2 text-sm font-medium transition ${
                          sourceDownloadDisabled
                            ? "pointer-events-none border-slate-800 text-slate-600 opacity-60"
                            : "border-slate-700 text-slate-200 hover:border-shapdf-400 hover:text-shapdf-200"
                        }`}
                      >
                        <svg
                          className="h-4 w-4"
                          viewBox="0 0 24 24"
                          fill="none"
                          stroke="currentColor"
                          strokeWidth="1.6"
                        >
                          <path strokeLinecap="round" strokeLinejoin="round" d="M12 5v14m7-7H5" />
                        </svg>
                        <span>Save source</span>
                      </button>
                    </div>
                    <select
                      id="sampleSelect"
                      className="w-full rounded-lg border border-slate-700 bg-slate-900/70 px-4 py-2 text-sm text-slate-200 focus:border-shapdf-400 focus:ring-shapdf-400"
                      value={selectedSampleId}
                      onChange={handleSampleChange}
                    >
                      <option value="">Browse templates…</option>
                      {SAMPLES.map((sample) => (
                        <option key={sample.id} value={sample.id}>
                          {sample.label}
                        </option>
                      ))}
                    </select>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div className="flex flex-1 flex-col overflow-hidden">
            <div className="flex h-full w-full flex-1 flex-col px-4 py-4 sm:px-6 lg:px-8">
              <label
                htmlFor="editor"
                className="text-xs font-medium uppercase tracking-widest text-slate-500"
              >
                .shapdf script
              </label>
              <div className="mt-3 flex-1 overflow-hidden">
                <textarea
                  id="editor"
                  value={script}
                  onChange={handleScriptChange}
                  onKeyDown={handleScriptKeyDown}
                  spellCheck={false}
                  wrap={softWrap ? "soft" : "off"}
                  className={textareaClassNames}
                />
              </div>
              <div className="mt-4 flex items-center justify-between text-xs text-slate-500">
                <label className="inline-flex cursor-pointer items-center gap-2">
                  <input
                    id="syncScale"
                    type="checkbox"
                    className="rounded border-slate-700 bg-slate-900/70 text-shapdf-400 focus:ring-shapdf-400"
                    checked={fitToWidth}
                    onChange={(event) => setFitToWidth(event.target.checked)}
                  />
                  <span>Fit preview to width</span>
                </label>
                <label className="inline-flex cursor-pointer items-center gap-2">
                  <input
                    id="wrapText"
                    type="checkbox"
                    className="rounded border-slate-700 bg-slate-900/70 text-shapdf-400 focus:ring-shapdf-400"
                    checked={softWrap}
                    onChange={(event) => setSoftWrap(event.target.checked)}
                  />
                  <span>Soft wrap</span>
                </label>
              </div>
            </div>
          </div>
        </section>

        <section className="flex flex-1 flex-col overflow-hidden">
          <div className="shrink-0 border-b border-slate-900/50 bg-slate-900/40">
            <div className="flex w-full flex-wrap items-center justify-between gap-4 px-4 py-4 sm:px-6 lg:px-8">
              <div className="flex flex-col gap-2">
                <div className="flex flex-wrap items-center gap-3">
                  <div>
                    <p className="text-xs uppercase tracking-widest text-slate-400">PDF preview</p>
                    <p className="text-sm text-slate-300">Rendered with pdf.js</p>
                  </div>
                  <div className="flex items-center gap-2 text-xs text-slate-400">
                    <span className="h-1 w-1 rounded-full bg-shapdf-400" />
                    <span>{pageSummary}</span>
                  </div>
                </div>
                <div className={`flex items-center gap-2 text-xs ${STATUS_TONE_CLASSES[status.tone]}`}>
                  <span className="h-1 w-1 rounded-full bg-shapdf-400" />
                  <span>{status.message}</span>
                </div>
              </div>
              <div className="flex flex-wrap items-center justify-end gap-2">
                <button
                  type="button"
                  onClick={() => void handleRender()}
                  disabled={isRendering}
                  className="inline-flex items-center justify-center gap-2 rounded-lg bg-shapdf-500 px-4 py-2 text-sm font-semibold text-white shadow-glow transition hover:bg-shapdf-400 focus:outline-none focus:ring-2 focus:ring-shapdf-300 focus:ring-offset-2 focus:ring-offset-slate-950 disabled:cursor-not-allowed disabled:opacity-60"
                >
                  <svg className="h-4 w-4 text-white" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M8 5v14l11-7z" />
                  </svg>
                  <span>{isRendering ? "Rendering…" : "Render (⌘⏎)"}</span>
                </button>
                <label className="inline-flex items-center gap-2 rounded-lg border border-slate-800 px-3 py-2 text-xs text-slate-300">
                  <input
                    id="autoRender"
                    type="checkbox"
                    className="rounded border-slate-700 bg-slate-900/70 text-shapdf-400 focus:ring-shapdf-400"
                    checked={autoRender}
                    onChange={(event) => setAutoRender(event.target.checked)}
                  />
                  <span>Auto render</span>
                </label>
                <a
                  className={`inline-flex items-center justify-center gap-2 rounded-lg border px-4 py-2 text-sm font-medium transition ${
                    downloadDisabled
                      ? "pointer-events-none border-slate-800 text-slate-600 opacity-60"
                      : "border-slate-700 text-slate-200 hover:border-shapdf-400 hover:text-shapdf-200"
                  }`}
                  href={downloadUrl ?? "#"}
                  download={`${sanitizedTitle}.pdf`}
                  aria-disabled={downloadDisabled}
                >
                  <svg
                    className="h-4 w-4"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    strokeWidth="1.6"
                  >
                    <path strokeLinecap="round" strokeLinejoin="round" d="M12 3v12m0 0 4-4m-4 4-4-4m9 6H7" />
                  </svg>
                  <span>Download PDF</span>
                </a>
              </div>
            </div>
          </div>
          <div className="flex flex-1 flex-col overflow-hidden">
            <div className="flex-1 overflow-auto bg-slate-950 px-4 py-6 sm:px-6 lg:px-8">
              <div className="flex w-full flex-col items-stretch gap-6">
                {pageCount === 0 ? (
                  <div className="flex h-64 w-full flex-col items-center justify-center border border-dashed border-slate-800 bg-slate-900/70 text-center text-sm text-slate-500">
                    <svg
                      className="mb-3 h-10 w-10 text-slate-600"
                      viewBox="0 0 24 24"
                      fill="none"
                      stroke="currentColor"
                      strokeWidth="1.4"
                    >
                      <path strokeLinecap="round" strokeLinejoin="round" d="M8 5h8m-6 4h6m-6 4h6m-6 4h4" />
                      <rect x="4" y="3" width="16" height="18" rx="2" />
                    </svg>
                    <p className="font-medium text-slate-400">No preview yet</p>
                    <p className="mt-1 max-w-sm text-xs text-slate-500">
                      Load a template, upload a .shapdf file, or start writing to render a PDF preview.
                    </p>
                  </div>
                ) : null}
                <div ref={previewRef} className="flex w-full flex-col gap-6" />
              </div>
            </div>
          </div>
        </section>
      </main>

      <footer className="border-t border-slate-900/70 bg-slate-950/80">
        <div className="flex w-full flex-col items-center justify-center gap-1 px-4 py-2 text-center text-xs text-slate-500 sm:flex-row sm:gap-2">
          <span>Licensed under GPL-3.0</span>
          <span>
            © {currentYear}{" "}
            <a
              href="https://wqzhao.org"
              target="_blank"
              rel="noreferrer"
              className="text-slate-300 transition hover:text-shapdf-200"
            >
              Wuqiong Zhao
            </a>
          </span>
        </div>
      </footer>
    </div>
  );
}
