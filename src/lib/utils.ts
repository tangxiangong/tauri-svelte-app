import type { Storage } from "./models";

export function pct(used: Storage | undefined, total: Storage | undefined): number {
  if (!used || !total) return 0;
  const t = total.bytes;
  if (t <= 0) return 0;
  return Math.min(100, Math.round((used.bytes / t) * 100));
}