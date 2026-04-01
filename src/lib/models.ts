/** 与 Rust `Storage` 的 JSON 一致；`invoke` 只能得到数据，没有 Rust 方法，请用 `bytes` / `display`。 */
export type Storage = {
  quotient: number;
  remainder: number;
  unit: Unit;
  bytes: number;
  display: string;
};

export enum Unit {
  B = "B",
  KB = "KB",
  MB = "MB",
  GB = "GB",
  TB = "TB",
  PB = "PB",
}

export type ProcessMemoryInfo = {
  pid: number;
  memory: Storage;
  rawMemory: number;
  name: string;
  exe: string | null;
  parent: number | null;
  root: string | null;
  totalMemory: Storage;
};

export type Memory = {
  totalMemory: Storage;
  usedMemory: Storage;
  totalSwap: Storage;
  usedSwap: Storage;
};

/** 与 `get_used_memory` 返回的 JSON 一致 */
export type UsedMemorySnapshot = {
  memory: Memory;
  topProcesses: ProcessMemoryInfo[];
};

export type SystemInfo = {
  osName: string | null;
  kernelVersion: string | null;
  osVersion: string | null;
  longOsVersion: string | null;
  hostName: string | null;
  cpuCount: number;
};
