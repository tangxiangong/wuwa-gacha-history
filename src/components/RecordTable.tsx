import { For, Show, createMemo, createSignal } from "solid-js";
import { QualityLevel } from "../lib/types";
import { assetPath, isCharacter } from "../lib/catalog";
import { type EnrichedPull, SOFT_PITY } from "../lib/stats";

function fmtDate(iso: string): string {
  const d = new Date(iso);
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${d.getFullYear()}.${pad(d.getMonth() + 1)}.${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}`;
}

function kindLabel(name: string, rarity: QualityLevel): string {
  if (rarity === QualityLevel.FiveStar) {
    return isCharacter(name) ? "角色" : "武器";
  }
  if (rarity === QualityLevel.FourStar) {
    return isCharacter(name) ? "角色" : "装备";
  }
  return "道具";
}

interface RecordTableProps {
  pulls: EnrichedPull[];
  loading: boolean;
}

export default function RecordTable(props: RecordTableProps) {
  const [order, setOrder] = createSignal<"desc" | "asc">("desc");

  const displayed = createMemo(() =>
    order() === "desc" ? [...props.pulls].reverse() : props.pulls,
  );

  const dateRange = createMemo(() => {
    const list = props.pulls;
    if (list.length === 0) return "—";
    return `${fmtDate(list[0].record.time)} → ${fmtDate(list[list.length - 1].record.time)}`;
  });

  return (
    <section class="log-section">
      <header class="section-head">
        <div class="section-no">§ 04</div>
        <h2 class="section-title">
          完整记录 · <em>The Complete Log</em>
        </h2>
        <div class="section-meta">逐抽回溯 · 保底 / UP 实时标注</div>
      </header>

      <div class="log-toolbar">
        <div class="log-count">
          共 <span class="n">{props.pulls.length}</span> 条记录
        </div>
        <button
          class="sort-toggle"
          onClick={() => setOrder((o) => (o === "desc" ? "asc" : "desc"))}
        >
          排序 · {order() === "desc" ? "最新 ↓" : "最旧 ↑"}
        </button>
      </div>

      <div class="log-wrap">
        <div class="log-head">
          <span>#</span>
          <span>物品</span>
          <span>时间</span>
          <span>版本</span>
          <span>类型</span>
          <span>稀有度</span>
          <span>保底 / UP</span>
        </div>

        <div class="log-body">
          <Show
            when={!props.loading && displayed().length > 0}
            fallback={
              <div class="log-empty">
                {props.loading ? "加载中…" : "暂无记录"}
              </div>
            }
          >
            <For each={displayed()}>
              {(e) => {
                const r = e.record;
                const rarity = r.qualityLevel;
                return (
                  <div class={`log-row r${rarity}`}>
                    <span class="idx">
                      {String(e.index).padStart(3, "0")}
                    </span>
                    <span class="name">
                      <img
                        class="portrait"
                        src={assetPath(r.name)}
                        alt=""
                        loading="lazy"
                        onError={(ev) => {
                          (ev.currentTarget as HTMLImageElement).style.visibility =
                            "hidden";
                        }}
                      />
                      <span class="name-text">{r.name}</span>
                      <Show when={rarity === QualityLevel.FiveStar && e.isUp}>
                        <span class="up-tag">UP</span>
                      </Show>
                    </span>
                    <span class="time">{fmtDate(r.time)}</span>
                    <span class="version-cell">
                      {r.version ? `V${r.version}` : "—"}
                    </span>
                    <span class="kind">{kindLabel(r.name, rarity)}</span>
                    <span class="rarity">★{rarity}</span>
                    <span
                      class={`pity ${
                        rarity === QualityLevel.FiveStar &&
                        (e.pityAtPull ?? 0) >= SOFT_PITY
                          ? "over"
                          : ""
                      }`}
                    >
                      {rarity === QualityLevel.FiveStar
                        ? `${e.pityAtPull ?? "—"} 抽`
                        : "—"}
                    </span>
                  </div>
                );
              }}
            </For>
          </Show>
        </div>

        <div class="log-footer">
          <span>区间 · {dateRange()}</span>
          <span>全部数据可通过导出 CSV / XLSX / JSON 下载</span>
        </div>
      </div>
    </section>
  );
}
