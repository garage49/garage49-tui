/** Props every gallery page receives. */
export type PageProps = {
  /** True when the page, not the sidebar, owns the keyboard. */
  focused: boolean;
  /** Reports an action for the status line. */
  report: (text: string) => void;
};
