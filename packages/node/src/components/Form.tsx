import {Box} from 'ink';
import {Children, createContext, isValidElement, useContext, type ReactNode} from 'react';
import {TextWidth} from '../theme/TextWidth.js';

/** The two columns every field of a Form shares. */
export type FormLayout = {readonly labelWidth: number; readonly valueWidth: number};

const FormContext = createContext<FormLayout | null>(null);

/** The enclosing Form's columns, if any; fields fall back to their own defaults outside a Form. */
export function useFormLayout(): FormLayout | null {
  return useContext(FormContext);
}

/** A field's two widths: its own props win, then the enclosing Form, then the gallery defaults (14 / 24). */
export function useFieldColumns(labelWidth?: number, width?: number): {labelCol: number; valueCol: number} {
  const form = useContext(FormContext);
  return {labelCol: labelWidth ?? form?.labelWidth ?? 14, valueCol: width ?? form?.valueWidth ?? 24};
}

type Props = {children: ReactNode; /** The form's width; the value column is what the label column leaves. */ width?: number};

/** The narrowest label column a Form uses, so short labels still line up with the gallery. */
export const MIN_LABEL_WIDTH = 12;

/**
 * A group of fields that share one label column and one value column. The label column is as wide
 * as the longest label (plus two cells, at least 12); the value column is the rest of the width, so
 * every text field, select and text area in the form is exactly as wide as its neighbours.
 */
export function Form({children, width = 60}: Props) {
  const labels = Children.toArray(children).flatMap(child => (isValidElement<{label?: string}>(child) && typeof child.props.label === 'string' ? [child.props.label] : []));
  const labelWidth = Math.max(MIN_LABEL_WIDTH, TextWidth.widest(labels) + 2);
  const valueWidth = Math.max(8, width - labelWidth - 1);
  return (
    <FormContext.Provider value={{labelWidth, valueWidth}}>
      <Box flexDirection="column" width={width} flexShrink={0}>{children}</Box>
    </FormContext.Provider>
  );
}
