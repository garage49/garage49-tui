import {type DOMElement} from 'ink';
import {useRef} from 'react';
import {FocusRegion} from '../components/FocusRegion.js';
import {List, type ListItem} from '../components/List.js';
import {Panel} from '../components/Panel.js';
import {useKeys} from '../input/Keys.js';
import {useMouseTarget} from '../input/Mouse.js';
import {Selection} from '../components/Selection.js';
import {RegionProvider, useFocusRegion} from './FocusRegistry.js';
import {useOverlayState} from './Overlay.js';

type Props = {
  items: readonly ListItem[];
  selectedId: string | undefined;
  onSelect: (item: ListItem) => void;
  width?: number;
};

/** The App's left panel: a sectioned list that is a focus region. ↑↓ (jk) move the selection; enter goes down to the next region. */
export function Sidebar({items, selectedId, onSelect, width = 26}: Props) {
  const box = useRef<DOMElement>(null);
  const {focused, focus} = useFocusRegion(box, {descendOnEnter: true});
  const overlayOpen = useOverlayState();
  const active = focused && !overlayOpen;
  useMouseTarget(box, {onPress: focus});
  // Keys faster than renders chain on `pending`, not on the selection captured at the last render.
  const pending = useRef(selectedId);
  pending.current = selectedId;
  const move = (delta: number) => {
    const next = items.find(item => item.id === Selection.move(items, pending.current, delta));
    if (next) {
      pending.current = next.id;
      onSelect(next);
    }
  };
  useKeys([
    {keys: ['up', 'k'], run: () => move(-1)},
    {keys: ['down', 'j'], run: () => move(1)},
  ], {isActive: active});
  return (
    <RegionProvider focused={active}>
      <FocusRegion focused={active} surface="panel">
        <Panel ref={box} width={width} flexShrink={0} flexGrow={1}>
          <List items={items} selectedId={selectedId} focused={focused} onClick={onSelect} onSelect={onSelect} />
        </Panel>
      </FocusRegion>
    </RegionProvider>
  );
}
