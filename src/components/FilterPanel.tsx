import { Show } from "solid-js";
import { QualityLevel } from "../lib/types";

interface FilterPanelProps {
  open: boolean;
  qualityLevel: QualityLevel | null;
  nameQuery: string;
  timeFrom: string;
  timeTo: string;
  onQualityChange: (level: QualityLevel | null) => void;
  onNameChange: (name: string) => void;
  onTimeFromChange: (date: string) => void;
  onTimeToChange: (date: string) => void;
}

export default function FilterPanel(props: FilterPanelProps) {
  function toggleQuality(level: QualityLevel) {
    props.onQualityChange(props.qualityLevel === level ? null : level);
  }

  function chipClass(level: QualityLevel): string {
    if (props.qualityLevel !== level) return "chip";
    return `chip active-${level}`;
  }

  return (
    <Show when={props.open}>
      <div class="filter-panel">
        <div class="filter-row">
          <span class="filter-label">星级</span>
          <button
            class={chipClass(QualityLevel.FiveStar)}
            onClick={() => toggleQuality(QualityLevel.FiveStar)}
          >
            5★
          </button>
          <button
            class={chipClass(QualityLevel.FourStar)}
            onClick={() => toggleQuality(QualityLevel.FourStar)}
          >
            4★
          </button>
          <button
            class={chipClass(QualityLevel.ThreeStar)}
            onClick={() => toggleQuality(QualityLevel.ThreeStar)}
          >
            3★
          </button>
        </div>
        <div class="filter-row">
          <span class="filter-label">名称</span>
          <input
            class="filter-input"
            placeholder="搜索角色/武器名称..."
            value={props.nameQuery}
            onInput={(e) => props.onNameChange(e.currentTarget.value)}
          />
        </div>
        <div class="filter-row">
          <span class="filter-label">时间</span>
          <input
            type="date"
            class="filter-input filter-input-short"
            value={props.timeFrom}
            onInput={(e) => props.onTimeFromChange(e.currentTarget.value)}
          />
          <span class="filter-separator">—</span>
          <input
            type="date"
            class="filter-input filter-input-short"
            value={props.timeTo}
            onInput={(e) => props.onTimeToChange(e.currentTarget.value)}
          />
        </div>
      </div>
    </Show>
  );
}
