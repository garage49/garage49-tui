use garage49_tui_iocraft::{HelpEntry, Label, LabelVariant, TextWidth};
use iocraft::prelude::*;

#[derive(Default, Props)]
pub struct HelpPageProps {
    pub entries: Vec<HelpEntry>,
}

#[component]
pub fn HelpPage(props: &mut HelpPageProps) -> impl Into<AnyElement<'static>> {
    let key_width = TextWidth::widest(props.entries.iter().map(|e| e.keys.as_str())) as u16 + 2;
    element! {
        View(flex_direction: FlexDirection::Column) {
            Label(content: "Keys (this app)", variant: LabelVariant::Heading)
            Label(content: "The shell's own keys are under ? · tab cycles, enter/esc go down/up, ctrl+p palette, t theme, m mouse, q quit.", variant: LabelVariant::Muted)
            View(height: 1)
            #(props.entries.iter().enumerate().map(|(index, entry)| element! {
                View(key: index, flex_direction: FlexDirection::Row) {
                    View(width: key_width, flex_shrink: 0.0_f32) { Label(content: entry.keys.clone()) }
                    Label(content: entry.action.clone(), variant: LabelVariant::Muted)
                }
            }))
            View(height: 1)
            Label(content: "Color roles", variant: LabelVariant::Heading)
            Label(content: "accent (orange) = the cursor: one per focused region · accentSecondary (blue) = the focused region's bar, shown only where the keys go · success/warning/error = state values", variant: LabelVariant::Muted)
        }
    }
}
