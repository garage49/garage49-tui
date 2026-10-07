use garage49_tui_iocraft::{Intro, Label, LabelVariant, Section};
use iocraft::prelude::*;

#[component]
pub fn HomePage() -> impl Into<AnyElement<'static>> {
    element! {
        View(flex_direction: FlexDirection::Column) {
            Intro(title: "garage49 TUI design system".to_string()) {
                Label(content: "iocraft sample gallery · every element the system defines, one page each.", variant: LabelVariant::Muted)
            }
            Section(title: "How to move".to_string()) {
            Label(content: "tab        cycle focus: navigation → sidebar → page; click a region to focus it")
            Label(content: "enter/esc  go down into the view / back up")
            Label(content: "↑↓ / jk    move in the sidebar or in the page's list")
            Label(content: "←→ / hl    switch the view while the top navigation is focused")
            Label(content: "ctrl+p     command palette · t toggle theme · ? help · q quit")
            Label(content: "mouse      click rows, tabs, menus, buttons and key hints; wheel scrolls lists and logs")
            }
            Section(title: "Surfaces".to_string()) {
                Label(content: "No line borders. Regions are background colors; edges are half blocks; focus is a left accent bar.", variant: LabelVariant::Muted)
            }
        }
    }
}
