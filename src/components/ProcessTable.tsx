import type { ProcessMemoryInfo } from "../lib/models";

export function ProcessTable(props: { processes: ProcessMemoryInfo[] }) {
  return (
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
            {props.processes.map((p, i) => (
              <tr classList={{ "row-alt": i % 2 === 1 }}>
                <td class="mono td-numeric">{p.pid}</td>
                <td class="td-name" title={p.name}>{p.name}</td>
                <td class="mono td-numeric td-usage">{p.totalMemory.display}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  );
}
