import { createSignal, onCleanup } from "solid-js";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import {
  EVENT_SNIFFER_PARAMS,
  fetchGachaRecords,
  readParamsFromLog,
  startSniffer,
  stopSniffer,
  type CapturedParams,
} from "../lib/commands";
import { CardPool } from "../lib/types";
import type { FetchParams } from "../lib/types";

const ALL_POOLS: CardPool[] = [
  CardPool.FeaturedResonatorConvene,
  CardPool.FeaturedWeaponConvene,
  CardPool.StandardResonatorConvene,
  CardPool.StandardWeaponConvene,
  CardPool.NoviceConvene,
  CardPool.BeginnerChoiceConvene,
  CardPool.GivebackCustomConvene,
];

const REQUIRED_FIELDS: (keyof FetchParams)[] = [
  "playerId",
  "serverId",
  "languageCode",
  "recordId",
];

const SNIFFER_TIMEOUT_MS = 180_000;
const GAME_DIR_KEY = "wuwa.gameDir";

function loadGameDir(): string | undefined {
  try {
    return localStorage.getItem(GAME_DIR_KEY) ?? undefined;
  } catch {
    return undefined;
  }
}

function saveGameDir(dir: string) {
  try {
    localStorage.setItem(GAME_DIR_KEY, dir);
  } catch {
    // ignore
  }
}

interface FetchFormProps {
  onSuccess: (playerId: string) => void | Promise<void>;
}

