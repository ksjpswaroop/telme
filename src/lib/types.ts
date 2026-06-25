// Shared types between frontend and Tauri commands.
// Mirror these on the Rust side in src-tauri/src/types.rs.

export interface FolderInfo {
  path: string;
  status: "pending" | "indexing" | "indexed" | "error" | "paused";
  lastIndexedAt: string | null;
  fileCount: number;
}

export interface IndexStats {
  files: number;
  chunks: number;
  sizeMb: number;
  model: string;
}

export interface Config {
  folders: string[];
  model: string;
  hotkey: string;
  semanticWeight: number; // 0..1
  topK: number;
  clearHistoryOnQuit: boolean;
}
