import { FileText, FileCode } from "lucide-react";

export interface SearchResult {
  id: string;
  filename: string;
  path: string;
  snippet: string;
  score: number; // 0..1
  kind: "semantic" | "keyword";
  fileType: "doc" | "code" | "pdf" | "other";
  size?: number;
  modified?: string;
}

interface ResultListProps {
  results: SearchResult[];
  selectedIndex: number;
  onSelect: (i: number) => void;
}

export function ResultList({ results, selectedIndex, onSelect }: ResultListProps) {
  return (
    <ul className="flex flex-col gap-1 px-3 py-2" role="listbox" aria-label="Search results">
      {results.map((r, i) => {
        const selected = i === selectedIndex;
        const Icon = r.fileType === "code" ? FileCode : FileText;
        return (
          <li
            key={r.id}
            role="option"
            aria-selected={selected}
            onMouseEnter={() => onSelect(i)}
            className={`
              flex flex-col gap-0.5 px-3 py-2.5 rounded-md cursor-pointer
              transition-colors duration-60
              ${
                selected
                  ? "bg-accent text-accent-fg"
                  : "text-fg-primary hover:bg-bg-base"
              }
            `}
          >
            <div className="flex items-center gap-2 min-w-0">
              <Icon
                className={`w-4 h-4 shrink-0 ${
                  selected ? "text-accent-fg" : "text-fg-tertiary"
                }`}
                strokeWidth={1.75}
              />
              <span className="text-sm font-medium truncate">{r.filename}</span>
              {selected && (
                <span className="ml-auto text-xs font-mono opacity-70">↵</span>
              )}
            </div>
            <div className="flex items-center gap-2 text-xs">
              <span
                className={`font-mono truncate ${
                  selected ? "opacity-80" : "text-fg-secondary"
                }`}
              >
                {r.path}
              </span>
              <span
                className={`ml-auto shrink-0 px-2 py-0.5 rounded-full text-[11px] ${
                  selected ? "bg-bg-overlay/30 text-accent-fg" : "bg-bg-base text-fg-secondary"
                }`}
              >
                {r.kind === "semantic"
                  ? `${Math.round(r.score * 100)}% match`
                  : "↗ keyword"}
              </span>
            </div>
            {r.snippet && (
              <p
                className={`text-xs leading-[18px] truncate ${
                  selected ? "opacity-80" : "text-fg-secondary"
                }`}
              >
                {r.snippet}
              </p>
            )}
          </li>
        );
      })}
    </ul>
  );
}
