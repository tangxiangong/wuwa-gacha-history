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

export async function startSniffer(): Promise<number> {
  return invoke("start_sniffer");
}

export async function stopSniffer(): Promise<void> {
  return invoke("stop_sniffer");
}

export const EVENT_SNIFFER_PARAMS = "sniffer://params-captured";
export const EVENT_SNIFFER_STATUS = "sniffer://status";

export interface CapturedParams {
  playerId: string;
  serverId: string;
  languageCode: string;
  recordId: string;
  cardPoolId: string;
}

export interface LogParams {
  playerId: string;
  serverId: string;
  languageCode: string;
  recordId: string;
  cardPoolId: string | null;
  sourcePath: string;
  sourceUrl: string;
}

export async function readParamsFromLog(opts: {
  path?: string;
  gameDir?: string;
} = {}): Promise<LogParams> {
  return invoke("read_params_from_log", {
    path: opts.path,
    gameDir: opts.gameDir,
  });
}
