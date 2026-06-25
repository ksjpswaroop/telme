import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { TitleBar } from "./components/TitleBar";
import { EmptyState } from "./components/EmptyState";
import { ResultList, type SearchResult } from "./components/ResultList";
import { StatusBar } from "./components/StatusBar";

export default function App() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [latencyMs, setLatencyMs] = useState(0);
  const [degraded, setDegraded] = useState(false);

  const inputRef = useRef<HTMLInputElement | null>(null);

  // Show + focus the title bar on mount
  useEffect(() => {
    const win = getCurrentWindow();
    win.show().catch(console.error);
    win.setFocus().catch(console.error);
    inputRef.current?.focus();
  }, []);

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

  // Phase 0: query → no results yet (indexing not implemented)
  // Phase 1+: this is where the debounced search command will fire.
  useEffect(() => {
    if (query.trim().length === 0) {
      setResults([]);
      setLatencyMs(0);
      setDegraded(false);
      return;
    }
    // Placeholder behavior until Phase 3 wires the backend search command.
    setResults([]);
    setLatencyMs(0);
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
        // Phase 0: no backend open command yet.
        console.log("Phase 0 placeholder: open file", r.path);
      }
    };
    input.addEventListener("keydown", handler);
    return () => input.removeEventListener("keydown", handler);
  }, [results, selectedIndex, query]);

  return (
    <div
      className="h-screen w-screen flex items-start justify-center pt-[80px] bg-transparent"
      data-tauri-drag-region
    >
      <div className="w-[700px] max-h-[600px] flex flex-col bg-bg-elevated border border-border-subtle rounded-xl shadow-2xl overflow-hidden">
        <div ref={(el) => { if (el) (inputRef as any).current = el.querySelector("input"); }} className="contents">
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
            title="No folders indexed yet."
            hint="Add a folder from Settings to begin."
            actionLabel="Open Settings"
            onAction={() => {
              // Settings window not implemented in Phase 0.
              console.log("Phase 0 placeholder: open settings");
            }}
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
      </div>
    </div>
  );
}
