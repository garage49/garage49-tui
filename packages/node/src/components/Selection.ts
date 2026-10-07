import type {ListItem} from '../components/List.js';

/** Cursor movement over a list of items, clamped at both ends. */
export class Selection {
  static move(items: readonly ListItem[], selectedId: string | undefined, delta: number): string | undefined {
    if (items.length === 0) return undefined;
    const index = items.findIndex(item => item.id === selectedId);
    const next = Math.min(items.length - 1, Math.max(0, (index < 0 ? 0 : index) + delta));
    return items[next]!.id;
  }

  static ensure(items: readonly ListItem[], selectedId: string | undefined): string | undefined {
    return items.some(item => item.id === selectedId) ? selectedId : items[0]?.id;
  }
}
