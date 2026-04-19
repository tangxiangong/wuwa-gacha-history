import { For, Show, createMemo } from "solid-js";
import {
  type EnrichedPull,
  type Version,
  fmtDay,
  groupByVersion,
} from "../lib/stats";

interface SummaryViewProps {
  pulls: EnrichedPull[];
}

export default function SummaryView(props: SummaryViewProps) {
  const versions = createMemo(() => {
    const list = groupByVersion(props.pulls);
    return [...list].reverse(); // oldest first for reading left-to-right
  });
  const maxPulls = createMemo(() =>
    Math.max(1, ...versions().map((v) => v.total)),
  );

  return (
    <div class="summary-view">
      <div class="summary-grid">
        <div class="sum-card">
          <h3>每版本抽取数</h3>
          <div class="sum-chart">
            <Show
              when={versions().length > 0}
              fallback={<div class="sum-empty">暂无数据</div>}
            >
              <For each={versions()}>
                {(v) => <VersionBar version={v} maxPulls={maxPulls()} />}
              </For>
            </Show>
          </div>
        </div>
        <div class="sum-card">
          <h3>每版本五星</h3>
          <div class="sum-chart">
            <Show
              when={versions().length > 0}
              fallback={<div class="sum-empty">暂无数据</div>}
            >
              <For each={versions()}>{(v) => <VersionR5Row version={v} />}</For>
            </Show>
          </div>
        </div>
      </div>

      <div class="sum-card">
        <h3>版本详情</h3>
        <div class="sum-table">
          <div class="sum-th">版本</div>
          <div class="sum-th">开启日期</div>
          <div class="sum-th">抽数</div>
          <div class="sum-th">五星</div>
          <div class="sum-th">UP</div>
          <div class="sum-th">常驻</div>
          <div class="sum-th">均金</div>
          <For each={versions()}>
            {(v) => {
              const avg = v.r5.length ? (v.total / v.r5.length).toFixed(1) : "—";
              return (
                <>
                  <div class="sum-td v-label">V{v.version}</div>
                  <div class="sum-td">{fmtDay(v.start)}</div>
                  <div class="sum-td num">{v.total}</div>
                  <div class="sum-td num gold">{v.r5.length}</div>
                  <div class="sum-td num">{v.ups}</div>
                  <div class="sum-td num muted">{v.stray}</div>
                  <div class="sum-td num">{avg}</div>
                </>
              );
            }}
          </For>
        </div>
      </div>
    </div>
  );
}

function VersionBar(props: { version: Version; maxPulls: number }) {
  const v = () => props.version;
  const widthPct = () => (v().total / props.maxPulls) * 100;
  return (
    <div class="sum-row">
      <span class="sum-row-label">V{v().version}</span>
      <div class="sum-row-track">
        <div class="sum-row-fill" style={{ width: `${widthPct()}%` }} />
      </div>
      <span class="sum-row-num">{v().total}</span>
    </div>
  );
}

function VersionR5Row(props: { version: Version }) {
  const v = () => props.version;
  return (
    <div class="sum-row">
      <span class="sum-row-label">V{v().version}</span>
      <div class="sum-row-stars">
        <Show
          when={v().r5.length > 0}
          fallback={<span class="sum-row-none">无五星</span>}
        >
          <For each={v().r5}>
            {(p) => (
              <span
                class={`pentagon ${p.isUp ? "up" : "stray"}`}
                title={p.record.name}
              />
            )}
          </For>
        </Show>
      </div>
      <span class={`sum-row-num ${v().r5.length ? "gold" : "muted"}`}>
        {v().r5.length}
      </span>
    </div>
  );
}
