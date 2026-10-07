import {Box, Text, useWindowSize, type DOMElement} from 'ink';
import {createContext, useContext, useMemo, useRef, useState, type ReactNode} from 'react';
import {MouseLayer, MouseProvider, useMouseTarget} from '../input/Mouse.js';
import {ThemeProvider, useTheme} from '../theme/ThemeContext.js';

export type OverlayOptions = {
  /** Anchor the overlay at this position; without one it is centered. The screen below is always dimmed. */
  top?: number;
  left?: number;
};

export type OverlayHandle = {
  /** Shows one element on top of the screen; showing again replaces it. A click outside it hides it. */
  show: (element: ReactNode, options?: OverlayOptions) => void;
  hide: () => void;
  isOpen: boolean;
};

const OverlayContext = createContext<OverlayHandle>({show: () => {}, hide: () => {}, isOpen: false});

/** Access to the screen's overlay slot from any component below the Screen. */
export function useOverlay(): OverlayHandle {
  return useContext(OverlayContext);
}

/** minColumns/minRows: show a notice instead of the app below this size. Off (0) by default: the App degrades instead. */
type Props = {children: ReactNode; minColumns?: number; minRows?: number};

type Shown = {element: ReactNode; options: OverlayOptions};

/** The full-window root: fills the terminal with the background color, hosts the overlay slot and owns the mouse. */
export function Screen(props: Props) {
  return (
    <MouseProvider>
      <ScreenBody {...props} />
    </MouseProvider>
  );
}

function ScreenBody({children, minColumns = 0, minRows = 0}: Props) {
  const {columns, rows} = useWindowSize();
  const theme = useTheme();
  const [shown, setShown] = useState<Shown | null>(null);
  const handle = useMemo<OverlayHandle>(
    () => ({show: (element, options = {}) => setShown({element, options}), hide: () => setShown(null), isOpen: shown !== null}),
    [shown],
  );
  if (columns < minColumns || rows < minRows) return <TooSmall columns={columns} rows={rows} minColumns={minColumns} minRows={minRows} />;
  const below = shown ? theme.dimmed() : theme;
  return (
    <OverlayContext.Provider value={handle}>
      <Box width={columns} height={rows} backgroundColor={below.tokens.background} flexDirection="column">
        <ThemeProvider theme={below}>
          <Box flexDirection="column" flexGrow={1} flexShrink={1} minHeight={0} overflow="hidden">{children}</Box>
        </ThemeProvider>
        {shown && <OverlayLayer shown={shown} columns={columns} rows={rows} onBackdrop={() => setShown(null)} />}
      </Box>
    </OverlayContext.Provider>
  );
}

function TooSmall({columns, rows, minColumns, minRows}: {columns: number; rows: number; minColumns: number; minRows: number}) {
  const theme = useTheme();
  return (
    <Box width={columns} height={rows} backgroundColor={theme.tokens.background} alignItems="center" justifyContent="center">
      <Text color={theme.tokens.textMuted}>Terminal too small: {columns}×{rows}, need {minColumns}×{minRows}</Text>
    </Box>
  );
}

/** The backdrop (a click on it hides the overlay) and the overlay element, centered or anchored, on their own mouse layers. */
function OverlayLayer({shown, columns, rows, onBackdrop}: {shown: Shown; columns: number; rows: number; onBackdrop: () => void}) {
  const backdrop = useRef<DOMElement>(null);
  useMouseTarget(backdrop, {onPress: onBackdrop});
  const anchored = shown.options.top !== undefined;
  return (
    <>
      <MouseLayer layer={1}>
        <Box ref={backdrop} position="absolute" top={0} left={0} width={columns} height={rows} />
      </MouseLayer>
      <MouseLayer layer={2}>
        {anchored ? (
          <Box position="absolute" top={shown.options.top ?? 0} left={shown.options.left ?? 0}>{shown.element}</Box>
        ) : (
          <Box position="absolute" top={0} left={0} width={columns} height={rows} alignItems="center" justifyContent="center">{shown.element}</Box>
        )}
      </MouseLayer>
    </>
  );
}
