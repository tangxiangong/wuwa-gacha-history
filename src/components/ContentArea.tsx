import { createSignal, createEffect, createMemo, on, Show } from "solid-js";
import { queryGachaRecords } from "../lib/commands";
import { CARD_POOL_LABELS, QualityLevel } from "../lib/types";
import type { CardPool, GachaFilter, GachaRecord } from "../lib/types";
import FilterPanel from "./FilterPanel";
import RecordTable from "./RecordTable";
import BarsView from "./BarsView";
import CardsView from "./CardsView";
import SummaryView from "./SummaryView";
import { bannerStats, enrichPulls } from "../lib/stats";

type ViewMode = "bars" | "cards" | "summary";

interface ContentAreaProps {
  activePool: CardPool | null;
  playerId: string;
}

export default function ContentArea(props: ContentAreaProps) {
  const [records, setRecords] = createSignal<GachaRecord[]>([]);
  const [loading, setLoading] = createSignal(false);
  const [view, setView] = createSignal<ViewMode>("bars");

  const [filterOpen, setFilterOpen] = createSignal(false);
  const [qualityLevel, setQualityLevel] = createSignal<QualityLevel | null>(null);
  const [nameQuery, setNameQuery] = createSignal("");
  const [timeFrom, setTimeFrom] = createSignal("");
  const [timeTo, setTimeTo] = createSignal("");

  const hasActiveFilter = () =>
    qualityLevel() !== null ||
    nameQuery() !== "" ||
    timeFrom() !== "" ||
    timeTo() !== "";

  function buildFilter(): GachaFilter {
    return {
      cardPool: props.activePool,
      qualityLevel: qualityLevel(),
      name: nameQuery() || null,
      timeFrom: timeFrom() ? `${timeFrom()}T00:00:00` : null,
      timeTo: timeTo() ? `${timeTo()}T23:59:59` : null,
      limit: null,
      offset: null,
    };
  }

  async function loadRecords() {
    if (!props.activePool || !props.playerId) return;
    setLoading(true);
    try {
      const result = await queryGachaRecords(props.playerId, buildFilter());
      setRecords(result);
    } catch (e) {
      console.error("Failed to query records:", e);
      setRecords([]);
    } finally {
      setLoading(false);
    }
  }

  createEffect(
    on(
      () => [
        props.activePool,
        props.playerId,
        qualityLevel(),
        nameQuery(),
        timeFrom(),
        timeTo(),
      ],
      () => loadRecords(),
    ),
  );

  const chrono = createMemo(() => enrichPulls(records()));
  const stats = createMemo(() => bannerStats(chrono()));

  return (
    <div class="content-area">
      <Show
        when={props.activePool !== null}
        fallback={<div class="record-empty">请选择一个卡池类型</div>}
      >
        <div class="content-header">
          <span class="content-title">
            {CARD_POOL_LABELS[props.activePool!]}
          </span>
          <button
            class={`filter-toggle ${hasActiveFilter() ? "has-filter" : ""}`}
            onClick={() => setFilterOpen(!filterOpen())}
          >
            {filterOpen() ? "▲ 筛选" : "▼ 筛选"}
          </button>
        </div>
        <FilterPanel
          open={filterOpen()}
          qualityLevel={qualityLevel()}
          nameQuery={nameQuery()}
          timeFrom={timeFrom()}
          timeTo={timeTo()}
          onQualityChange={setQualityLevel}
          onNameChange={setNameQuery}
          onTimeFromChange={setTimeFrom}
          onTimeToChange={setTimeTo}
        />

        <div class="view-modes">
          <button
            class={`vm-pill ${view() === "bars" ? "active" : ""}`}
            onClick={() => setView("bars")}
          >
            条形式
          </button>
          <button
            class={`vm-pill ${view() === "cards" ? "active" : ""}`}
            onClick={() => setView("cards")}
          >
            卡片式
          </button>
          <button
            class={`vm-pill ${view() === "summary" ? "active" : ""}`}
            onClick={() => setView("summary")}
          >
            版本总结
          </button>
        </div>

        <div class="sum-strip">
          <div class="sum-cell">
            <div class="v">{stats().total}</div>
            <div class="k">抽卡数</div>
          </div>
          <div class="sum-cell">
            <div class="v">{stats().upCount}</div>
            <div class="k">UP数</div>
          </div>
          <div class="sum-cell">
            <div class="v">
              {stats().upCount
                ? (stats().total / stats().upCount).toFixed(1)
                : "—"}
            </div>
            <div class="k">每UP抽数</div>
          </div>
          <div class="sum-cell">
            <div class="v muted">
              {stats().strayCount}/{stats().r5.length}
            </div>
            <div class="k">歪/出卡数</div>
          </div>
        </div>

        <Show when={view() === "bars"}>
          <BarsView pulls={chrono()} />
        </Show>
        <Show when={view() === "cards"}>
          <CardsView pulls={chrono()} />
        </Show>
        <Show when={view() === "summary"}>
          <SummaryView pulls={chrono()} />
        </Show>

        <RecordTable pulls={chrono()} loading={loading()} />
      </Show>
    </div>
  );
}
