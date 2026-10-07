import {Box, type DOMElement} from 'ink';
import {useRef, type ReactNode} from 'react';
import {FocusRegion} from '../components/FocusRegion.js';
import {Label} from '../components/Label.js';
import {useMouseTarget} from '../input/Mouse.js';
import {RegionProvider, useFocusRegion} from './FocusRegistry.js';
import {useOverlayState} from './Overlay.js';

type Props = {title?: string; children: ReactNode};

/** The App's page area: a focus region that takes the remaining width. Enter is the page's own key here. */
export function Main({title, children}: Props) {
  const box = useRef<DOMElement>(null);
  const {focused, focus} = useFocusRegion(box, {descendOnEnter: false});
  const overlayOpen = useOverlayState();
  const active = focused && !overlayOpen;
  useMouseTarget(box, {onPress: focus});
  return (
    <RegionProvider focused={active}>
      <FocusRegion focused={active} grow>
        <Box ref={box} flexDirection="column" flexGrow={1} paddingX={2} paddingY={1} overflow="hidden">
          {title && (
            <>
              <Label variant="bright">{title}</Label>
              <Box height={1} />
            </>
          )}
          <Box flexDirection="column" flexGrow={1}>{children}</Box>
        </Box>
      </FocusRegion>
    </RegionProvider>
  );
}
