import { invoke } from "@tauri-apps/api/core";
import type { CardPool, FetchParams, GachaFilter, GachaRecord } from "./types";

export async function queryGachaRecords(
  playerId: string,
  filter: GachaFilter,
): Promise<GachaRecord[]> {
  return invoke("query_gacha_records", { playerId, filter });
}

export async function fetchGachaRecords(
  params: FetchParams,
  poolTypes: CardPool[],
): Promise<number> {
  return invoke("fetch_gacha_records", { params, poolTypes });
}

export async function exportGachaRecords(
  playerId: string,
  filter: GachaFilter,
  path: string,
): Promise<void> {
  return invoke("export_gacha_records", { playerId, filter, path });
}

export async function listUsers(): Promise<string[]> {
  return invoke("list_users");
}
