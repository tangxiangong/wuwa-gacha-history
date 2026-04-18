import { Show } from "solid-js";
import FetchForm from "./FetchForm";

interface AddUserDialogProps {
  open: boolean;
  onClose: () => void;
  onUserAdded: (playerId: string) => void;
}

export default function AddUserDialog(props: AddUserDialogProps) {
  function handleSuccess(playerId: string) {
    props.onUserAdded(playerId);
    props.onClose();
  }

  return (
    <Show when={props.open}>
      <div class="dialog-overlay" onClick={() => props.onClose()}>
        <div class="dialog" onClick={(e) => e.stopPropagation()}>
          <h3>添加用户</h3>
          <FetchForm onSuccess={handleSuccess} />
          <div class="dialog-actions">
            <button class="btn" onClick={() => props.onClose()}>
              取消
            </button>
          </div>
        </div>
      </div>
    </Show>
  );
}
