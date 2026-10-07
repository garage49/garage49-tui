import {Box, type DOMElement} from 'ink';
import {useRef} from 'react';
import {FocusRegion} from '../components/FocusRegion.js';
import {TopNav, type NavItem} from '../components/TopNav.js';
import {useKeys} from '../input/Keys.js';
import {useMouseTarget} from '../input/Mouse.js';
import {RegionProvider, useFocusRegion} from './FocusRegistry.js';
import {useOverlayState} from './Overlay.js';

type Props = {
  brand: string;
  items: readonly NavItem[];
  activeId: string;
  onChange: (item: NavItem) => void;
  right?: string;
};

/** The App's top navigation: a focus region whose ←→ (hl) switch the active item. Enter goes down to the next region. */
export function Nav({brand, items, activeId, onChange, right}: Props) {
  const box = useRef<DOMElement>(null);
  const {focused, focus} = useFocusRegion(box, {descendOnEnter: true});
  const overlayOpen = useOverlayState();
  const active = focused && !overlayOpen;
  useMouseTarget(box, {onPress: focus});
  const index = items.findIndex(item => item.id === activeId);
  useKeys([
    {keys: ['left', 'h'], run: () => onChange(items[(index + items.length - 1) % items.length]!)},
    {keys: ['right', 'l'], run: () => onChange(items[(index + 1) % items.length]!)},
  ], {isActive: active});
  return (
    <RegionProvider focused={active}>
      <FocusRegion focused={active} surface="heading" rows={1}>
        <Box ref={box} flexDirection="column">
          <TopNav brand={brand} items={items} activeId={activeId} focused={active} onChange={item => { focus(); onChange(item); }} right={right} />
        </Box>
      </FocusRegion>
    </RegionProvider>
  );
}
