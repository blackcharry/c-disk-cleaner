// src/types/index.ts

export type RiskLevel = 'Safe' | 'Caution' | 'Forbidden';

export const RiskLabel: Record<RiskLevel, string> = {
  Safe: '🟢 安全',
  Caution: '🟡 谨慎',
  Forbidden: '🔴 禁止',
};

export const RiskColor: Record<RiskLevel, string> = {
  Safe: '#52c41a',
  Caution: '#faad14',
  Forbidden: '#ff4d4f',
};

export interface FileEntry {
  path: string;
  is_dir: boolean;
  size_bytes: number;
  file_count: number;
  last_modified: string;
  last_accessed: string;
  risk_level: RiskLevel;
  category: string;
  software_name: string | null;
  description: string | null;
  cleanable_advice: string | null;
}

export interface ScanResult {
  entries: FileEntry[];
  total_size: number;
  used_size: number;
  free_size: number;
  scan_duration_ms: number;
  entry_count: number;
}
