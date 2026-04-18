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
    items: [
      CardPool.NoviceConvene,
      CardPool.BeginnerChoiceConvene,
      CardPool.GivebackCustomConvene,
    ],
  },
];

interface SidebarProps {
  users: string[];
  playerId: string | null;
  activePool: CardPool | null;
  onSelectUser: (playerId: string) => void;
  onSelectPool: (pool: CardPool) => void;
  onAddUser: () => void;
  onExport: () => void;
}

export default function Sidebar(props: SidebarProps) {
  return (
    <nav class="sidebar">
      <div class="user-selector">
        <label class="user-selector-label">当前用户</label>
        <select
          class="user-selector-input"
          value={props.playerId ?? ""}
          onChange={(e) => props.onSelectUser(e.currentTarget.value)}
        >
          <For each={props.users}>
            {(id) => <option value={id}>{id}</option>}
          </For>
        </select>
      </div>
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
        <div class="nav-item" onClick={props.onAddUser}>
          添加用户
        </div>
        <div class="nav-item" onClick={props.onExport}>
          导出
        </div>
      </div>
    </nav>
  );
}
