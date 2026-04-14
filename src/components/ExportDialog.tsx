import { createSignal, Show } from "solid-js";
import { save } from "@tauri-apps/plugin-dialog";
import { exportGachaRecords } from "../lib/commands";
import type { GachaFilter } from "../lib/types";

interface ExportDialogProps {
  open: boolean;
  userId: string;
  filter: GachaFilter;
  onClose: () => void;
}

type ExportFormat = "csv" | "xlsx" | "json";

const FORMAT_EXTENSIONS: Record<ExportFormat, string> = {
  csv: "csv",
  xlsx: "xlsx",
  json: "json",
};

const FORMAT_LABELS: Record<ExportFormat, string> = {
  csv: "CSV",
  xlsx: "Excel",
  json: "JSON",
};

export default function ExportDialog(props: ExportDialogProps) {
  const [format, setFormat] = createSignal<ExportFormat>("xlsx");
  const [exporting, setExporting] = createSignal(false);
  const [error, setError] = createSignal("");

  async function handleExport() {
    setError("");
    const ext = FORMAT_EXTENSIONS[format()];
    const filePath = await save({
      defaultPath: `gacha-history.${ext}`,
      filters: [{ name: FORMAT_LABELS[format()], extensions: [ext] }],
    });

    if (!filePath) return;

    setExporting(true);
    try {
      await exportGachaRecords(props.userId, props.filter, filePath);
      props.onClose();
    } catch (e) {
      setError(String(e));
    } finally {
      setExporting(false);
    }
  }

  return (
    <Show when={props.open}>
      <div class="dialog-overlay" onClick={() => props.onClose()}>
        <div class="dialog" onClick={(e) => e.stopPropagation()}>
          <h3>导出记录</h3>
          <div class="format-options">
            {(["csv", "xlsx", "json"] as ExportFormat[]).map((fmt) => (
              <button
                class={`format-option ${format() === fmt ? "active" : ""}`}
                onClick={() => setFormat(fmt)}
              >
                {FORMAT_LABELS[fmt]}
              </button>
            ))}
          </div>
          <Show when={error()}>
            <p style={{ color: "var(--star-5)", "font-size": "12px", "margin-top": "8px" }}>
              {error()}
            </p>
          </Show>
          <div class="dialog-actions">
            <button class="btn" onClick={() => props.onClose()}>
              取消
            </button>
            <button class="btn btn-primary" onClick={handleExport} disabled={exporting()}>
              {exporting() ? "导出中..." : "导出"}
            </button>
          </div>
        </div>
      </div>
    </Show>
  );
}
