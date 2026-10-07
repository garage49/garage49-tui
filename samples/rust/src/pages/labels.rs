use garage49_tui_iocraft::{Label, LabelVariant, Section, Theme};
use iocraft::prelude::*;

#[component]
pub fn LabelsPage(hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    let variants = [
        ("default", LabelVariant::Default), ("muted", LabelVariant::Muted), ("bright", LabelVariant::Bright), ("heading", LabelVariant::Heading),
        ("accent", LabelVariant::Accent), ("success", LabelVariant::Success), ("warning", LabelVariant::Warning), ("error", LabelVariant::Error),
    ];
    let surfaces = [("background", t.background, t.text), ("panel", t.panel, t.text), ("surface", t.surface, t.text), ("surfaceRaised", t.surface_raised, t.text), ("selectionBackground", t.selection_background, t.selection_text)];
    element! {
        View(flex_direction: FlexDirection::Column) {
            Section(title: "Label variants".to_string()) {
            #(variants.iter().enumerate().map(|(index, (name, variant))| element! {
                View(key: index, flex_direction: FlexDirection::Row) {
                    View(width: 12) { Label(content: *name, variant: LabelVariant::Muted) }
                    Label(content: "The quick brown fox · 빠른 갈색 여우 · 🦊", variant: *variant)
                }
            }))
            }
            Section(title: format!("Surfaces ({})", theme.name)) {
            View(flex_direction: FlexDirection::Row) {
                #(surfaces.iter().enumerate().map(|(index, (name, bg, fg))| element! {
                    View(key: index, background_color: *bg, padding_left: 1, padding_right: 1, margin_right: 1, flex_direction: FlexDirection::Column) {
                        Text(content: *name, color: *fg)
                        Text(content: format!("{:?}", bg).to_lowercase().replace("rgb { r: ", "#").replace(", g: ", ",").replace(", b: ", ",").replace(" }", ""), color: *fg)
                    }
                }))
            }
            }
        }
    }
}
