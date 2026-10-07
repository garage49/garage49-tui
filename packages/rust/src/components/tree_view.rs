use std::collections::HashSet;

use iocraft::prelude::*;

use super::truncate::Truncate;
use crate::input::{MouseLayer, UseMouse};
use crate::theme::{Glyphs, Theme};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct TreeNode {
    pub id: String,
    pub label: String,
    pub children: Option<Vec<TreeNode>>,
}

impl TreeNode {
    pub fn leaf(id: &str, label: &str) -> Self {
        Self { id: id.into(), label: label.into(), children: None }
    }
    pub fn branch(id: &str, label: &str, children: Vec<TreeNode>) -> Self {
        Self { id: id.into(), label: label.into(), children: Some(children) }
    }
}

/// One visible row of a flattened tree.
#[derive(Clone, Debug, PartialEq)]
pub struct TreeRow {
    pub node: TreeNode,
    pub depth: usize,
    pub expanded: bool,
    pub parent_id: Option<String>,
}

/// What a mouse click on a tree row means, by the usual file-manager convention: a click on the
/// ▸/▾ marker toggles the branch; a click on the row that is already selected toggles it too; a
/// click on any other row only selects it.
pub struct TreeClick;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeClickAction {
    Toggle,
    Select,
}

impl TreeClick {
    pub fn decide(on_marker: bool, already_selected: bool) -> TreeClickAction {
        if on_marker || already_selected { TreeClickAction::Toggle } else { TreeClickAction::Select }
    }

    /// Whether column `x` of a row at `depth` is the marker cell (after the indent).
    pub fn is_marker(x: i32, depth: usize) -> bool {
        x == depth as i32 * 2
    }
}

/// Flattens a tree into the rows that are visible given the expanded set.
pub struct TreeLayout;

impl TreeLayout {
    /// Whether `id` is `ancestor_id` itself or lies anywhere under it.
    pub fn contains(nodes: &[TreeNode], ancestor_id: &str, id: &str) -> bool {
        fn find<'a>(list: &'a [TreeNode], id: &str) -> Option<&'a TreeNode> {
            list.iter().find_map(|node| if node.id == id { Some(node) } else { node.children.as_deref().and_then(|c| find(c, id)) })
        }
        fn has(node: &TreeNode, id: &str) -> bool {
            node.id == id || node.children.as_deref().is_some_and(|c| c.iter().any(|n| has(n, id)))
        }
        find(nodes, ancestor_id).is_some_and(|ancestor| has(ancestor, id))
    }

    /// The cursor after `branch_id` collapses: a cursor inside the branch would vanish, so it moves to
    /// the branch; any other cursor stays.
    pub fn cursor_after_collapse(nodes: &[TreeNode], branch_id: &str, cursor_id: Option<&str>) -> Option<String> {
        match cursor_id {
            Some(cursor) if Self::contains(nodes, branch_id, cursor) => Some(branch_id.to_string()),
            other => other.map(|s| s.to_string()),
        }
    }

    pub fn rows(nodes: &[TreeNode], expanded: &HashSet<String>) -> Vec<TreeRow> {
        let mut rows = Vec::new();
        Self::collect(nodes, expanded, 0, None, &mut rows);
        rows
    }

    fn collect(nodes: &[TreeNode], expanded: &HashSet<String>, depth: usize, parent: Option<&str>, out: &mut Vec<TreeRow>) {
        for node in nodes {
            let open = expanded.contains(&node.id);
            out.push(TreeRow { node: node.clone(), depth, expanded: open, parent_id: parent.map(|p| p.to_string()) });
            if let (true, Some(children)) = (open, &node.children) {
                Self::collect(children, expanded, depth + 1, Some(&node.id), out);
            }
        }
    }
}

#[derive(Default, Props)]
pub struct TreeViewProps {
    pub nodes: Vec<TreeNode>,
    pub expanded: Vec<String>,
    pub selected_id: Option<String>,
    pub focused: bool,
    /// The cursor moves to a row (click on an unselected row, or the wheel).
    pub on_select: HandlerMut<'static, TreeRow>,
    /// A branch opens or closes (click on the ▸/▾ marker, or on the row that is already selected).
    pub on_toggle: HandlerMut<'static, TreeRow>,
}

