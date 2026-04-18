import { createSignal, createEffect, on, Show } from "solid-js";
import { queryGachaRecords } from "../lib/commands";
import { CARD_POOL_LABELS, QualityLevel } from "../lib/types";
import type { CardPool, GachaFilter, GachaRecord } from "../lib/types";
import FilterPanel from "./FilterPanel";
import RecordTable from "./RecordTable";
import Pagination from "./Pagination";

const PAGE_SIZE = 20;

interface ContentAreaProps {
  activePool: CardPool | null;
  playerId: string;
}

export default function ContentArea(props: ContentAreaProps) {
  const [records, setRecords] = createSignal<GachaRecord[]>([]);
  const [loading, setLoading] = createSignal(false);
  const [page, setPage] = createSignal(1);
  const [totalRecords, setTotalRecords] = createSignal(0);

  // Filter state
  const [filterOpen, setFilterOpen] = createSignal(false);
  const [qualityLevel, setQualityLevel] = createSignal<QualityLevel | null>(
    null,
  );
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
      limit: PAGE_SIZE,
      offset: (page() - 1) * PAGE_SIZE,
    };
  }

  async function loadRecords() {
    if (!props.activePool || !props.playerId) return;
    setLoading(true);
    try {
      const filter = buildFilter();
      const result = await queryGachaRecords(props.playerId, filter);
      setRecords(result);

      // Fetch total count (without limit/offset) for pagination
      const countFilter = { ...filter, limit: null, offset: null };
      const allResults = await queryGachaRecords(props.playerId, countFilter);
      setTotalRecords(allResults.length);
    } catch (e) {
      console.error("Failed to query records:", e);
      setRecords([]);
      setTotalRecords(0);
    } finally {
      setLoading(false);
    }
  }

  // Reset page and reload when user, pool, or filters change
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
      () => {
        setPage(1);
        loadRecords();
      },
    ),
  );

  // Reload when page changes (but don't reset page)
  createEffect(
    on(
      () => page(),
      () => loadRecords(),
      { defer: true },
    ),
  );

  function handlePageChange(newPage: number) {
    setPage(newPage);
  }

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
        <RecordTable records={records()} loading={loading()} />
        <Pagination
          currentPage={page()}
          totalRecords={totalRecords()}
          pageSize={PAGE_SIZE}
          onPageChange={handlePageChange}
        />
      </Show>
    </div>
  );
}
