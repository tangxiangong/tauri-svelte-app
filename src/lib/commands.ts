import { invoke } from "@tauri-apps/api/core";
import type { SystemInfo, UsedMemorySnapshot } from "./models";

export async function getUsedMemory(
  topN?: number,
): Promise<UsedMemorySnapshot> {
  return await invoke<UsedMemorySnapshot>(
    "get_used_memory",
    topN === undefined ? {} : { topN },
  );
}

export async function getSystemInfo(): Promise<SystemInfo> {
  return await invoke<SystemInfo>("get_system_info");
}
