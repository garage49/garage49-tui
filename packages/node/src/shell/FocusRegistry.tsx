import {measureElement, type DOMElement} from 'ink';
import {createContext, useContext, useEffect, useRef, useState, type ReactNode, type RefObject} from 'react';

export type RegionOptions = {
  /** Enter moves the focus to the next region (navigation, sidebar). False where enter means "activate" (main). */
  descendOnEnter?: boolean;
};

type Region = {readonly id: number; readonly ref: RefObject<DOMElement | null>; readonly options: RegionOptions};

/**
 * Keeps the focusable regions of an App and which one owns the keyboard. The order of regions is the
 * order on screen (top to bottom, then left to right), measured when the focus moves, so a region
 * that appears or disappears (a conditional sidebar) slots in where it is drawn.
 */
export class FocusRegistry {
  private regions: Region[] = [];
  private nextId = 0;

  register(ref: RefObject<DOMElement | null>, options: RegionOptions): Region {
    const region: Region = {id: this.nextId++, ref, options};
    this.regions.push(region);
    return region;
  }

  unregister(region: Region): void {
    this.regions = this.regions.filter(candidate => candidate !== region);
  }

  /** Regions in screen order. */
  ordered(): Region[] {
    const placed = this.regions.flatMap(region => {
      if (!region.ref.current) return [];
      const rect = measureElement(region.ref.current);
      return [{region, y: rect.y, x: rect.x}];
    });
    return placed.sort((a, b) => a.y - b.y || a.x - b.x).map(placed => placed.region);
  }

  /** The region `delta` steps away in screen order. Wrapping around is for tab; esc and enter stop at the ends. */
  neighbour(currentId: number | undefined, delta: number, wrap: boolean): Region | undefined {
    const ordered = this.ordered();
    if (ordered.length === 0) return undefined;
    const index = Math.max(0, ordered.findIndex(region => region.id === currentId));
    const next = index + delta;
    if (wrap) return ordered[(next + ordered.length) % ordered.length];
    return ordered[Math.min(ordered.length - 1, Math.max(0, next))];
  }

  first(): Region | undefined {
    return this.ordered()[0];
  }

  find(id: number | undefined): Region | undefined {
    return this.regions.find(region => region.id === id);
  }
}

type FocusState = {
  registry: FocusRegistry;
  focusedId: number | undefined;
  setFocusedId: (update: number | undefined | ((current: number | undefined) => number | undefined)) => void;
};

const FocusContext = createContext<FocusState | null>(null);

export function FocusProvider({children}: {children: ReactNode}) {
  const registry = useRef(new FocusRegistry()).current;
  const [focusedId, setFocusedId] = useState<number | undefined>();
  return <FocusContext.Provider value={{registry, focusedId, setFocusedId}}>{children}</FocusContext.Provider>;
}

/** The App's focus state: who is focused, and moves between regions. */
export function useFocusState(): FocusState {
  const state = useContext(FocusContext);
  if (!state) throw new Error('useFocusState: no <App> above');
  return state;
}

/** Registers the box behind `ref` as a focusable region for as long as the component is mounted. Returns whether it is focused, and a focus() call. */
export function useFocusRegion(ref: RefObject<DOMElement | null>, options: RegionOptions = {}): {focused: boolean; focus: () => void} {
  const {registry, focusedId, setFocusedId} = useFocusState();
  const [regionId, setRegionId] = useState<number | undefined>();
  useEffect(() => {
    const region = registry.register(ref, options);
    setRegionId(region.id);
    // The first region to appear takes the focus.
    setFocusedId(current => (current === undefined ? region.id : current));
    return () => {
      registry.unregister(region);
      setFocusedId(current => (current === region.id ? registry.first()?.id : current));
    };
  }, [registry, ref]); // options are read once, at registration
  return {focused: regionId !== undefined && regionId === focusedId, focus: () => regionId !== undefined && setFocusedId(regionId)};
}

const RegionContext = createContext(false);

/** Marks the subtree as belonging to a region with the given focus. */
export function RegionProvider({focused, children}: {focused: boolean; children: ReactNode}) {
  return <RegionContext.Provider value={focused}>{children}</RegionContext.Provider>;
}

/** Whether the nearest enclosing region (Nav, Sidebar, Main) owns the keyboard. */
export function useFocused(): boolean {
  return useContext(RegionContext);
}
