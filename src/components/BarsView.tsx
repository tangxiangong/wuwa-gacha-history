import { For, Show, createMemo } from "solid-js";
import { assetPath } from "../lib/catalog";
import {
  type EnrichedPull,
  type FiveStarSegment,
  pityColorClass,
  segmentsByFive,
} from "../lib/stats";

interface BarsViewProps {
  pulls: EnrichedPull[];
}

export default function BarsView(props: BarsViewProps) {
  const segments = createMemo(() => segmentsByFive(props.pulls));
  const rows = createMemo(() => [...segments()].reverse());
  const maxPity = createMemo(() =>
    Math.max(80, ...segments().map((s) => s.pity)),
  );

  return (
    <div class="bars-wrap">
      <Show
        when={rows().length > 0}
        fallback={<div class="bars-empty">暂无 5★ 段</div>}
      >
        <For each={rows()}>
          {(s) => <BarRow segment={s} maxPity={maxPity()} />}
        </For>
      </Show>
    </div>
  );
}

function BarRow(props: { segment: FiveStarSegment; maxPity: number }) {
  const s = () => props.segment;
  const fillClass = () => (s().pad ? "good" : pityColorClass(s().pity));
  const widthPct = () => Math.min(100, (s().pity / props.maxPity) * 100);
  const name = () => s().end?.record.name ?? "垫刀";
  const isUp = () => !s().pad && s().isUp;

  return (
    <div class="bar-row">
      <Show
        when={!s().pad && s().end}
        fallback={<div class="bar-avatar pad">垫</div>}
      >
        <div class={`bar-avatar ${isUp() ? "up5" : ""}`}>
          <img
            src={assetPath(s().end!.record.name)}
            alt=""
            loading="lazy"
            onError={(ev) => {
              (ev.currentTarget as HTMLImageElement).style.visibility = "hidden";
            }}
          />
        </div>
      </Show>
      <div class="bar-name-col">
        <div class={`bar-fill ${fillClass()}`} style={{ width: `${widthPct()}%` }}>
          {s().pity}抽
        </div>
      </div>
      <div class="bar-tag">
        <Show when={!s().pad}>
          <span>{name()}</span>
          <Show when={isUp()}>
            <span class="up">UP</span>
          </Show>
        </Show>
        <Show when={s().pad}>
          <span class="pad-tag">垫刀中</span>
        </Show>
      </div>
    </div>
  );
}