export default function FetchForm(props: FetchFormProps) {
  const [json, setJson] = createSignal("");
  const [loading, setLoading] = createSignal(false);
  const [sniffing, setSniffing] = createSignal(false);
  const [readingLog, setReadingLog] = createSignal(false);
  const [error, setError] = createSignal("");
  const [status, setStatus] = createSignal("");
  const [gameDir, setGameDir] = createSignal<string | undefined>(loadGameDir());
  let activeCleanup: (() => Promise<void>) | null = null;
  let cancelRequested = false;

  const busy = () => loading() || sniffing() || readingLog();

  function fillJson(p: {
    playerId: string;
    serverId: string;
    languageCode: string;
    recordId: string;
  }) {
    setJson(
      JSON.stringify(
        {
          playerId: p.playerId,
          serverId: p.serverId,
          languageCode: p.languageCode,
          recordId: p.recordId,
        },
        null,
        2,
      ),
    );
  }

  function parseParams(raw: string): FetchParams {
    let parsed: unknown;
    try {
      parsed = JSON.parse(raw);
    } catch {
      throw new Error("JSON 格式错误");
    }
    if (typeof parsed !== "object" || parsed === null) {
      throw new Error("JSON 格式错误");
    }
    const obj = parsed as Record<string, unknown>;
    const missing = REQUIRED_FIELDS.filter(
      (k) => typeof obj[k] !== "string" || (obj[k] as string).trim() === "",
    );
    if (missing.length > 0) {
      throw new Error(`缺少必要字段: ${missing.join(", ")}`);
    }
    const playerId = (obj.playerId as string).trim();
    if (!/^\d{9}$/.test(playerId)) {
      throw new Error("playerId 格式错误");
    }
    return {
      playerId,
      serverId: (obj.serverId as string).trim(),
      languageCode: (obj.languageCode as string).trim(),
      recordId: (obj.recordId as string).trim(),
    };
  }

  async function handleFetch() {
    setError("");
    let params: FetchParams;
    try {
      params = parseParams(json());
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
      return;
    }

    setLoading(true);
    try {
      await fetchGachaRecords(params, ALL_POOLS);
      await props.onSuccess(params.playerId);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
    }
  }

  async function handleAutoCaptureSniff() {
    if (activeCleanup) return;
    setError("");
    setStatus("");
    setSniffing(true);
    cancelRequested = false;

    let unlisten: UnlistenFn | undefined;
    let timer: number | undefined;
    let cleaning = false;
    const cleanup = async () => {
      if (cleaning) return;
      cleaning = true;
      if (unlisten) unlisten();
      if (timer !== undefined) clearTimeout(timer);
      try {
        await stopSniffer();
      } catch (e) {
        console.error("stopSniffer failed", e);
      }
      setSniffing(false);
      activeCleanup = null;
    };
    activeCleanup = cleanup;

    try {
      unlisten = await listen<CapturedParams>(EVENT_SNIFFER_PARAMS, (event) => {
        fillJson(event.payload);
        setStatus(`已捕获玩家 ${event.payload.playerId} 的参数（抓包）`);
        cleanup();
      });

      await startSniffer();

      if (cancelRequested) {
        await cleanup();
        return;
      }

      setStatus("代理已启动，请打开游戏 → 抽卡 → 历史记录（翻几页以触发请求）");
      timer = window.setTimeout(() => {
        setError("超时未捕获到请求，已停止代理");
        cleanup();
      }, SNIFFER_TIMEOUT_MS);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
      await cleanup();
    }
  }

  async function handleCancelSniff() {
    cancelRequested = true;
    const c = activeCleanup;
    if (!c) {
      setSniffing(false);
      return;
    }
    setStatus("正在停止代理…（macOS 可能再次弹出授权）");
    await c();
    setStatus("已取消监听");
  }

  async function pickGameDir(): Promise<string | undefined> {
    const picked = await openDialog({
      multiple: false,
      directory: true,
      title: "选择鸣潮游戏安装目录（包含 Client 子目录）",
    });
    if (!picked || typeof picked !== "string") return undefined;
    saveGameDir(picked);
    setGameDir(picked);
    return picked;
  }

  async function readFromLog(dir?: string) {
    setError("");
    setStatus("");
    setReadingLog(true);
    try {
      const p = await readParamsFromLog({ gameDir: dir });
      fillJson(p);
      setStatus(`已从日志读取玩家 ${p.playerId} 的参数（来源：${p.sourcePath}）`);
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      setError(msg);
    } finally {
      setReadingLog(false);
    }
  }

  async function handleReadLog() {
    let dir = gameDir();
    if (!dir) {
      dir = await pickGameDir();
      if (!dir) return;
    }
    await readFromLog(dir);
  }

  async function handlePickGameDir() {
    const dir = await pickGameDir();
    if (!dir) return;
    await readFromLog(dir);
  }

  onCleanup(() => {
    if (activeCleanup) {
      activeCleanup().catch(() => undefined);
    }
  });

  return (
    <div class="fetch-form">
      <textarea
        class="fetch-form-input"
        placeholder='粘贴 JSON，例如 {"playerId":"123456789","serverId":"...","languageCode":"zh-Hans","recordId":"..."}'
        value={json()}
        onInput={(e) => setJson(e.currentTarget.value)}
        rows={6}
        disabled={busy()}
      />
      {gameDir() && (
        <p class="fetch-form-hint">已记住游戏目录：{gameDir()}</p>
      )}
      {status() && <p class="fetch-form-status">{status()}</p>}
      {error() && <p class="fetch-form-error">{error()}</p>}
      <div class="fetch-form-actions">
        <button
          class="btn btn-secondary"
          onClick={handleReadLog}
          disabled={busy()}
          title="从游戏日志提取抽卡参数"
        >
          {readingLog() ? "读取中…" : "从日志获取"}
        </button>
        <button
          class="btn btn-secondary"
          onClick={handlePickGameDir}
          disabled={busy()}
          title="选择鸣潮游戏安装目录"
        >
          选择游戏目录…
        </button>
        <button
          class="btn btn-secondary"
          onClick={sniffing() ? handleCancelSniff : handleAutoCaptureSniff}
          disabled={loading() || readingLog()}
          title={
            sniffing()
              ? "停止抓包并还原系统代理"
              : "启动本地 MITM 代理抓取游戏请求（需授权证书）"
          }
        >
          {sniffing() ? "取消监听" : "抓包获取"}
        </button>
        <button
          class="btn btn-primary"
          onClick={handleFetch}
          disabled={busy() || json().trim() === ""}
        >
          {loading() ? "获取中..." : "获取记录"}
        </button>
      </div>
    </div>
  );
}
