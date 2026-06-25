import type { ReactNode } from "react";
import { FolderOpen } from "lucide-react";

interface EmptyStateProps {
  title: string;
  hint: string;
  actionLabel?: string;
  onAction?: () => void;
  icon?: ReactNode;
}

export function EmptyState({ title, hint, actionLabel, onAction, icon }: EmptyStateProps) {
  return (
    <div className="flex flex-col items-center justify-center gap-3 py-12 text-center px-6">
      <div className="text-fg-tertiary">
        {icon ?? <FolderOpen className="w-8 h-8" strokeWidth={1.5} />}
      </div>
      <div className="text-sm font-medium text-fg-secondary">{title}</div>
      <div className="text-xs text-fg-tertiary max-w-[280px]">{hint}</div>
      {actionLabel && onAction && (
        <button
          onClick={onAction}
          className="
            mt-2 px-4 py-1.5 rounded-md
            bg-accent text-accent-fg
            text-sm font-medium
            hover:opacity-90 transition-opacity
          "
        >
          {actionLabel}
        </button>
      )}
    </div>
  );
}
