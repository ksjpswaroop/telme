import { Search, X } from "lucide-react";

interface TitleBarProps {
  query: string;
  onQueryChange: (q: string) => void;
  onClear: () => void;
  onClose: () => void;
  hotkeyRegistered: boolean;
}

export function TitleBar({
  query,
  onQueryChange,
  onClear,
  onClose,
  hotkeyRegistered,
}: TitleBarProps) {
  return (
    <div className="flex items-center gap-3 px-5 h-14 shrink-0">
      <Search
        className="w-5 h-5 text-fg-tertiary shrink-0"
        strokeWidth={1.75}
        aria-hidden
      />
      <input
        id="telme-input"
        type="text"
        autoFocus
        autoComplete="off"
        spellCheck={false}
        value={query}
        onChange={(e) => onQueryChange(e.target.value)}
        placeholder={hotkeyRegistered ? "Search your files…" : "Press hotkey or use tray"}
        className="
          flex-1 bg-transparent
          text-[20px] leading-7 text-fg-primary
          placeholder:text-fg-tertiary
          outline-none border-none
          font-sans tracking-tight
        "
        style={{ fontFeatureSettings: '"ss01", "cv11"' }}
        aria-label="Search Telme"
      />
      {query.length > 0 && (
        <button
          onClick={onClear}
          aria-label="Clear query"
          className="
            w-7 h-7 rounded-md grid place-items-center
            text-fg-tertiary hover:text-fg-primary hover:bg-bg-base
            transition-colors duration-80
          "
        >
          <X className="w-4 h-4" strokeWidth={2} />
        </button>
      )}
      <button
        onClick={onClose}
        aria-label="Close"
        className="
          w-7 h-7 rounded-md grid place-items-center
          text-fg-tertiary hover:text-fg-primary hover:bg-bg-base
          transition-colors duration-80
        "
      >
        <span className="text-xs font-mono">esc</span>
      </button>
    </div>
  );
}
