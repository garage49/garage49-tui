import {useState, type ReactNode} from 'react';
import {DialogsPage} from './pages/DialogsPage.js';
import {EditingPage} from './pages/EditingPage.js';
import {FormPage} from './pages/FormPage.js';
import {HelpPage} from './pages/HelpPage.js';
import {HomePage} from './pages/HomePage.js';
import {IndicatorsPage} from './pages/IndicatorsPage.js';
import {LabelsPage} from './pages/LabelsPage.js';
import {ListPage} from './pages/ListPage.js';
import {LogPage} from './pages/LogPage.js';
import type {PageProps} from './pages/Page.js';
import {SettingsPage} from './pages/SettingsPage.js';
import {TablePage} from './pages/TablePage.js';
import {TabsPage} from './pages/TabsPage.js';
import {TreePage} from './pages/TreePage.js';
import {App, Content, Main, Nav, Sidebar, useFocused, useStatus} from '@garage49/garage49-tui-ink';
import type {ListItem, Command} from '@garage49/garage49-tui-ink';

type Page = ListItem & {readonly title: string; readonly render: (props: PageProps) => ReactNode};

/** The gallery's pages, listed in the sidebar of the Gallery view. */
const pages: readonly Page[] = [
  {id: 'home', label: 'Home', section: 'Overview', title: 'Home', render: p => <HomePage {...p} />},
  {id: 'labels', label: 'Labels & tokens', section: 'Overview', title: 'Labels and color tokens', render: p => <LabelsPage {...p} />},
  {id: 'tabs', label: 'Tabs', section: 'Navigation', title: 'Tabs', render: p => <TabsPage {...p} />},
  {id: 'tree', label: 'Tree view', section: 'Navigation', title: 'Tree view', render: p => <TreePage {...p} />},
  {id: 'list', label: 'List', section: 'Data', title: 'List', render: p => <ListPage {...p} />},
  {id: 'table', label: 'Table', section: 'Data', title: 'Table', render: p => <TablePage {...p} />},
  {id: 'log', label: 'Log view', section: 'Data', title: 'Log view', render: p => <LogPage {...p} />},
  {id: 'form', label: 'Form fields', section: 'Input', title: 'Form fields', render: p => <FormPage {...p} />},
  {id: 'editing', label: 'Text editing', section: 'Input', title: 'Text editing · text area and input box', render: p => <EditingPage {...p} />},
  {id: 'dialogs', label: 'Dialogs & pop-ups', section: 'Feedback', title: 'Dialogs and pop-ups', render: p => <DialogsPage {...p} />},
  {id: 'indicators', label: 'Chips & spinner', section: 'Feedback', title: 'Chips and spinners', render: p => <IndicatorsPage {...p} />},
];

const galleryHelp = [{keys: '←→ / hl', action: 'switch the view while the navigation is focused'}];

/** The views behind the top navigation. Gallery has a sidebar; the others are a single page. */
const views: readonly (ListItem & {readonly page?: Page})[] = [
  {id: 'gallery', label: 'Gallery'},
  {id: 'dashboard', label: 'Dashboard', page: {id: 'dashboard', label: 'Dashboard', title: 'Dashboard · a view without a sidebar', render: p => <HomePage {...p} />}},
  {id: 'settings', label: 'Settings', page: {id: 'settings', label: 'Settings', title: 'Settings', render: p => <SettingsPage {...p} />}},
  {id: 'help', label: 'Help', page: {id: 'help', label: 'Help', title: 'Help', render: p => <HelpPage {...p} entries={galleryHelp} />}},
];

/** Bridges a page written against PageProps to the shell's hooks. */
function PageHost({page}: {page: Page}) {
  const focused = useFocused();
  const {report} = useStatus();
  return <>{page.render({focused, report})}</>;
}

export function Gallery() {
  const [viewId, setViewId] = useState('gallery');
  const [pageId, setPageId] = useState('home');
  const view = views.find(candidate => candidate.id === viewId) ?? views[0]!;
  const page = view.page ?? pages.find(candidate => candidate.id === pageId) ?? pages[0]!;
  const commands: readonly Command[] = [
    ...views.map(v => ({id: `view:${v.id}`, label: `Go to ${v.label}`, section: 'Views', run: () => setViewId(v.id)})),
    ...pages.map(p => ({id: `page:${p.id}`, label: `Open ${p.label}`, section: 'Gallery', run: () => { setViewId('gallery'); setPageId(p.id); }})),
  ];
  return (
    <App
      commands={commands}
      help={galleryHelp}
      status={[{id: 'view', text: view.label, tone: 'muted'}, {id: 'page', text: page.label, tone: 'muted'}]}
      context="garage49 · ~/work/garage49-tui"
      quitMessage="Quit garage49?"
    >
      <Nav brand="garage49" items={views} activeId={view.id} onChange={item => setViewId(item.id)} right="v0.1.0" />
      <Content>
        {view.page === undefined && <Sidebar items={pages} selectedId={page.id} onSelect={item => setPageId(item.id)} />}
        <Main title={page.title}>
          <PageHost page={page} />
        </Main>
      </Content>
    </App>
  );
}
