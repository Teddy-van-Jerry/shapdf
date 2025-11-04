import {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
  memo,
  type ChangeEvent,
} from "react";
import initWasm, { render_script } from "./wasm/shapdf_wasm_example.js";
import MonacoEditor from "@monaco-editor/react";
import { registerShapdfLanguage } from "./shapdfLanguage";

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

const DEFAULT_SAMPLE_ID = SAMPLES[0]?.id ?? "";

const PdfToolbar = memo(({
  pageSummary,
  pdfScale,
  onZoomOut,
  onZoomIn,
  onZoomReset,
  onRender,
  isRendering,
  downloadUrl,
  sanitizedTitle
}: {
  pageSummary: string;
  pdfScale: number;
  onZoomOut: () => void;
  onZoomIn: () => void;
  onZoomReset: () => void;
  onRender: () => void;
  isRendering: boolean;
  downloadUrl: string | null;
  sanitizedTitle: string;
}) => {
  const downloadDisabled = !downloadUrl || isRendering;

  return (
    <div className="shrink-0 border-b border-slate-800 bg-slate-900/40 px-4 py-2.5 sm:px-6" style={{ willChange: 'auto', transform: 'translateZ(0)' }}>
      <div className="flex items-center justify-between gap-3 min-h-[32px]">
        <div className="flex items-center gap-2">
          <p className="text-xs font-medium uppercase tracking-wider text-slate-400">PDF preview</p>
          <span className="text-xs text-slate-500">•</span>
          <span className="text-xs text-slate-400">{pageSummary}</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="flex items-center gap-1 rounded-md border border-slate-700 bg-slate-900/50">
            <button
              type="button"
              onClick={onZoomOut}
              className="px-2 py-1 text-slate-300 hover:text-shapdf-200 transition"
              title="Zoom out"
            >
              <svg className="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <circle cx="11" cy="11" r="8" />
                <path strokeLinecap="round" d="M8 11h6M21 21l-4.35-4.35" />
              </svg>
            </button>
            <span className="text-xs text-slate-400 px-1 border-x border-slate-700">{Math.round(pdfScale * 100)}%</span>
            <button
              type="button"
              onClick={onZoomIn}
              className="px-2 py-1 text-slate-300 hover:text-shapdf-200 transition"
              title="Zoom in"
            >
              <svg className="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <circle cx="11" cy="11" r="8" />
                <path strokeLinecap="round" d="M11 8v6M8 11h6M21 21l-4.35-4.35" />
              </svg>
            </button>
            <button
              type="button"
              onClick={onZoomReset}
              className="px-2 py-1 text-xs text-slate-300 hover:text-shapdf-200 transition border-l border-slate-700"
              title="Reset zoom"
            >
              1:1
            </button>
          </div>
          <button
            type="button"
            onClick={onRender}
            disabled={isRendering}
            className="inline-flex items-center gap-1.5 rounded-md bg-shapdf-500 px-3 py-1.5 text-xs font-semibold text-white shadow-glow transition hover:bg-shapdf-400 focus:outline-none focus:ring-2 focus:ring-shapdf-300 focus:ring-offset-2 focus:ring-offset-slate-950 disabled:cursor-not-allowed disabled:opacity-60"
          >
            <svg className="h-3.5 w-3.5 text-white" viewBox="0 0 24 24" fill="currentColor">
              <path d="M8 5v14l11-7z" />
            </svg>
            <span>{isRendering ? "Rendering…" : "Render"}</span>
          </button>
          <a
            className={`inline-flex items-center gap-1.5 rounded-md border px-2.5 py-1.5 text-xs font-medium transition ${
              downloadDisabled
                ? "pointer-events-none border-slate-800 text-slate-600 opacity-60"
                : "border-slate-700 text-slate-200 hover:border-shapdf-400 hover:text-shapdf-200"
            }`}
            href={downloadUrl ?? "#"}
            download={`${sanitizedTitle}.pdf`}
            aria-disabled={downloadDisabled}
          >
            <svg className="h-3.5 w-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path strokeLinecap="round" strokeLinejoin="round" d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4m4-5l5 5m0 0l5-5m-5 5V3" />
            </svg>
            <span>Download</span>
          </a>
        </div>
      </div>
    </div>
  );
});

