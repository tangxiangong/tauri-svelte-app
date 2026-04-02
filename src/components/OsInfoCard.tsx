import type { SystemInfo } from "../lib/models";

export function OsInfoCard(props: { info: SystemInfo }) {
  return (
    <section class="card card-elevated card-compact">
      <div class="card-head">
        <span class="card-icon card-icon--os" aria-hidden="true"></span>
        <h2>操作系统</h2>
      </div>
      <dl class="kv kv-compact">
        <dt>主机名</dt>
        <dd>{props.info.hostName ?? "—"}</dd>
        <dt>系统名称</dt>
        <dd>{props.info.osName ?? "—"}</dd>
        <dt>内核</dt>
        <dd>{props.info.kernelVersion ?? "—"}</dd>
        <dt>版本</dt>
        <dd>{props.info.osVersion ?? "—"}</dd>
        <dt>详细版本</dt>
        <dd class="kv-long">{props.info.longOsVersion ?? "—"}</dd>
        <dt>逻辑 CPU</dt>
        <dd><span class="chip">{props.info.cpuCount} 核</span></dd>
      </dl>
    </section>
  );
}
