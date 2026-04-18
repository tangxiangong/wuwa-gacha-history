import { createSignal } from "solid-js";
import { fetchGachaRecords } from "../lib/commands";
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

interface FetchFormProps {
  onSuccess: (playerId: string) => void;
}

export default function FetchForm(props: FetchFormProps) {
  const [json, setJson] = createSignal("");
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal("");

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
      props.onSuccess(params.playerId);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
    }
  }

  return (
    <div class="fetch-form">
      <textarea
        class="fetch-form-input"
        placeholder='粘贴 JSON，例如 {"playerId":"123456789","serverId":"...","languageCode":"zh-Hans","recordId":"..."}'
        value={json()}
        onInput={(e) => setJson(e.currentTarget.value)}
        rows={6}
        disabled={loading()}
      />
      {error() && <p class="fetch-form-error">{error()}</p>}
      <button
        class="btn btn-primary"
        onClick={handleFetch}
        disabled={loading() || json().trim() === ""}
      >
        {loading() ? "获取中..." : "获取记录"}
      </button>
    </div>
  );
}