/// Indented rows with ▸/▾ markers for branches; the selected row is filled with the accent color.
/// Mouse, by the file-manager convention: a click selects a row, a click on its marker or on the
/// already selected row toggles it; the wheel moves the cursor.
#[component]
pub fn TreeView(props: &mut TreeViewProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    let expanded: HashSet<String> = props.expanded.iter().cloned().collect();
    let rows = TreeLayout::rows(&props.nodes, &expanded);
    let allowed = hooks.mouse_allowed(MouseLayer::Screen);
    {
        let rows = rows.clone();
        let selected = props.selected_id.clone();
        let mut on_toggle = props.on_toggle.take();
        let mut on_select = props.on_select.take();
        hooks.use_mouse(allowed, move |event| match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if event.local_y < 0 {
                    return;
                }
                if let Some(row) = rows.get(event.local_y as usize) {
                    let already = Some(&row.node.id) == selected.as_ref();
                    match TreeClick::decide(TreeClick::is_marker(event.local_x, row.depth), already) {
                        TreeClickAction::Toggle => on_toggle(row.clone()),
                        TreeClickAction::Select => on_select(row.clone()),
                    }
                }
            }
            MouseEventKind::ScrollUp | MouseEventKind::ScrollDown => {
                let delta = if event.kind == MouseEventKind::ScrollUp { -1 } else { 1 };
                let index = rows.iter().position(|r| Some(&r.node.id) == selected.as_ref()).unwrap_or(0) as i32;
                if let Some(row) = rows.get((index + delta).clamp(0, rows.len() as i32 - 1) as usize) {
                    on_select(row.clone());
                }
            }
            _ => {}
        });
    }
    let focused = props.focused;
    let selected = props.selected_id.clone();
    element! {
        View(flex_direction: FlexDirection::Column) {
            #(rows.iter().enumerate().map(|(index, row)| {
                let is_selected = selected.as_deref() == Some(row.node.id.as_str());
                let highlighted = is_selected && focused;
                let fill = if is_selected { Some(if focused { t.selection_background } else { t.surface_raised }) } else { None };
                let marker = match row.node.children { Some(_) => if row.expanded { Glyphs::EXPANDED } else { Glyphs::COLLAPSED }, None => " " };
                let prefix = format!("{}{} ", "  ".repeat(row.depth), marker);
                element! {
                    View(key: index, background_color: fill, flex_direction: FlexDirection::Row) {
                        Text(content: prefix, color: if highlighted { t.selection_text } else { t.text_muted })
                        Truncate(content: row.node.label.clone(), weight: if is_selected { Weight::Bold } else { Weight::Normal }, color: if highlighted { t.selection_text } else { t.text })
                    }
                }
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_only_rows_under_expanded_branches() {
        let nodes = vec![
            TreeNode::branch("a", "a", vec![TreeNode::leaf("a1", "a1"), TreeNode::branch("a2", "a2", vec![TreeNode::leaf("a2x", "a2x")])]),
            TreeNode::leaf("b", "b"),
        ];
        let rows = TreeLayout::rows(&nodes, &HashSet::from(["a".to_string()]));
        let ids: Vec<(&str, usize, Option<&str>)> = rows.iter().map(|r| (r.node.id.as_str(), r.depth, r.parent_id.as_deref())).collect();
        assert_eq!(ids, vec![("a", 0, None), ("a1", 1, Some("a")), ("a2", 1, Some("a")), ("b", 0, None)]);
        let hidden = TreeLayout::rows(&nodes, &HashSet::from(["a2".to_string()]));
        assert_eq!(hidden.iter().map(|r| r.node.id.as_str()).collect::<Vec<_>>(), vec!["a", "b"]);
    }

    #[test]
    fn a_cursor_inside_a_collapsing_branch_moves_to_the_branch() {
        let nodes = vec![
            TreeNode::branch("a", "a", vec![TreeNode::leaf("a1", "a1"), TreeNode::branch("a2", "a2", vec![TreeNode::leaf("a2x", "a2x")])]),
            TreeNode::leaf("b", "b"),
        ];
        assert_eq!(TreeLayout::cursor_after_collapse(&nodes, "a", Some("a2x")).as_deref(), Some("a"));
        assert_eq!(TreeLayout::cursor_after_collapse(&nodes, "a", Some("b")).as_deref(), Some("b"));
        assert_eq!(TreeLayout::cursor_after_collapse(&nodes, "a2", Some("a1")).as_deref(), Some("a1"));
    }

    #[test]
    fn a_click_selects_unless_it_hits_the_marker_or_the_selected_row() {
        assert_eq!(TreeClick::decide(false, false), TreeClickAction::Select);
        assert_eq!(TreeClick::decide(true, false), TreeClickAction::Toggle);
        assert_eq!(TreeClick::decide(false, true), TreeClickAction::Toggle);
        assert!(TreeClick::is_marker(0, 0) && TreeClick::is_marker(2, 1) && !TreeClick::is_marker(3, 1));
    }
}
