import { useCallback, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/plugin-dialog";
import { TitleBar } from "./components/TitleBar";
import { EmptyState } from "./components/EmptyState";
import { ResultList, type SearchResult } from "./components/ResultList";
import { StatusBar } from "./components/StatusBar";

interface IndexStats {
  folders: number;
  files: number;
  chunks: number;
  bytes_indexed: number;
}

interface SearchHit {
  chunk_id: number;
  file_id: number;
  path: string;
  filename: string;
  snippet: string;
  score: number;
  kind: "semantic" | "keyword" | "hybrid";
  file_type: string;
}

interface SearchResponse {
  hits: SearchHit[];
  total_candidates: number;
  latency_ms: number;
  degraded: boolean;
}

function toUiResult(h: SearchHit): SearchResult {
  return {
    id: String(h.chunk_id),
    filename: h.filename,
    path: h.path,
    snippet: h.snippet,
    score: h.score,
    kind: h.kind === "semantic" ? "semantic" : "keyword",
    fileType:
      h.file_type === "code"
        ? "code"
        : h.file_type === "pdf"
        ? "pdf"
        : h.file_type === "doc"
        ? "doc"
        : "other",
  };
}

interface SearchStatus {
  ollama_reachable: boolean;
  ollama_url: string;
  model: string;
  dim: number;
  circuit_open: boolean;
}

export default function App() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [latencyMs, setLatencyMs] = useState(0);
  const [degraded, setDegraded] = useState(false);

  // Phase 2: folder + stats state.
  const [folders, setFolders] = useState<string[]>([]);
  const [stats, setStats] = useState<IndexStats | null>(null);
  const [indexing, setIndexing] = useState(false);

  // Phase 3: search status.
  const [searchStatus, setSearchStatus] = useState<SearchStatus | null>(null);

  const inputRef = useRef<HTMLInputElement | null>(null);

  const refreshStats = useCallback(async () => {
    try {
      const s = await invoke<IndexStats>("get_stats");
      setStats(s);
    } catch (e) {
      console.error("get_stats failed:", e);
    }
  }, []);

  const loadFolders = useCallback(async () => {
    try {
      const list = await invoke<string[]>("list_folders");
      setFolders(list);
    } catch (e) {
      console.error("list_folders failed:", e);
    }
  }, []);

  // Show + focus the title bar on mount; load initial state.
  useEffect(() => {
    const win = getCurrentWindow();
    win.show().catch(console.error);
    win.setFocus().catch(console.error);
    inputRef.current?.focus();
    loadFolders();
    refreshStats();
    invoke<SearchStatus>("get_search_status")
      .then(setSearchStatus)
      .catch((e) => console.error("get_search_status failed:", e));
  }, [loadFolders, refreshStats]);

  // Re-focus after the title bar is re-shown via global hotkey
  useEffect(() => {
    const handler = () => {
      setTimeout(() => inputRef.current?.focus(), 50);
    };
    window.addEventListener("telme:focus", handler);
    return () => window.removeEventListener("telme:focus", handler);
  }, []);

  const handleClose = async () => {
    if (query.length > 0) {
      setQuery("");
      setResults([]);
      setSelectedIndex(0);
      inputRef.current?.focus();
      return;
    }
    try {
      await invoke("close_titlebar");
    } catch (e) {
      console.error("close_titlebar failed:", e);
    }
  };

  const handleClear = () => {
    setQuery("");
    setResults([]);
    setSelectedIndex(0);
    inputRef.current?.focus();
  };

  const handleAddFolder = async () => {
    try {
      const picked = await open({
        directory: true,
        multiple: false,
        title: "Add folder to index",
      });
      if (!picked || typeof picked !== "string") return;
      try {
        await invoke("add_folder", { path: picked });
      } catch (e) {
        // Backend rejects duplicates + invalid paths with a string error.
        console.error("add_folder failed:", e);
        alert(`Couldn't add folder:\n${e}`);
        return;
      }
      await loadFolders();
      // Trigger a reindex; this is sync in Phase 2 (background thread lands in Phase 5).
      setIndexing(true);
      try {
        const summary = await invoke<{ scanned: number; indexed: number }>("reindex");
        console.log("reindex:", summary);
      } catch (e) {
        console.error("reindex failed:", e);
      } finally {
        setIndexing(false);
      }
      await refreshStats();
    } finally {
      inputRef.current?.focus();
    }
  };

  // Phase 3: query -> debounced backend search via Tauri command.
  useEffect(() => {
    const q = query.trim();
    if (q.length === 0) {
      setResults([]);
      setLatencyMs(0);
      setDegraded(false);
      return;
    }

    let cancelled = false;
    const handle = setTimeout(async () => {
      try {
        const resp = await invoke<SearchResponse>("search", { query: q });
        if (cancelled) return;
        setResults(resp.hits.map(toUiResult));
        setLatencyMs(resp.latency_ms);
        setDegraded(resp.degraded);
        setSelectedIndex(0);
      } catch (e) {
        if (cancelled) return;
        console.error("search failed:", e);
        setResults([]);
      }
    }, 80);

    return () => {
      cancelled = true;
      clearTimeout(handle);
    };
  }, [query]);

  // Capture-phase listener on the input so Esc/Arrows work even though the
  // user is typing. Avoids re-rendering the input element on every state change.
  useEffect(() => {
    const input = inputRef.current;
    if (!input) return;
    const handler = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        e.preventDefault();
        handleClose();
        return;
      }
      if (e.key === "ArrowDown") {
        e.preventDefault();
        if (results.length > 0) {
          setSelectedIndex((i) => (i + 1) % results.length);
        }
        return;
      }
      if (e.key === "ArrowUp") {
        e.preventDefault();
        if (results.length > 0) {
          setSelectedIndex((i) => (i - 1 + results.length) % results.length);
        }
        return;
      }
      if (e.key === "Enter" && results.length > 0) {
        e.preventDefault();
        const r = results[selectedIndex];
        console.log("Phase 2 placeholder: open file", r.path);
      }
    };
    input.addEventListener("keydown", handler);
    return () => input.removeEventListener("keydown", handler);
  }, [results, selectedIndex, query]);

  // ───────────────────────────── render ─────────────────────────────

  const emptyTitle =
    folders.length === 0 ? "No folders indexed yet." : "Type to search…";
  const emptyHint =
    folders.length === 0
      ? "Pick a folder to start indexing its text and code files."
      : "Semantic search lands in Phase 4. For now you can manage folders.";
  const emptyAction =
    folders.length === 0 ? "Add folder" : "Add another folder";

  return (
    <div
      className="h-screen w-screen flex items-start justify-center pt-[80px] bg-transparent"
      data-tauri-drag-region
    >
      <div className="w-[700px] max-h-[600px] flex flex-col bg-bg-elevated border border-border-subtle rounded-xl shadow-2xl overflow-hidden">
        <div
          ref={(el) => {
            if (el) (inputRef as unknown as { current: HTMLInputElement | null }).current =
              el.querySelector("input");
          }}
          className="contents"
        >
          <TitleBar
            query={query}
            onQueryChange={setQuery}
            onClear={handleClear}
            onClose={handleClose}
            hotkeyRegistered={true}
          />
        </div>

        {results.length === 0 ? (
          <EmptyState
            title={emptyTitle}
            hint={emptyHint}
            actionLabel={indexing ? "Indexing…" : emptyAction}
            onAction={handleAddFolder}
            icon={
              <svg
                className="w-8 h-8"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth={1.5}
              >
                <path d="M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V7z" />
              </svg>
            }
          />
        ) : (
          <ResultList
            results={results}
            selectedIndex={selectedIndex}
            onSelect={setSelectedIndex}
          />
        )}

        <StatusBar
          showing={results.length}
          total={results.length}
          latencyMs={latencyMs}
          degraded={degraded}
        />

        {/* Phase 2 footer: folder + index stats. Shown whenever folders exist. */}
        {folders.length > 0 && stats && (
          <div className="px-4 py-1.5 border-t border-border-subtle text-[11px] text-fg-tertiary flex items-center gap-3 shrink-0">
            <span className="truncate">
              {folders.length} {folders.length === 1 ? "folder" : "folders"}
            </span>
            <span className="font-mono">{stats.files.toLocaleString()} files</span>
            <span className="font-mono">{stats.chunks.toLocaleString()} chunks</span>
            {searchStatus && (
              <span className="font-mono">
                {searchStatus.model} ({searchStatus.dim}d)
                {searchStatus.circuit_open ? " ⚠" : ""}
              </span>
            )}
            <span className="ml-auto">
              <button
                onClick={handleAddFolder}
                className="text-fg-secondary hover:text-fg-primary"
              >
                + Add folder
              </button>
            </span>
          </div>
        )}
      </div>
    </div>
  );
}
