import {Box} from 'ink';
import {useKeys} from '../input/Keys.js';
import {Button} from './Button.js';
import {Label} from './Label.js';
import {Overlay} from './Overlay.js';

type Props = {title: string; message: string; variant?: 'default' | 'error'; onClose: () => void};

/** A one-button dialog for information and errors. */
export function MessageDialog({title, message, variant = 'default', onClose}: Props) {
  useKeys([{keys: ['esc', 'enter'], run: onClose}]);
  return (
    <Overlay title={title} width={50} variant={variant}>
      <Box paddingX={1}><Label>{message}</Label></Box>
      <Box height={1} />
      <Box flexDirection="row" justifyContent="flex-end">
        <Button label="OK" selected onPress={onClose} />
      </Box>
    </Overlay>
  );
}