PdfToolbar.displayName = 'PdfToolbar';

// Custom comparison to prevent unnecessary re-renders
const PdfToolbarMemoized = memo(PdfToolbar, (prevProps, nextProps) => {
  return (
    prevProps.pageSummary === nextProps.pageSummary &&
    prevProps.pdfScale === nextProps.pdfScale &&
    prevProps.onZoomOut === nextProps.onZoomOut &&
    prevProps.onZoomIn === nextProps.onZoomIn &&
    prevProps.onZoomReset === nextProps.onZoomReset &&
    prevProps.onRender === nextProps.onRender &&
    prevProps.isRendering === nextProps.isRendering &&
    prevProps.downloadUrl === nextProps.downloadUrl &&
    prevProps.sanitizedTitle === nextProps.sanitizedTitle
  );
});

export default function App() {
  const [selectedSampleId, setSelectedSampleId] = useState<string>("");
  const [script, setScript] = useState("");
  const [projectTitle, setProjectTitle] = useState("shapdf-output");
  const [isTitleDirty, setIsTitleDirty] = useState(false);
  const [autoRender, setAutoRender] = useState(true);
  const [pdfScale, setPdfScale] = useState(1.0);
  const [pageCount, setPageCount] = useState(0);
  const [isRendering, setIsRendering] = useState(false);
  const [downloadUrl, setDownloadUrl] = useState<string | null>(null);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const previewRef = useRef<HTMLDivElement>(null);
  const wasmInitRef = useRef<Promise<void> | null>(null);
  const lastPdfRef = useRef<Uint8Array | null>(null);
  const isRenderingRef = useRef(false);
  const queuedRenderRef = useRef(false);
  const autoRenderTimerRef = useRef<number | null>(null);
  const fileInputRef = useRef<HTMLInputElement | null>(null);
  const pageCountRef = useRef<number>(0);
  const pdfScaleRef = useRef<number>(1.0);

  const ensureWasm = useCallback(async () => {
    if (!wasmInitRef.current) {
      wasmInitRef.current = initWasm()
        .then(() => {
          // WASM module ready
        })
        .catch((error: unknown) => {
          const message = error instanceof Error ? error.message : String(error);
          console.error(`Failed to load WebAssembly: ${message}`);
          wasmInitRef.current = null;
          throw error;
        });
    }

    await wasmInitRef.current;
  }, []);

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

      // Defer rendering to next frame to allow UI to update first
      await new Promise(resolve => requestAnimationFrame(() => resolve(undefined)));

      try {
        const { getDocument } = await loadPdfJs();
        const pdfDoc: PDFDocumentProxy = await getDocument({ data: bytes.slice() }).promise;

        // Always update page count to ensure UI consistency
        pageCountRef.current = pdfDoc.numPages;
        setPageCount(pdfDoc.numPages);

        // Build all canvases, rendering each page in separate animation frames
        const canvases: HTMLCanvasElement[] = [];
        const outputScale = window.devicePixelRatio || 1;
        const currentScale = pdfScaleRef.current;

        for (let pageNumber = 1; pageNumber <= pdfDoc.numPages; pageNumber += 1) {
          // Yield to browser between pages to keep UI responsive
          await new Promise(resolve => requestAnimationFrame(() => resolve(undefined)));

          const page = await pdfDoc.getPage(pageNumber);
          const viewport = page.getViewport({ scale: currentScale });

          const canvas = document.createElement("canvas");
          canvas.width = Math.floor(viewport.width * outputScale);
          canvas.height = Math.floor(viewport.height * outputScale);
          canvas.style.width = Math.floor(viewport.width) + "px";
          canvas.style.height = Math.floor(viewport.height) + "px";
          canvas.className = "border border-slate-800 bg-white shadow-lg";

          const ctx = canvas.getContext("2d");
          if (!ctx) {
            throw new Error("Unable to obtain canvas context.");
          }

          const renderContext = {
            canvasContext: ctx,
            viewport: viewport,
            canvas: canvas,
            ...(outputScale !== 1 && {
              transform: [outputScale, 0, 0, outputScale, 0, 0] as [number, number, number, number, number, number],
            }),
          };

          await page.render(renderContext).promise;
          canvases.push(canvas);
        }

        // Replace all canvases at once
        const fragment = document.createDocumentFragment();
        canvases.forEach(canvas => fragment.appendChild(canvas));

        container.innerHTML = "";
        container.appendChild(fragment);
        setErrorMessage(null);
      } catch (error: unknown) {
        console.error(error);
        const message = error instanceof Error ? error.message : String(error);
        setErrorMessage(`Unable to display PDF: ${message}`);
        if (pageCountRef.current !== 0) {
          pageCountRef.current = 0;
          setPageCount(0);
        }
      }
    },
    [],
  );

  // Sync pdfScale state to ref
  useEffect(() => {
    pdfScaleRef.current = pdfScale;
  }, [pdfScale]);

  // Clear preview container when pageCount becomes 0
  useEffect(() => {
    if (pageCount === 0 && previewRef.current) {
      previewRef.current.innerHTML = "";
    }
  }, [pageCount]);

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

      const rendered = render_script(script);
      const bytes = rendered instanceof Uint8Array ? rendered : new Uint8Array(rendered);
      const downloadCopy = bytes.slice();
      lastPdfRef.current = downloadCopy;

      await displayPdf(downloadCopy);

      revokeDownloadUrl();
      const blobUrl = URL.createObjectURL(new Blob([downloadCopy.slice()], { type: "application/pdf" }));
      setDownloadUrl(blobUrl);
      setErrorMessage(null);
    } catch (error: unknown) {
      console.error(error);
      const message = error instanceof Error ? error.message : String(error);
      setErrorMessage(`Render failed: ${message}`);
      revokeDownloadUrl();
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
  }, [displayPdf, ensureWasm, revokeDownloadUrl, script]);

  const loadSample = useCallback(
    async (id: string) => {
      const sample = SAMPLES.find((item) => item.id === id);
      if (!sample) {
        console.error("Unknown sample selection.");
        return;
      }

      try {
        revokeDownloadUrl();
        lastPdfRef.current = null;
        pageCountRef.current = 0;
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
      } catch (error: unknown) {
        console.error(error);
        const message = error instanceof Error ? error.message : String(error);
        console.error(`Could not load template: ${message}`);
      }
    },
    [isTitleDirty, revokeDownloadUrl],
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

  // Trigger PDF re-render when scale changes
  useEffect(() => {
    if (lastPdfRef.current) {
      void displayPdf(lastPdfRef.current);
    }
  }, [pdfScale, displayPdf]);

  const pageSummary = useMemo(() => {
    if (pageCount <= 0) {
      return "0 pages";
    }
    return pageCount === 1 ? "1 page" : `${pageCount} pages`;
  }, [pageCount]);

  const sourceDownloadDisabled = script.trim().length === 0;

  const handleScriptChange = (value: string) => {
    setScript(value);
  };

  const handleProjectTitleChange = (event: ChangeEvent<HTMLInputElement>) => {
    setProjectTitle(event.target.value);
    setIsTitleDirty(true);
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
      revokeDownloadUrl();
      lastPdfRef.current = null;
      pageCountRef.current = 0;
      setPageCount(0);
      const text = await file.text();
      setScript(text);
      setSelectedSampleId("");
      const baseName = file.name.replace(/\.[^.]+$/, "");
      setProjectTitle(baseName || "shapdf-output");
      setIsTitleDirty(true);
    } catch (error: unknown) {
      console.error(error);
      const message = error instanceof Error ? error.message : String(error);
      console.error(`Failed to read file: ${message}`);
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

  const handleZoomIn = useCallback(() => {
    requestAnimationFrame(() => {
      setPdfScale((prev) => Math.min(prev + 0.25, 3.0));
    });
  }, []);

  const handleZoomOut = useCallback(() => {
    requestAnimationFrame(() => {
      setPdfScale((prev) => Math.max(prev - 0.25, 0.25));
    });
  }, []);

  const handleZoomReset = useCallback(() => {
    requestAnimationFrame(() => {
      setPdfScale(1.0);
    });
  }, []);

  const handleRenderClick = useCallback(() => {
    void handleRender();
  }, [handleRender]);

  return (
    <div className="flex h-screen flex-col overflow-hidden bg-slate-950 text-slate-100">
      <header className="border-b border-slate-800 bg-slate-900/80 backdrop-blur">
        <div className="flex items-center justify-between px-4 py-2.5 sm:px-6">
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
          <input
            type="text"
            value={projectTitle}
            onChange={handleProjectTitleChange}
            placeholder="Project title"
            className="w-full max-w-xs rounded-lg border border-slate-700 bg-slate-900/70 px-3 py-1.5 text-center text-sm text-slate-100 shadow-inner focus:border-shapdf-400 focus:outline-none focus:ring-1 focus:ring-shapdf-300"
          />
          <div className="flex items-center gap-2">
            <a
              href="https://docs.rs/shapdf"
              target="_blank"
              rel="noreferrer"
              className="inline-flex items-center gap-1.5 rounded-lg border border-slate-700 px-2.5 py-1.5 text-sm text-slate-200 transition hover:border-shapdf-400 hover:text-shapdf-200"
              title="Rust Documentation"
            >
              <svg className="h-4 w-4" viewBox="20 20 104 104" fill="currentColor" aria-hidden>
                <path d="m71.05 23.68c-26.06 0-47.27 21.22-47.27 47.27s21.22 47.27 47.27 47.27 47.27-21.22 47.27-47.27-21.22-47.27-47.27-47.27zm-.07 4.2a3.1 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11zm7.12 5.12a38.27 38.27 0 0 1 26.2 18.66l-3.67 8.28c-.63 1.43.02 3.11 1.44 3.75l7.06 3.13a38.27 38.27 0 0 1 .08 6.64h-3.93c-.39 0-.55.26-.55.64v1.8c0 4.24-2.39 5.17-4.49 5.4-2 .23-4.21-.84-4.49-2.06-1.18-6.63-3.14-8.04-6.24-10.49 3.85-2.44 7.85-6.05 7.85-10.87 0-5.21-3.57-8.49-6-10.1-3.42-2.25-7.2-2.7-8.22-2.7h-40.6a38.27 38.27 0 0 1 21.41-12.08l4.79 5.02c1.08 1.13 2.87 1.18 4 .09zm-44.2 23.02a3.1 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11zm74.15.14a3.11 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11zm-68.29.5h5.42v24.44h-10.94a38.27 38.27 0 0 1 -1.24-14.61l6.7-2.98c1.43-.64 2.08-2.31 1.44-3.74zm22.62.26h12.91c.67 0 4.71.77 4.71 3.8 0 2.51-3.1 3.41-5.65 3.41h-11.98zm0 17.56h9.89c.9 0 4.83.26 6.08 5.28.39 1.54 1.26 6.56 1.85 8.17.59 1.8 2.98 5.4 5.53 5.4h16.14a38.27 38.27 0 0 1 -3.54 4.1l-6.57-1.41c-1.53-.33-3.04.65-3.37 2.18l-1.56 7.28a38.27 38.27 0 0 1 -31.91-.15l-1.56-7.28c-.33-1.53-1.83-2.51-3.36-2.18l-6.43 1.38a38.27 38.27 0 0 1 -3.32-3.92h31.27c.35 0 .59-.06.59-.39v-11.06c0-.32-.24-.39-.59-.39h-9.15zm-14.43 25.33a3.11 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11zm46.05.14a3.11 3.11 0 0 1 3.02 3.11 3.11 3.11 0 0 1 -6.22 0 3.11 3.11 0 0 1 3.2-3.11z" />
              </svg>
              <span>Docs</span>
            </a>
            <a
              href="https://github.com/Teddy-van-Jerry/shapdf"
              target="_blank"
              rel="noreferrer"
              className="inline-flex items-center gap-1.5 rounded-lg border border-slate-700 px-2.5 py-1.5 text-sm text-slate-200 transition hover:border-shapdf-400 hover:text-shapdf-200"
            >
              <svg className="h-4 w-4" viewBox="0 0 24 24" fill="currentColor" aria-hidden>
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
        <section className="flex flex-1 flex-col overflow-hidden border-b border-slate-900/60 lg:border-b-0 lg:border-r lg:border-slate-800">
          <div className="shrink-0 border-b border-slate-800 bg-slate-900/40 px-4 py-2.5 sm:px-6" style={{ willChange: 'auto', transform: 'translateZ(0)' }}>
            <div className="flex items-center justify-between gap-3 min-h-[32px]">
              <div className="flex items-center gap-2">
                <button
                  type="button"
                  onClick={handleUploadButton}
                  className="inline-flex items-center gap-1.5 rounded-md border border-slate-700 px-2.5 py-1.5 text-xs font-medium text-slate-200 transition hover:border-shapdf-400 hover:text-shapdf-200"
                >
                  <svg className="h-3.5 w-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
                  </svg>
                  <span>Upload</span>
                </button>
                <button
                  type="button"
                  onClick={handleSaveSource}
                  disabled={sourceDownloadDisabled}
                  className={`inline-flex items-center gap-1.5 rounded-md border px-2.5 py-1.5 text-xs font-medium transition ${
                    sourceDownloadDisabled
                      ? "pointer-events-none border-slate-800 text-slate-600 opacity-60"
                      : "border-slate-700 text-slate-200 hover:border-shapdf-400 hover:text-shapdf-200"
                  }`}
                >
                  <svg className="h-3.5 w-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4" />
                  </svg>
                  <span>Save</span>
                </button>
              </div>
              <select
                className="flex-1 max-w-xs rounded-md border border-slate-700 bg-slate-900/70 px-2.5 py-1.5 text-xs text-slate-200 focus:border-shapdf-400 focus:ring-1 focus:ring-shapdf-400"
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

          <div className="flex flex-1 flex-col overflow-hidden px-4 py-3 sm:px-6">
            <div className="mb-2 flex items-center justify-between">
              <label className="text-xs font-medium uppercase tracking-wider text-slate-400">
                .shapdf script
              </label>
              <label className="inline-flex cursor-pointer items-center gap-1.5 text-xs text-slate-400">
                <input
                  id="autoRender"
                  type="checkbox"
                  className="rounded border-slate-700 bg-slate-900/70 text-shapdf-400 focus:ring-shapdf-400 focus:ring-offset-0"
                  checked={autoRender}
                  onChange={(event) => setAutoRender(event.target.checked)}
                />
                <span>Auto render</span>
              </label>
            </div>
            <div className="flex-1 overflow-hidden rounded-xl border border-slate-800 shadow-inner">
              <MonacoEditor
                height="100%"
                language="shapdf"
                theme="shapdf-dark"
                value={script}
                onChange={(value) => handleScriptChange(value || '')}
                options={{
                  minimap: { enabled: false },
                  fontSize: 13,
                  lineHeight: 20,
                  fontFamily: 'ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, "Liberation Mono", monospace',
                  scrollBeyondLastLine: false,
                  wordWrap: 'off',
                  automaticLayout: true,
                  padding: { top: 8, bottom: 8 },
                  lineNumbers: 'on',
                  glyphMargin: false,
                  folding: false,
                  lineDecorationsWidth: 20,
                  lineNumbersMinChars: 3,
                  renderLineHighlight: 'line',
                  scrollbar: {
                    vertical: 'visible',
                    horizontal: 'auto',
                    useShadows: false,
                    verticalScrollbarSize: 10,
                    horizontalScrollbarSize: 10,
                  },
                }}
                beforeMount={(monaco) => {
                  // Register shapdf language with syntax highlighting
                  registerShapdfLanguage(monaco);

                  // Override theme colors to match app style - desaturated color scheme
                  monaco.editor.defineTheme('shapdf-dark', {
                    base: 'vs-dark',
                    inherit: true,
                    rules: [
                      { token: 'comment.line.number-sign.shapdf', foreground: '6b7280', fontStyle: 'italic' },
                      { token: 'keyword.control.shapdf', foreground: '8b7db8', fontStyle: 'bold' },
                      { token: 'entity.name.function.shapdf', foreground: '6b9dd4', fontStyle: 'bold' },
                      { token: 'variable.parameter.shapdf', foreground: 'd4a574' },
                      { token: 'constant.language.shapdf', foreground: '9d8bc7' },
                      { token: 'constant.language.cap.shapdf', foreground: '9d8bc7' },
                      { token: 'constant.language.anchor.shapdf', foreground: '9d8bc7' },
                      { token: 'constant.numeric.shapdf', foreground: 'd4956b' },
                      { token: 'constant.numeric.measurement.shapdf', foreground: 'd4956b' },
                      { token: 'constant.other.color.hex.shapdf', foreground: '6dac8f' },
                      { token: 'constant.other.color.named.shapdf', foreground: '6dac8f' },
                      { token: 'support.function.color.shapdf', foreground: '5a9478' },
                      { token: 'keyword.other.unit.shapdf', foreground: 'd4956b' },
                    ],
                    colors: {
                      'editor.background': '#020617',
                      'editor.lineHighlightBackground': '#1e293b',
                      'editorLineNumber.foreground': '#64748b',
                      'editorLineNumber.activeForeground': '#94a3b8',
                      'editor.selectionBackground': '#3b82f640',
                      'editor.inactiveSelectionBackground': '#3b82f620',
                    },
                  });
                }}
                onMount={(_editor, monaco) => {
                  monaco.editor.setTheme('shapdf-dark');
                }}
              />
            </div>
          </div>
        </section>

        <section className="flex flex-1 flex-col overflow-hidden">
          <PdfToolbarMemoized
            pageSummary={pageSummary}
            pdfScale={pdfScale}
            onZoomOut={handleZoomOut}
            onZoomIn={handleZoomIn}
            onZoomReset={handleZoomReset}
            onRender={handleRenderClick}
            isRendering={isRendering}
            downloadUrl={downloadUrl}
            sanitizedTitle={sanitizedTitle}
          />
          <div className="flex-1 overflow-auto bg-slate-950 px-4 py-4 sm:px-6">
            <div className="flex w-full flex-col items-center gap-4">
              {errorMessage ? (
                <div className="flex w-full flex-col items-center justify-center border border-red-900/50 rounded-lg bg-red-950/30 px-6 py-4 text-center">
                  <svg
                    className="mb-2 h-8 w-8 text-red-400"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    strokeWidth="2"
                  >
                    <circle cx="12" cy="12" r="10" />
                    <path strokeLinecap="round" d="M12 8v4m0 4h.01" />
                  </svg>
                  <p className="font-medium text-red-300 text-sm">Error</p>
                  <p className="mt-1 max-w-lg text-xs text-red-400/90 break-words">
                    {errorMessage}
                  </p>
                </div>
              ) : pageCount === 0 ? (
                <div className="flex h-64 w-full flex-col items-center justify-center border border-dashed border-slate-800 rounded-lg bg-slate-900/50 text-center text-sm text-slate-500">
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
              <div ref={previewRef} className="flex w-full flex-col items-center gap-4" />
            </div>
          </div>
        </section>
      </main>

      <footer className="border-t border-slate-800 bg-slate-950/80">
        <div className="flex w-full items-center justify-center gap-2 px-4 py-1.5 text-center text-xs text-slate-500">
          <span>Licensed under GPL-3.0</span>
          <span>•</span>
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
