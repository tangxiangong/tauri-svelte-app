<script lang="ts">
  import { getUsedMemory, getSystemInfo } from "$lib/commands";
  import type { Memory, ProcessMemoryInfo, SystemInfo } from "$lib/models";
  import { pct } from "$lib/utils";

  let systemInfo = $state<SystemInfo | null>(null);
  let memory = $state<Memory | null>(null);
  let topProcesses = $state<ProcessMemoryInfo[] | null>(null);
  let loadError = $state<string | null>(null);

  let refreshInFlight = false;

  async function refreshSystem() {
    if (refreshInFlight) return;
    refreshInFlight = true;
    loadError = null;
    try {
      const [info, snapshot] = await Promise.all([
        getSystemInfo(),
        getUsedMemory(10),
      ]);
      systemInfo = info;
      memory = snapshot.memory;
      topProcesses = snapshot.topProcesses;
    } catch (e) {
      loadError = e instanceof Error ? e.message : "无法加载系统信息";
      systemInfo = null;
      memory = null;
      topProcesses = null;
    } finally {
      refreshInFlight = false;
    }
  }

  $effect(() => {
    void refreshSystem();
    const id = setInterval(() => {
      void refreshSystem();
    }, 1000);
    return () => clearInterval(id);
  });
</script>

<main class="page">
  <div class="page-inner">
    <header class="header">
      <div class="header-brand">
        <h1>系统信息</h1>
        <span class="live-badge" title="每秒自动刷新">
          <span class="live-dot" aria-hidden="true"></span>
          实时
        </span>
      </div>
      <p class="header-sub">内存与进程概览</p>
    </header>

    {#if loadError}
      <div class="error" role="alert">
        <span class="error-icon" aria-hidden="true">!</span>
        {loadError}
      </div>
    {/if}

    {#if systemInfo || memory}
      <div class="card-row">
        {#if systemInfo}
          <section class="card card-elevated card-compact">
            <div class="card-head">
              <span class="card-icon card-icon--os" aria-hidden="true"></span>
              <h2>操作系统</h2>
            </div>
            <dl class="kv kv-compact">
              <dt>主机名</dt>
              <dd>{systemInfo.hostName ?? "—"}</dd>
              <dt>系统名称</dt>
              <dd>{systemInfo.osName ?? "—"}</dd>
              <dt>内核</dt>
              <dd>{systemInfo.kernelVersion ?? "—"}</dd>
              <dt>版本</dt>
              <dd>{systemInfo.osVersion ?? "—"}</dd>
              <dt>详细版本</dt>
              <dd class="kv-long">{systemInfo.longOsVersion ?? "—"}</dd>
              <dt>逻辑 CPU</dt>
              <dd><span class="chip">{systemInfo.cpuCount} 核</span></dd>
            </dl>
          </section>
        {/if}

        {#if memory}
          <section class="card card-elevated card-compact">
            <div class="card-head">
              <span class="card-icon card-icon--ram" aria-hidden="true"></span>
              <h2>内存与交换</h2>
            </div>
            <div class="meter-block">
              <div class="meter-head meter-head--compact">
                <span class="meter-label">物理内存</span>
                <span class="meter-value"
                  >{memory.usedMemory.display}
                  <span class="meter-sep">/</span>
                  {memory.totalMemory.display}
                  <span class="meter-pct"
                    >（{pct(memory.usedMemory, memory.totalMemory)}%）</span
                  ></span
                >
              </div>
              <div
                class="meter meter--compact"
                style="--p: {pct(memory.usedMemory, memory.totalMemory)}"
                role="progressbar"
                aria-valuenow={pct(memory.usedMemory, memory.totalMemory)}
                aria-valuemin="0"
                aria-valuemax="100"
              ></div>
            </div>
            <div class="meter-block">
              <div class="meter-head meter-head--compact">
                <span class="meter-label">交换分区</span>
                <span class="meter-value"
                  >{memory.usedSwap.display}
                  <span class="meter-sep">/</span>
                  {memory.totalSwap.display}
                  <span class="meter-pct"
                    >（{pct(memory.usedSwap, memory.totalSwap)}%）</span
                  ></span
                >
              </div>
              <div
                class="meter meter--compact meter--swap"
                style="--p: {pct(memory.usedSwap, memory.totalSwap)}"
                role="progressbar"
                aria-valuenow={pct(memory.usedSwap, memory.totalSwap)}
                aria-valuemin="0"
                aria-valuemax="100"
              ></div>
            </div>
          </section>
        {/if}
      </div>
    {/if}

    {#if topProcesses}
      <section class="card card-elevated card-table">
        <div class="card-head">
          <span class="card-icon card-icon--proc" aria-hidden="true"></span>
          <h2>内存占用前列进程</h2>
        </div>
        <div class="table-wrap">
          <table>
            <thead>
              <tr>
                <th class="th-numeric">PID</th>
                <th>名称</th>
                <th class="th-numeric th-usage">占用</th>
              </tr>
            </thead>
            <tbody>
              {#each topProcesses as p, i}
                <tr class:row-alt={i % 2 === 1}>
                  <td class="mono td-numeric">{p.pid}</td>
                  <td class="td-name" title={p.name}>{p.name}</td>
                  <td class="mono td-numeric td-usage"
                    >{p.totalMemory.display}</td
                  >
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </section>
    {/if}
  </div>
</main>
