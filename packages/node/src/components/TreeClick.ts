/** What a mouse click on a tree row means, by the usual file-manager convention. */
export type TreeClickAction = 'toggle' | 'select';

/**
 * A click on the ▸/▾ marker toggles the branch; a click on the row that is already selected
 * toggles it too; a click on any other row only selects it.
 */
export class TreeClick {
  static decide(onMarker: boolean, alreadySelected: boolean): TreeClickAction {
    return onMarker || alreadySelected ? 'toggle' : 'select';
  }

  /** Whether column `x` of a row at `depth` is the marker cell (after the row padding and the indent). */
  static isMarker(x: number, depth: number): boolean {
    return x === 1 + depth * 2;
  }
}
