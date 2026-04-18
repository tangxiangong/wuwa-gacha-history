import { createSignal, createResource, Show } from "solid-js";
import type { CardPool, GachaFilter } from "./lib/types";
import { listUsers } from "./lib/commands";
import Sidebar from "./components/Sidebar";
import ContentArea from "./components/ContentArea";
import ExportDialog from "./components/ExportDialog";
import WelcomePage from "./components/WelcomePage";
import AddUserDialog from "./components/AddUserDialog";
import "./App.css";

function App() {
  const [activePool, setActivePool] = createSignal<CardPool | null>(null);
  const [exportOpen, setExportOpen] = createSignal(false);
  const [addUserOpen, setAddUserOpen] = createSignal(false);
  const [playerId, setPlayerId] = createSignal<string | null>(null);

  const [users, { refetch: refetchUsers }] = createResource(async () => {
    const list = await listUsers();
    if (list.length > 0 && playerId() === null) {
      setPlayerId(list[0]);
    }
    return list;
  });

  async function handleUserAdded(newPlayerId: string) {
    await refetchUsers();
    setPlayerId(newPlayerId);
  }

  const exportFilter = (): GachaFilter => ({
    cardPool: activePool(),
  });

  return (
    <div class="app">
      <Show
        when={!users.error}
        fallback={
          <div class="load-error">
            <h2>无法加载用户列表</h2>
            <p>{String(users.error)}</p>
            <button class="btn btn-primary" onClick={() => refetchUsers()}>
              重试
            </button>
          </div>
        }
      >
      <Show
        when={(users() ?? []).length > 0 && playerId() !== null}
        fallback={
          <Show when={!users.loading}>
            <WelcomePage onUserAdded={handleUserAdded} />
          </Show>
        }
      >
        <Sidebar
          users={users() ?? []}
          playerId={playerId()}
          activePool={activePool()}
          onSelectUser={setPlayerId}
          onSelectPool={setActivePool}
          onAddUser={() => setAddUserOpen(true)}
          onExport={() => setExportOpen(true)}
        />
        <ContentArea activePool={activePool()} playerId={playerId()!} />
        <ExportDialog
          open={exportOpen()}
          playerId={playerId()!}
          filter={exportFilter()}
          onClose={() => setExportOpen(false)}
        />
        <AddUserDialog
          open={addUserOpen()}
          onClose={() => setAddUserOpen(false)}
          onUserAdded={handleUserAdded}
        />
      </Show>
      </Show>
    </div>
  );
}

export default App;
