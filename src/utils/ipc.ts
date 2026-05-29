// src/utils/ipc.ts

import { invoke } from '@tauri-apps/api/core';
import type { ScanResult } from '../types';

export async function startScan(
  drive: string = 'C:',
  minSizeMb: number = 1
): Promise<ScanResult> {
  return invoke<ScanResult>('start_scan', {
    drive,
    minSizeMb,
  });
}
