import type {ListItem} from './List.js';

/** One rendered row of a List: a section heading, a blank gap between sections, or an item. */
export type ListRow =
  | {readonly kind: 'section'; readonly key: string; readonly title: string}
  | {readonly kind: 'gap'; readonly key: string}
  | {readonly kind: 'item'; readonly key: string; readonly item: ListItem};

/** Turns items with optional sections into the rows a List draws, in order. */
export class ListLayout {
  static rows(items: readonly ListItem[]): ListRow[] {
    const rows: ListRow[] = [];
    let section: string | undefined;
    for (const item of items) {
      if (item.section !== section) {
        section = item.section;
        if (rows.length > 0) rows.push({kind: 'gap', key: `gap-${item.id}`});
        if (section !== undefined) rows.push({kind: 'section', key: `section-${item.id}`, title: section});
      }
      rows.push({kind: 'item', key: item.id, item});
    }
    return rows;
  }

  /** The item drawn on the given row, if any (for mouse hit-testing). */
  static itemAt(rows: readonly ListRow[], index: number): ListItem | undefined {
    const row = rows[index];
    return row?.kind === 'item' ? row.item : undefined;
  }
}
