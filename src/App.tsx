import { createSignal } from "solid-js";
import type { CardPool, GachaFilter } from "./lib/types";
import Sidebar from "./components/Sidebar";
import ContentArea from "./components/ContentArea";
import ExportDialog from "./components/ExportDialog";
import "./App.css";

function App() {
  const [activePool, setActivePool] = createSignal<CardPool | null>(null);
  const [exportOpen, setExportOpen] = createSignal(false);

  // TODO: This should come from a settings view in the future.
  // For now, hardcoded or empty — records won't load without a valid userId.
  const userId = () => "";

  const exportFilter = (): GachaFilter => ({
    cardPool: activePool(),
  });

  return (
    <div class="app">
      <Sidebar
        activePool={activePool()}
        onSelectPool={setActivePool}
        onExport={() => setExportOpen(true)}
      />
      <ContentArea activePool={activePool()} userId={userId()} />
      <ExportDialog
        open={exportOpen()}
        userId={userId()}
        filter={exportFilter()}
        onClose={() => setExportOpen(false)}
      />
    </div>
  );
}

export default App;
