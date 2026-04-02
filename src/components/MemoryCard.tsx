import type { Memory } from "../lib/models";
import { pct } from "../lib/utils";

export function MemoryCard(props: { memory: Memory }) {
  return (
    <section class="card card-elevated card-compact">
      <div class="card-head">
        <span class="card-icon card-icon--ram" aria-hidden="true"></span>
        <h2>内存与交换</h2>
      </div>
      <div class="meter-block">
        <div class="meter-head meter-head--compact">
          <span class="meter-label">物理内存</span>
          <span class="meter-value">
            {props.memory.usedMemory.display}
            <span class="meter-sep">/</span>
            {props.memory.totalMemory.display}
            <span class="meter-pct">
              （{pct(props.memory.usedMemory, props.memory.totalMemory)}%）
            </span>
          </span>
        </div>
        <div
          class="meter meter--compact"
          style={`--p: ${pct(props.memory.usedMemory, props.memory.totalMemory)}`}
          role="progressbar"
          aria-valuenow={pct(props.memory.usedMemory, props.memory.totalMemory)}
          aria-valuemin="0"
          aria-valuemax="100"
        ></div>
      </div>
      <div class="meter-block">
        <div class="meter-head meter-head--compact">
          <span class="meter-label">交换分区</span>
          <span class="meter-value">
            {props.memory.usedSwap.display}
            <span class="meter-sep">/</span>
            {props.memory.totalSwap.display}
            <span class="meter-pct">
              （{pct(props.memory.usedSwap, props.memory.totalSwap)}%）
            </span>
          </span>
        </div>
        <div
          class="meter meter--compact meter--swap"
          style={`--p: ${pct(props.memory.usedSwap, props.memory.totalSwap)}`}
          role="progressbar"
          aria-valuenow={pct(props.memory.usedSwap, props.memory.totalSwap)}
          aria-valuemin="0"
          aria-valuemax="100"
        ></div>
      </div>
    </section>
  );
}
