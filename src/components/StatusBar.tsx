interface StatusBarProps {
  showing: number;
  total: number;
  latencyMs: number;
  degraded: boolean;
}

export function StatusBar({ showing, total, latencyMs, degraded }: StatusBarProps) {
  return (
    <div
      className="
        flex items-center justify-between
        px-4 py-1.5
        text-[11px] text-fg-tertiary
        border-t border-border-subtle
        shrink-0
      "
    >
      <span>
        {total === 0
          ? degraded
            ? "⚠ Showing keyword-only results"
            : ""
          : `Showing ${showing} of ${total} results`}
      </span>
      <span className="font-mono">{latencyMs > 0 ? `${latencyMs}ms` : ""}</span>
    </div>
  );
}
