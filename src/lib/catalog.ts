// Minimal CN-name lookup mirroring wuwa-gacha-history/src/catalog.rs.
// Used by the history log to resolve portraits and flag UP 5★s.

const CHARACTER_NAMES: ReadonlySet<string> = new Set([
  "丹瑾", "仇远", "今汐", "凌阳", "千咲", "卜灵", "卡卡罗", "卡提希娅",
  "吟霖", "嘉贝莉娜", "坎特蕾拉", "夏空", "奥古斯塔", "守岸人", "安可",
  "尤诺", "布兰特", "弗洛洛", "忌炎", "折枝", "散华", "桃祈", "椿",
  "洛可可", "渊武", "漂泊者-女-气动", "漂泊者-女-湮灭", "漂泊者-女-衍射",
  "漂泊者-男-气动", "漂泊者-男-湮灭", "漂泊者-男-衍射", "灯灯", "炽霞",
  "爱弥斯", "珂莱塔", "琳奈", "白芷", "相里要", "秋水", "秧秧", "绯雪",
  "维里奈", "莫宁", "莫特斐", "菲比", "西格莉卡", "赞妮", "达妮娅",
  "釉瑚", "鉴心", "长离", "陆·赫斯", "露帕",
]);

const STANDARD_5_STAR_CHARACTERS: ReadonlySet<string> = new Set([
  "卡卡罗", "安可", "鉴心", "忌炎", "凌阳", "维里奈",
  "漂泊者-女-气动", "漂泊者-女-湮灭", "漂泊者-女-衍射",
  "漂泊者-男-气动", "漂泊者-男-湮灭", "漂泊者-男-衍射",
]);

const STANDARD_5_STAR_WEAPONS: ReadonlySet<string> = new Set([
  "苍鳞千嶂", "千古洑流", "停驻之烟", "擎渊怒涛", "漪澜浮录",
  "源能机锋", "镭射切变", "相位涟漪", "尘云旋臂", "玻色星仪",
]);

export function isCharacter(name: string): boolean {
  if (CHARACTER_NAMES.has(name)) return true;
  // API may return bare "漂泊者" or variant suffixes we don't have exact art for.
  return name.startsWith("漂泊者");
}

export function assetPath(name: string): string {
  const dir = isCharacter(name) ? "characters" : "weapons";
  return `/wiki-art/${dir}/${encodeURIComponent(name)}.png`;
}

export function isStandard5Star(name: string): boolean {
  return (
    STANDARD_5_STAR_CHARACTERS.has(name) ||
    STANDARD_5_STAR_WEAPONS.has(name) ||
    name.startsWith("漂泊者")
  );
}
