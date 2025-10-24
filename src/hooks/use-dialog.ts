import { useContext } from 'react';
import { DialogContext } from '../components/global-dialog';

// 这些hooks放在独立文件中以避免Fast Refresh问题

export function useGlobalDialog() {
  const context = useContext(DialogContext);
  if (!context) {
    throw new Error(
      "useGlobalDialog must be used within a GlobalDialogProvider"
    );
  }
  return context;
}

export function useConfirm() {
  const { showConfirm } = useGlobalDialog();
  return showConfirm;
}

export function useAlert() {
  const { showAlert } = useGlobalDialog();
  return showAlert;
}

export function useDestructiveConfirm() {
  const { showDestructive } = useGlobalDialog();
  return showDestructive;
}