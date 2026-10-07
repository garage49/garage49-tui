// Theme
export {Theme, opencodeTheme, systemTheme, type ThemeTokens} from './theme/Theme.js';
export {ThemeProvider, ThemeRoot, useTheme, useThemeSwitch} from './theme/ThemeContext.js';
export {Glyphs} from './theme/Glyphs.js';
export {TextWidth} from './theme/TextWidth.js';

// Input
export {useKeys, type Binding, type KeysOptions} from './input/Keys.js';
export {KeyChord} from './input/KeyChord.js';
export {useMouseTarget, useMouseSwitch, MouseLayer, type MouseHandlers, type LocalMouseEvent} from './input/Mouse.js';
export {MouseParser, type MouseEvent, type MouseButton} from './input/MouseParser.js';

// Screen and overlays
export {Screen, useOverlay, type OverlayHandle, type OverlayOptions} from './components/Screen.js';
export {Overlay} from './components/Overlay.js';
export {ConfirmDialog} from './components/ConfirmDialog.js';
export {MessageDialog} from './components/MessageDialog.js';
export {HelpOverlay, type HelpEntry} from './components/HelpOverlay.js';
export {Palette} from './components/Palette.js';
export {Dropdown} from './components/Dropdown.js';

// Regions and surfaces
export {FocusRegion, type RegionSurface} from './components/FocusRegion.js';
export {Panel} from './components/Panel.js';
export {TopNav, type NavItem} from './components/TopNav.js';
export {Tabs, type Tab, type TabsSize} from './components/Tabs.js';
export {StatusLine, type StatusSegment} from './components/StatusLine.js';
export {KeyHintBar, type KeyHint} from './components/KeyHintBar.js';

// Data
export {List, type ListItem} from './components/List.js';
export {ListLayout, type ListRow} from './components/ListLayout.js';
export {Selection} from './components/Selection.js';
export {Table, type Column} from './components/Table.js';
export {TreeView, TreeLayout, type TreeNode, type TreeRow} from './components/TreeView.js';
export {TreeClick, type TreeClickAction} from './components/TreeClick.js';
export {LogView, type LogEntry, type LogLevel} from './components/LogView.js';
export {Scrollbar, ScrollThumb} from './components/Scrollbar.js';

// Form fields and editing
export {FieldBar} from './components/FieldBar.js';
export {TextField} from './components/TextField.js';
export {TextArea} from './components/TextArea.js';
export {TextBuffer} from './components/TextBuffer.js';
export {Input} from './components/Input.js';
export {Select} from './components/Select.js';
export {Toggle} from './components/Toggle.js';
export {Checkbox} from './components/Checkbox.js';
export {RadioGroup} from './components/RadioGroup.js';
export {Button} from './components/Button.js';

// Layout
export {Section} from './components/Section.js';
export {Split} from './components/Split.js';
export {Form, useFormLayout, useFieldColumns, MIN_LABEL_WIDTH, type FormLayout} from './components/Form.js';

// Text and indicators
export {Label, type LabelVariant} from './components/Label.js';
export {Chip, type ChipTone} from './components/Chip.js';
export {Spinner, type SpinnerKind} from './components/Spinner.js';

// Application shell
export {App, type Command} from './shell/App.js';
export {Nav} from './shell/Nav.js';
export {Content} from './shell/Content.js';
export {Sidebar} from './shell/Sidebar.js';
export {Main} from './shell/Main.js';
export {useFocused} from './shell/FocusRegistry.js';
export {useStatus} from './shell/Status.js';
export {useTyping, useTypingState} from './shell/Typing.js';
export {run, TerminalTheme} from './shell/run.js';
export {ScreenFit, type Fit} from './shell/Fit.js';
