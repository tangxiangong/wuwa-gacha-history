import { For, Show, createMemo } from "solid-js";
import { QualityLevel } from "../lib/types";
import { assetPath } from "../lib/catalog";
import {
  type EnrichedPull,
  type Version,
  ASTRITE_PER_PULL,
  fmtDay,
  groupByVersion,
} from "../lib/stats";

interface CardsViewProps {
  pulls: EnrichedPull[];
}

interface GroupedItem {
  name: string;
  rarity: QualityLevel;
  isUp: boolean;
  count: number;
}

function groupItems(version: Version): GroupedItem[] {
  const order: GroupedItem[] = [];
  const index = new Map<string, GroupedItem>();
  for (const p of version.pulls) {
    const rarity = p.record.qualityLevel;
    if (rarity === QualityLevel.ThreeStar) continue;
    const isUp = rarity === QualityLevel.FiveStar ? !!p.isUp : false;
    const key = `${p.record.name}|${rarity}|${isUp ? 1 : 0}`;
    const hit = index.get(key);
    if (hit) {
      hit.count += 1;
    } else {
      const entry: GroupedItem = { name: p.record.name, rarity, isUp, count: 1 };
      index.set(key, entry);
      order.push(entry);
    }
  }
  return order.sort(
    (a, b) =>
      b.rarity - a.rarity || Number(b.isUp) - Number(a.isUp) || b.count - a.count,
  );
}

export default function CardsView(props: CardsViewProps) {
  const versions = createMemo(() => groupByVersion(props.pulls));

  return (
    <div class="cards-wrap">
      <Show
        when={versions().length > 0}
        fallback={<div class="cards-empty">暂无记录</div>}
      >
        <For each={versions()}>{(v) => <VersionBlock version={v} />}</For>
      </Show>
    </div>
  );
}

function VersionBlock(props: { version: Version }) {
  const v = () => props.version;
  const items = createMemo(() => groupItems(v()));
  const spent = () => ((v().total * ASTRITE_PER_PULL) / 10000).toFixed(1);

  const title = () => {
    const ups = v().upNames;
    return ups.length > 0 ? `V${v().version} · ${ups.join(" / ")}` : `V${v().version}`;
  };

  return (
    <div class="version-block">
      <div class="version-head">
        <div class="vh-title">
          <div class="vh-name">{title()}</div>
          <div class="vh-date">{fmtDay(v().start)} 开启</div>
        </div>
        <div class="vh-stat">
          <div class="v">{v().total}</div>
          <div class="k">总抽卡</div>
        </div>
        <div class="vh-stat">
          <div class="v">
            {spent()}
            <span class="u">w</span>
          </div>
          <div class="k">消耗星声</div>
        </div>
        <div class="vh-stat">
          <div class="v">{v().ups}</div>
          <div class="k">限定五星</div>
        </div>
        <div class="vh-stat">
          <div class="v">{v().stray}</div>
          <div class="k">常驻五星</div>
        </div>
        <div class="vh-stat">
          <div class="v">{v().r4.length}</div>
          <div class="k">四星</div>
        </div>
      </div>
      <div class="version-body">
        <Show
          when={items().length > 0}
          fallback={<div class="version-empty">此版本无四星及以上</div>}
        >
          <For each={items()}>{(it) => <CharCard item={it} />}</For>
        </Show>
      </div>
    </div>
  );
}

function CharCard(props: { item: GroupedItem }) {
  const it = () => props.item;
  const r5 = () => it().rarity === QualityLevel.FiveStar;

  return (
    <div class={`char-card ${r5() ? "r5" : ""}`}>
      <img
        class="silo"
        src={assetPath(it().name)}
        alt=""
        loading="lazy"
        onError={(ev) => {
          (ev.currentTarget as HTMLImageElement).style.visibility = "hidden";
        }}
      />
      <Show when={it().isUp}>
        <div class="up-badge">UP</div>
      </Show>
      <div class="count">
        <span class={r5() ? "r5-mark" : "r4-mark"}>★{it().rarity}</span>
        &nbsp;{it().name} × {it().count}
      </div>
    </div>
  );
}
