// src/stores/scanStore.ts

import { create } from 'zustand';
import { startScan } from '../utils/ipc';
import type { FileEntry, ScanResult, RiskLevel } from '../types';

interface ScanState {
  // 扫描状态
  isScanning: boolean;
  progress: number;
  result: ScanResult | null;
  error: string | null;

  // 过滤状态
  riskFilter: 'all' | RiskLevel;
  softwareFilter: string | null;
  searchKeyword: string;
  sortBy: 'size' | 'name' | 'risk';
  sortOrder: 'asc' | 'desc';

  // 操作
  doStartScan: () => Promise<void>;
  setRiskFilter: (filter: 'all' | RiskLevel) => void;
  setSearchKeyword: (keyword: string) => void;
  setSortBy: (field: 'size' | 'name' | 'risk') => void;
  toggleSortOrder: () => void;
  getFilteredEntries: () => FileEntry[];
}

export const useScanStore = create<ScanState>((set, get) => ({
  isScanning: false,
  progress: 0,
  result: null,
  error: null,

  riskFilter: 'all',
  softwareFilter: null,
  searchKeyword: '',
  sortBy: 'size',
  sortOrder: 'desc',

  doStartScan: async () => {
    set({ isScanning: true, progress: 0, error: null });
    try {
      const result = await startScan('C:', 1);
      set({ result, isScanning: false, progress: 100 });
    } catch (err) {
      set({
        isScanning: false,
        error: String(err),
      });
    }
  },

  setRiskFilter: (filter) => set({ riskFilter: filter }),
  setSearchKeyword: (keyword) => set({ searchKeyword: keyword }),
  setSortBy: (field) => set({ sortBy: field }),
  toggleSortOrder: () =>
    set((s) => ({ sortOrder: s.sortOrder === 'desc' ? 'asc' : 'desc' })),

  getFilteredEntries: () => {
    const { result, riskFilter, searchKeyword, sortBy, sortOrder } = get();
    if (!result) return [];

    let entries = [...result.entries];

    // 风险过滤
    if (riskFilter !== 'all') {
      entries = entries.filter((e) => e.risk_level === riskFilter);
    }

    // 搜索过滤
    if (searchKeyword) {
      const kw = searchKeyword.toLowerCase();
      entries = entries.filter(
        (e) =>
          e.path.toLowerCase().includes(kw) ||
          (e.software_name && e.software_name.toLowerCase().includes(kw))
      );
    }

    // 排序
    entries.sort((a, b) => {
      let cmp: number;
      switch (sortBy) {
        case 'size':
          cmp = a.size_bytes - b.size_bytes;
          break;
        case 'name':
          cmp = a.path.localeCompare(b.path);
          break;
        case 'risk':
          const order = { Forbidden: 0, Caution: 1, Safe: 2 };
          cmp = order[a.risk_level] - order[b.risk_level];
          break;
        default:
          cmp = 0;
      }
      return sortOrder === 'desc' ? -cmp : cmp;
    });

    return entries;
  },
}));
