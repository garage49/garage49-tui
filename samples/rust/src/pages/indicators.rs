use garage49_tui_iocraft::{Binding, Chip, ChipTone, Glyphs, Label, LabelVariant, MainFocus, Spinner, SpinnerKind, UseKeys, UseStatus};
use iocraft::prelude::*;

#[component]
pub fn IndicatorsPage(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let focused = hooks.use_context::<MainFocus>().0;
    let status = hooks.use_status();
    let tags = hooks.use_state(|| vec!["typescript", "rust", "한글", "ink", "iocraft", "tui"].into_iter().map(String::from).collect::<Vec<_>>());
    let cursor = hooks.use_state(|| 0usize);
    let filters = hooks.use_state(|| vec!["active".to_string()]);
    let list = tags.read().clone();
    let current = cursor.get().min(list.len().saturating_sub(1));
    let remove = move |tag: String| {
        let mut remaining = tags.read().clone();
        remaining.retain(|t| t != &tag);
        let mut tags = tags;
        let mut cursor = cursor;
        cursor.set(current.min(remaining.len().saturating_sub(1)));
        tags.set(remaining);
        status.report(&format!("removed: {tag}"));
    };
    let toggle_filter = move |filter: String| {
        let mut set = filters.read().clone();
        if set.contains(&filter) { set.retain(|f| f != &filter) } else { set.push(filter) }
        status.report(&format!("filters: {}", if set.is_empty() { "none".to_string() } else { set.join(", ") }));
        let mut filters = filters;
        filters.set(set);
    };
    let len = list.len();
    hooks.use_keys(focused, vec![
        Binding::new(&["left", "h"], move || { let mut cursor = cursor; cursor.set(current.saturating_sub(1)); }),
        Binding::new(&["right", "l"], move || { let mut cursor = cursor; cursor.set((current + 1).min(len.saturating_sub(1))); }),
        Binding::new(&["backspace", "delete", "x"], { let list = list.clone(); move || { if let Some(tag) = list.get(current) { remove(tag.clone()) } } }),
        Binding::new(&["enter"], { let list = list.clone(); move || { if let Some(tag) = list.get(current) { status.report(&format!("chip: {tag}")) } } }),
    ], None);
    let active_filters = filters.read().clone();
    element! {
        View(flex_direction: FlexDirection::Column) {
            Label(content: "Chips · tags", variant: LabelVariant::Heading)
            Label(content: "←→ moves the cursor · x or backspace removes · click selects, click × removes", variant: LabelVariant::Muted)
            View(flex_direction: FlexDirection::Row, margin_top: 1) {
                #(list.iter().enumerate().map(|(index, tag)| {
                    let tag_press = tag.clone();
                    let tag_remove = tag.clone();
                    element! { Chip(key: tag.clone(), label: tag.clone(), selected: focused && index == current, removable: true,
                        on_press: move |_| { let mut cursor = cursor; cursor.set(index); let _ = &tag_press; },
                        on_remove: move |_| remove(tag_remove.clone())) }
                }))
            }
            View(height: 1)
            Label(content: "Chips · filters (click toggles)", variant: LabelVariant::Heading)
            View(flex_direction: FlexDirection::Row, margin_top: 1) {
                #(["active", "idle", "archived"].iter().map(|filter| {
                    let on = active_filters.iter().any(|f| f == filter);
                    let name = filter.to_string();
                    let key = name.clone();
                    element! { Chip(key: key, label: format!("{} {}", if on { Glyphs::ON } else { Glyphs::OFF }, filter), tone: if on { ChipTone::Accent } else { ChipTone::Default }, on_press: move |_| toggle_filter(name.clone())) }
                }))
            }
            View(height: 1)
            Label(content: "Chips · status tones", variant: LabelVariant::Heading)
            View(flex_direction: FlexDirection::Row, margin_top: 1) {
                Chip(label: "running", tone: ChipTone::Success) Chip(label: "degraded", tone: ChipTone::Warning) Chip(label: "failed", tone: ChipTone::Error) Chip(label: "v0.1.0")
            }
            View(height: 1)
            Label(content: "Spinners", variant: LabelVariant::Heading)
            View(flex_direction: FlexDirection::Column, margin_top: 1) {
                Spinner(kind: SpinnerKind::Dots, label: "dots · syncing projects…".to_string())
                Spinner(kind: SpinnerKind::Line, label: "line · building".to_string())
                Spinner(kind: SpinnerKind::Bounce, label: "bounce · waiting for upstream".to_string())
                Spinner(kind: SpinnerKind::Dots, label: "inactive · shows the on glyph".to_string(), active: false)
            }
        }
    }
}
