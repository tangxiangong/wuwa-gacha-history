import { invoke } from "@tauri-apps/api/core";
import type { CardPool, FetchParams, GachaFilter, GachaRecord } from "./types";

export async function queryGachaRecords(
  userId: string,
  filter: GachaFilter,
): Promise<GachaRecord[]> {
  return invoke("query_gacha_records", { userId, filter });
}

export async function fetchGachaRecords(
  params: FetchParams,
  poolTypes: CardPool[],
): Promise<number> {
  return invoke("fetch_gacha_records", { params, poolTypes });
}

export async function exportGachaRecords(
  userId: string,
  filter: GachaFilter,
  path: string,
): Promise<void> {
  return invoke("export_gacha_records", { userId, filter, path });
}
