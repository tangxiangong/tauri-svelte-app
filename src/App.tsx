import { createSignal, createEffect, onCleanup } from "solid-js";
import "./App.css";
import { getUsedMemory, getSystemInfo } from "./lib/commands";
import type { Memory, ProcessMemoryInfo, SystemInfo } from "./lib/models";
import { OsInfoCard } from "./components/OsInfoCard";
import { MemoryCard } from "./components/MemoryCard";
import { ProcessTable } from "./components/ProcessTable";

function useSystemMonitor(intervalMs = 1000) {
  const [systemInfo, setSystemInfo] = createSignal<SystemInfo | null>(null);
  const [memory, setMemory] = createSignal<Memory | null>(null);
  const [topProcesses, setTopProcesses] = createSignal<ProcessMemoryInfo[] | null>(null);
  const [loadError, setLoadError] = createSignal<string | null>(null);

  let refreshInFlight = false;

  const refreshSystem = async () => {
    if (refreshInFlight) return;
    refreshInFlight = true;
    setLoadError(null);
    try {
      const [info, snapshot] = await Promise.all([
        getSystemInfo(),
        getUsedMemory(10),
      ]);
      setSystemInfo(info);
      setMemory(snapshot.memory);
      setTopProcesses(snapshot.topProcesses);
    } catch (e) {
      setLoadError(e instanceof Error ? e.message : "无法加载系统信息");
      setSystemInfo(null);
      setMemory(null);
      setTopProcesses(null);
    } finally {
      refreshInFlight = false;
    }
  };

  createEffect(() => {
    void refreshSystem();
    const interval = setInterval(() => {
      void refreshSystem();
    }, intervalMs);
    onCleanup(() => clearInterval(interval));
  });

  return { systemInfo, memory, topProcesses, loadError };
}

function App() {
  const { systemInfo, memory, topProcesses, loadError } = useSystemMonitor();

  return (
    <div class="page-inner">
      <header class="header">
        <div class="header-brand">
          <h1>系统信息</h1>
          <span class="live-badge">
            <span class="live-dot" aria-hidden="true"></span>
            实时
          </span>
          <p class="header-sub">内存与进程概览</p>
        </div>
      </header>

      {loadError() && (
        <div class="error" role="alert">
          <span class="error-icon" aria-hidden="true">!</span>
          {loadError()}
        </div>
      )}

      {(systemInfo() || memory()) && (
        <div class="card-row">
          {systemInfo() && <OsInfoCard info={systemInfo()!} />}
          {memory() && <MemoryCard memory={memory()!} />}
        </div>
      )}

      {topProcesses() && <ProcessTable processes={topProcesses()!} />}
    </div>
  );
}
export default App;