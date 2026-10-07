import {Box} from 'ink';
import {useState} from 'react';
import {useKeys} from '../input/Keys.js';
import {Button} from './Button.js';
import {Label} from './Label.js';
import {Overlay} from './Overlay.js';

type Props = {
  title: string;
  message: string;
  confirmLabel?: string;
  cancelLabel?: string;
  /** Marks the confirming action as destructive: the dialog title turns to the error color. */
  danger?: boolean;
  onConfirm: () => void;
  onCancel: () => void;
};

/** A yes/no dialog: message, then two buttons; the chosen button is filled with the accent color. */
export function ConfirmDialog({title, message, confirmLabel = 'OK', cancelLabel = 'Cancel', danger = false, onConfirm, onCancel}: Props) {
  const [confirmSelected, setConfirmSelected] = useState(false);
  useKeys([
    {keys: ['esc', 'n'], run: onCancel},
    {keys: ['y'], run: onConfirm},
    {keys: ['left', 'right', 'tab'], run: () => setConfirmSelected(!confirmSelected)},
    {keys: ['enter'], run: () => (confirmSelected ? onConfirm() : onCancel())},
  ]);
  return (
    <Overlay title={title} width={50} variant={danger ? 'error' : 'default'}>
      <Box paddingX={1}><Label>{message}</Label></Box>
      <Box height={1} />
      <Box flexDirection="row" justifyContent="flex-end">
        <Button label={cancelLabel} selected={!confirmSelected} onPress={onCancel} />
        <Button label={confirmLabel} selected={confirmSelected} onPress={onConfirm} />
      </Box>
    </Overlay>
  );
}
