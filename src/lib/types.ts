export enum CardPool {
  FeaturedResonatorConvene = 1,
  FeaturedWeaponConvene = 2,
  StandardResonatorConvene = 3,
  StandardWeaponConvene = 4,
  NoviceConvene = 5,
  BeginnerChoiceConvene = 6,
  GivebackCustomConvene = 7,
}

export const CARD_POOL_LABELS: Record<CardPool, string> = {
  [CardPool.FeaturedResonatorConvene]: "限定角色",
  [CardPool.FeaturedWeaponConvene]: "限定武器",
  [CardPool.StandardResonatorConvene]: "常驻角色",
  [CardPool.StandardWeaponConvene]: "常驻武器",
  [CardPool.NoviceConvene]: "新手唤取",
  [CardPool.BeginnerChoiceConvene]: "新手自选",
  [CardPool.GivebackCustomConvene]: "感恩自选",
};

export enum QualityLevel {
  ThreeStar = 3,
  FourStar = 4,
  FiveStar = 5,
}

export interface GachaRecord {
  id: number;
  serverId: string;
  cardPool: CardPool;
  languageCode: string;
  recordId: string;
  qualityLevel: QualityLevel;
  name: string;
  time: string;
  /** WuWa version whose window contains `time` (e.g. "2.4"), "pre" before 1.0. */
  version: string;
}

export interface GachaFilter {
  cardPool?: CardPool | null;
  qualityLevel?: QualityLevel | null;
  name?: string | null;
  timeFrom?: string | null;
  timeTo?: string | null;
  limit?: number | null;
  offset?: number | null;
}

export interface FetchParams {
  playerId: string;
  serverId: string;
  languageCode: string;
  recordId: string;
}
