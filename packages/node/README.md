# @garage49/garage49-tui-ink

The garage49 TUI design system for Node.js: OpenCode's look and feel as [Ink](https://github.com/vadimdemedes/ink) components, with an application shell that gives every app the same navigation, focus, mouse, overlays and status bars.

## Install

```bash
npm install @garage49/garage49-tui-ink ink react
```

Requires Node.js 20 or later, Ink 8 and React 19.

## Use

```tsx
import {useState} from 'react';
import {App, Content, Main, Nav, Sidebar, Table, run, useFocused, useStatus} from '@garage49/garage49-tui-ink';

const views = [{id: 'work', label: 'Work'}, {id: 'settings', label: 'Settings'}];
const pages = [{id: 'projects', label: 'Projects', section: 'Browse'}, {id: 'log', label: 'Activity', section: 'Browse'}];

function ProjectsPage() {
  const focused = useFocused();
  const {report} = useStatus();
  const [selected, setSelected] = useState('garage49-tui');
  return (
    <Table
      columns={[{id: 'name', title: 'Name', width: 24, cell: (p: {name: string}) => p.name}]}
      rows={[{name: 'garage49-tui'}, {name: 'skills'}]}
      rowId={p => p.name}
      selectedId={selected}
      focused={focused}
      onSelect={p => setSelected(p.name)}
      onClick={p => report(`open: ${p.name}`)}
    />
  );
}

function MyApp() {
  const [view, setView] = useState('work');
  const [page, setPage] = useState('projects');
  return (
    <App context="myapp">
      <Nav brand="myapp" items={views} activeId={view} onChange={item => setView(item.id)} right="v1.0.0" />
      <Content>
        {view === 'work' && <Sidebar items={pages} selectedId={page} onSelect={item => setPage(item.id)} />}
        <Main title="Projects"><ProjectsPage /></Main>
      </Content>
    </App>
  );
}

run(<MyApp />);
```

## Rules of use

Read these before writing app code.

1. **The root renders `App` and nothing else.** `useOverlay`, `useStatus`, `useTyping` and `useFocused` are contexts that `App` provides, so they work only in App's children. Put the app body, the pages and every hook call in child components.
2. **Running inside an existing event loop.** `run()` calls Ink's `render` and returns its `Instance`; it does not block. Keep servers and timers on the same Node event loop and call `instance.unmount()` (or `exit` from `useApp`) to leave.
3. **Pushing a background task's state into the UI.** Hold the task's state in React state (or an external store read with `useSyncExternalStore`) in a component under `App`, and update it from the task's callbacks; never render from the task.
4. **Page letter keys yield while a field is editing.** A page that binds letters (`j k e d s …`) must pass `isActive: focused && !typing` (read it with `useTypingState()` from the shell) or those keys also type into the editing field. App's own `q ? t m` already yield.
5. **Editor round trips.** Use Ink's `useApp().suspendTerminal()` (Ink 8) around the external program; the shell does not wrap it yet.

`run` mounts the app on the alternate screen with the theme that fits the terminal (OpenCode's truecolor theme, or the system theme without truecolor) and restores the terminal on exit. `App` provides focus cycling (tab, enter/esc), the mouse, the overlay slot, the command palette (ctrl+p), help (?), the quit confirmation (q), theme (t) and mouse (m) toggles, the status line and the key hint bar.

## Build

```bash
npm run build
```

Emits `dist/` with JavaScript and type declarations.

## Test

Unit tests:

```bash
npm test
```

Complexity check (ESLint: cyclomatic complexity 10, 50 lines per function, 5 parameters):

```bash
npm run complexity
```

`npm run check` runs the type check, the complexity check and the tests.

## Run

The gallery in `samples/node` of the repository shows every component:

```bash
npm run gallery
```
