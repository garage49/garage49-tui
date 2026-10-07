import {Box, Text} from 'ink';
import {useKeys} from '../input/Keys.js';
import {TextWidth} from '../theme/TextWidth.js';
import {useTheme} from '../theme/ThemeContext.js';
import {Overlay} from './Overlay.js';

export type HelpEntry = {readonly keys: string; readonly action: string};

type Props = {entries: readonly HelpEntry[]; onClose: () => void};

/** The `?` help overlay: one line per key binding. */
export function HelpOverlay({entries, onClose}: Props) {
  const theme = useTheme();
  useKeys([{keys: ['esc', '?', 'q'], run: onClose}]);
  const keyWidth = TextWidth.widest(entries.map(entry => entry.keys)) + 2;
  return (
    <Overlay title="Help">
      <Box flexDirection="column">
        {entries.map(entry => (
          <Box key={entry.keys} flexDirection="row">
            <Box width={keyWidth} flexShrink={0}><Text color={theme.tokens.text}>{entry.keys}</Text></Box>
            <Text color={theme.tokens.textMuted}>{entry.action}</Text>
          </Box>
        ))}
      </Box>
    </Overlay>
  );
}
