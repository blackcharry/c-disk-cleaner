// src/utils/format.ts

export function formatSize(bytes: number): string {
  if (bytes === 0) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.min(
    Math.floor(Math.log(bytes) / Math.log(1024)),
    units.length - 1
  );
  const v = bytes / Math.pow(1024, i);
  return `${v.toFixed(i === 0 ? 0 : 1)} ${units[i]}`;
}

export function formatDuration(ms: number): string {
  if (ms < 1000) return `${ms}ms`;
  const s = (ms / 1000).toFixed(1);
  return `${s}s`;
}

export function shortenPath(path: string, maxLen: number = 60): string {
  if (path.length <= maxLen) return path;
  const parts = path.split('\\');
  if (parts.length <= 2) return path;
  return `${parts[0]}\\...\\${parts[parts.length - 1]}`;
}
