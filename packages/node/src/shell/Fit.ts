/** What the App keeps at a given terminal size: it degrades instead of refusing to draw. */
export type Fit = {
  /** Content shows the Main page; false keeps only the Sidebar (narrow terminal with a sidebar). */
  readonly main: boolean;
  readonly statusLine: boolean;
  readonly keyHints: boolean;
};

/**
 * Decides what fits. Below 64 columns a sidebar-and-main layout keeps the sidebar alone (the
 * navigation stays readable; a mirror pane may own the rest of the screen); below 12 rows the key
 * hint bar goes, below 8 rows the status line too. Nothing is ever refused.
 */
export class ScreenFit {
  static readonly narrowColumns = 64;
  static readonly shortRows = 12;
  static readonly tinyRows = 8;

  static decide(columns: number, rows: number, hasSidebar: boolean): Fit {
    return {
      main: !hasSidebar || columns >= ScreenFit.narrowColumns,
      statusLine: rows >= ScreenFit.tinyRows,
      keyHints: rows >= ScreenFit.shortRows,
    };
  }
}
