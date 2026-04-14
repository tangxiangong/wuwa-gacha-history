import { For } from "solid-js";
import { CardPool, CARD_POOL_LABELS } from "../lib/types";

interface NavGroup {
  label: string;
  items: CardPool[];
}

const NAV_GROUPS: NavGroup[] = [
  {
    label: "限定池",
    items: [CardPool.FeaturedResonatorConvene, CardPool.FeaturedWeaponConvene],
  },
  {
    label: "常驻池",
    items: [CardPool.StandardResonatorConvene, CardPool.StandardWeaponConvene],
  },
  {
    label: "其他",
    items: [CardPool.NoviceConvene, CardPool.BeginnerChoiceConvene, CardPool.GivebackCustomConvene],
  },
];

interface SidebarProps {
  activePool: CardPool | null;
  onSelectPool: (pool: CardPool) => void;
  onExport: () => void;
}

export default function Sidebar(props: SidebarProps) {
  return (
    <nav class="sidebar">
      <For each={NAV_GROUPS}>
        {(group) => (
          <>
            <div class="nav-group-label">{group.label}</div>
            <For each={group.items}>
              {(pool) => (
                <div
                  class={`nav-item ${props.activePool === pool ? "active" : ""}`}
                  onClick={() => props.onSelectPool(pool)}
                >
                  {CARD_POOL_LABELS[pool]}
                </div>
              )}
            </For>
          </>
        )}
      </For>
      <div class="nav-footer">
        <div class="nav-item" onClick={props.onExport}>
          导出
        </div>
      </div>
    </nav>
  );
}
