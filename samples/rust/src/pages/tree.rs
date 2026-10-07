use std::collections::HashSet;

use garage49_tui_iocraft::{Binding, MainFocus, TreeLayout, TreeNode, TreeRow, TreeView, UseKeys, UseStatus, Section};
use iocraft::prelude::*;

fn nodes() -> Vec<TreeNode> {
    vec![
        TreeNode::branch("src", "src", vec![
            TreeNode::branch("src/components", "components", vec![TreeNode::leaf("src/components/list.rs", "list.rs"), TreeNode::leaf("src/components/table.rs", "table.rs"), TreeNode::leaf("src/components/tree_view.rs", "tree_view.rs")]),
            TreeNode::branch("src/theme", "theme", vec![TreeNode::leaf("src/theme/theme.rs", "theme.rs"), TreeNode::leaf("src/theme/glyphs.rs", "glyphs.rs")]),
            TreeNode::leaf("src/lib.rs", "lib.rs"),
        ]),
        TreeNode::branch("docs", "문서", vec![TreeNode::leaf("docs/HANDOFF.md", "HANDOFF.md"), TreeNode::leaf("docs/DESIGN.md", "DESIGN.md")]),
        TreeNode::leaf("Cargo.toml", "Cargo.toml"),
    ]
}

#[component]
pub fn TreePage(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let focused = hooks.use_context::<MainFocus>().0;
    let status = hooks.use_status();
    let expanded = hooks.use_state(|| vec!["src".to_string(), "src/components".to_string()]);
    let selected = hooks.use_state(|| Some("src".to_string()));
    let tree = nodes();
    let set: HashSet<String> = expanded.read().iter().cloned().collect();
    let rows = TreeLayout::rows(&tree, &set);
    let current_id = selected.read().clone();
    let index = rows.iter().position(|r| Some(&r.node.id) == current_id.as_ref());
    let current = index.and_then(|i| rows.get(i).cloned());
    let toggle = move |row: &TreeRow, open: Option<bool>| {
        if row.node.children.is_none() { status.report(&format!("open: {}", row.node.label)); return; }
        let mut list = expanded.read().clone();
        let will_open = open.unwrap_or(!list.contains(&row.node.id));
        list.retain(|id| id != &row.node.id);
        if will_open { list.push(row.node.id.clone()); }
        let mut expanded = expanded;
        expanded.set(list);
        // A cursor inside a collapsing branch would vanish: it moves to the branch.
        if !will_open {
            let cursor = selected.read().clone();
            let mut selected = selected;
            selected.set(TreeLayout::cursor_after_collapse(&nodes(), &row.node.id, cursor.as_deref()));
        }
    };
    let move_to = { let rows = rows.clone(); move |delta: i32| { let i = (index.unwrap_or(0) as i32 + delta).clamp(0, rows.len() as i32 - 1) as usize; let mut selected = selected; selected.set(Some(rows[i].node.id.clone())); } };
    let up = { let move_to = move_to.clone(); move || move_to(-1) };
    let down = { let move_to = move_to.clone(); move || move_to(1) };
    let expand = { let current = current.clone(); let move_to = move_to.clone(); move || { if let Some(row) = &current { if row.node.children.is_some() && !row.expanded { toggle(row, Some(true)) } else { move_to(1) } } } };
    let collapse = { let current = current.clone(); move || { if let Some(row) = &current { if row.expanded { toggle(row, Some(false)) } else if let Some(parent) = &row.parent_id { let mut selected = selected; selected.set(Some(parent.clone())); } } } };
    let enter = { let current = current.clone(); move || { if let Some(row) = &current { toggle(row, None) } } };
    hooks.use_keys(focused, vec![Binding::new(&["up", "k"], up), Binding::new(&["down", "j"], down), Binding::new(&["right", "l"], expand), Binding::new(&["left", "h"], collapse), Binding::new(&["enter", "space"], enter)], None);
    let on_toggle = move |row: TreeRow| toggle(&row, None);
    let on_select = move |row: TreeRow| { let mut selected = selected; selected.set(Some(row.node.id)); };
    element! {
        Section(title: "Files".to_string()) {
        View(flex_direction: FlexDirection::Column, width: 50) {
            TreeView(nodes: tree, expanded: expanded.read().clone(), selected_id: current_id, focused: focused, on_select: on_select, on_toggle: on_toggle)
        }
        }
    }
}
