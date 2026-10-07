import {measureElement, useStdin, useStdout, type DOMElement} from 'ink';
import {createContext, useContext, useEffect, useRef, useState, type ReactNode, type RefObject} from 'react';
import {MouseParser, type MouseEvent} from './MouseParser.js';

/** A mouse event translated into a target's own coordinates. */
export type LocalMouseEvent = MouseEvent & {readonly localX: number; readonly localY: number};

export type MouseHandlers = {
  onPress?: (event: LocalMouseEvent) => void;
  onWheel?: (event: LocalMouseEvent) => void;
};

type Target = {
  readonly id: number;
  readonly layer: number;
  readonly ref: RefObject<DOMElement | null>;
  readonly handlers: RefObject<MouseHandlers>;
};

/**
 * Keeps every mouse target and routes an event to the targets under the pointer.
 * Only the topmost layer that is hit receives the event; inside it the event bubbles from the
 * innermost (smallest) target outwards, so a row click can select the row and focus its region.
 */
export class MouseRouter {
  private targets: Target[] = [];
  private observers: Array<{ref: RefObject<DOMElement | null>; onPressOutside: RefObject<() => void>}> = [];
  private nextId = 0;

  register(layer: number, ref: RefObject<DOMElement | null>, handlers: RefObject<MouseHandlers>): () => void {
    const target: Target = {id: this.nextId++, layer, ref, handlers};
    this.targets.push(target);
    return () => {
      this.targets = this.targets.filter(candidate => candidate !== target);
    };
  }

  /** Calls `onPressOutside` for every press that lands outside the box behind `ref` (an editing field ends on it). */
  observe(ref: RefObject<DOMElement | null>, onPressOutside: RefObject<() => void>): () => void {
    const observer = {ref, onPressOutside};
    this.observers.push(observer);
    return () => {
      this.observers = this.observers.filter(candidate => candidate !== observer);
    };
  }

  private notifyOutside(event: MouseEvent): void {
    for (const observer of this.observers) {
      if (observer.ref.current && !hits(observer.ref, event)) observer.onPressOutside.current();
    }
  }

  dispatch(event: MouseEvent): void {
    if (event.kind !== 'press' && event.kind !== 'wheel-up' && event.kind !== 'wheel-down') return;
    if (event.kind === 'press') this.notifyOutside(event);
    const under = this.targets.flatMap(target => {
      if (!hits(target.ref, event)) return [];
      const rect = measureElement(target.ref.current!);
      return [{target, rect, area: rect.width * rect.height}];
    });
    if (under.length === 0) return;
    const topLayer = Math.max(...under.map(hit => hit.target.layer));
    const ordered = under.filter(hit => hit.target.layer === topLayer).sort((a, b) => a.area - b.area || b.target.id - a.target.id);
    for (const {target, rect} of ordered) {
      const local = {...event, localX: event.x - rect.x, localY: event.y - rect.y};
      const handler = event.kind === 'press' ? target.handlers.current.onPress : target.handlers.current.onWheel;
      handler?.(local);
    }
  }
}

const RouterContext = createContext<MouseRouter | null>(null);

/** A press lies inside the box behind `ref`. */
function hits(ref: RefObject<DOMElement | null>, event: MouseEvent): boolean {
  if (!ref.current) return false;
  const rect = measureElement(ref.current);
  return event.x >= rect.x && event.x < rect.x + rect.width && event.y >= rect.y && event.y < rect.y + rect.height;
}
const LayerContext = createContext(0);

export type MouseSwitch = {readonly enabled: boolean; setEnabled: (enabled: boolean) => void};
const SwitchContext = createContext<MouseSwitch>({enabled: false, setEnabled: () => {}});

/** Whether mouse reporting is on, and the switch. Off, the terminal's own drag-to-select works again. */
export function useMouseSwitch(): MouseSwitch {
  return useContext(SwitchContext);
}

const ENABLE = '\x1b[?1000h\x1b[?1006h';
const DISABLE = '\x1b[?1000l\x1b[?1006l';

/**
 * Turns SGR mouse reporting on (while enabled) and feeds reports to the router. Reporting takes the
 * terminal's drag-to-select away, so apps expose the switch; shift+drag selects in most terminals anyway.
 */
export function MouseProvider({children, initiallyEnabled = true}: {children: ReactNode; initiallyEnabled?: boolean}) {
  const {stdin} = useStdin();
  const {stdout} = useStdout();
  const router = useRef(new MouseRouter()).current;
  const [enabled, setEnabled] = useState(initiallyEnabled);
  useEffect(() => {
    if (!enabled) return;
    stdout.write(ENABLE);
    const restore = () => stdout.write(DISABLE);
    process.on('exit', restore);
    const onData = (chunk: Buffer | string) => {
      for (const event of MouseParser.parse(chunk.toString())) router.dispatch(event);
    };
    stdin.on('data', onData);
    return () => {
      stdin.off('data', onData);
      process.off('exit', restore);
      restore();
    };
  }, [stdin, stdout, router, enabled]);
  return (
    <SwitchContext.Provider value={{enabled, setEnabled}}>
      <RouterContext.Provider value={router}>{children}</RouterContext.Provider>
    </SwitchContext.Provider>
  );
}

/** Puts its children on a higher mouse layer: an overlay's targets win over the screen below. */
export function MouseLayer({layer, children}: {layer: number; children: ReactNode}) {
  return <LayerContext.Provider value={layer}>{children}</LayerContext.Provider>;
}

/** Calls `onPressOutside` for every press outside the box behind `ref` for as long as the component is mounted. */
export function useMouseOutside(ref: RefObject<DOMElement | null>, onPressOutside: () => void): void {
  const router = useContext(RouterContext);
  const latest = useRef(onPressOutside);
  latest.current = onPressOutside;
  useEffect(() => router?.observe(ref, latest), [router, ref]);
}

/** Registers the box behind `ref` as a mouse target for as long as the component is mounted. */
export function useMouseTarget(ref: RefObject<DOMElement | null>, handlers: MouseHandlers): void {
  const router = useContext(RouterContext);
  const layer = useContext(LayerContext);
  const latest = useRef(handlers);
  latest.current = handlers;
  useEffect(() => router?.register(layer, ref, latest), [router, layer, ref]);
}
