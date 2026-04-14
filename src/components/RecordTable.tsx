import { For, Show } from "solid-js";
import type { GachaRecord } from "../lib/types";
import { QualityLevel } from "../lib/types";

function qualityClass(level: QualityLevel): string {
  switch (level) {
    case QualityLevel.FiveStar:
      return "star-5";
    case QualityLevel.FourStar:
      return "star-4";
    case QualityLevel.ThreeStar:
      return "star-3";
  }
}

function qualityText(level: QualityLevel): string {
  return `${level}★`;
}

function formatTime(time: string): string {
  return time.replace("T", " ").slice(0, 16);
}

interface RecordTableProps {
  records: GachaRecord[];
  loading: boolean;
}

export default function RecordTable(props: RecordTableProps) {
  return (
    <div class="record-table">
      <div class="record-table-header">
        <span class="col-name">名称</span>
        <span class="col-quality">星级</span>
        <span class="col-time">时间</span>
      </div>
      <Show
        when={!props.loading && props.records.length > 0}
        fallback={
          <div class="record-empty">
            {props.loading ? "加载中..." : "暂无记录"}
          </div>
        }
      >
        <For each={props.records}>
          {(record) => (
            <div class={`record-row ${qualityClass(record.qualityLevel)}`}>
              <span class="col-name">{record.name}</span>
              <span class="col-quality">
                {qualityText(record.qualityLevel)}
              </span>
              <span class="col-time">{formatTime(record.time)}</span>
            </div>
          )}
        </For>
      </Show>
    </div>
  );
}
