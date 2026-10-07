use super::list::ListItem;

/// One rendered row of a List: a section heading, a blank gap between sections, or an item.
#[derive(Clone, Debug, PartialEq)]
pub enum ListRow {
    Section(String),
    Gap,
    Item(ListItem),
}

/// Turns items with optional sections into the rows a List draws, in order.
pub struct ListLayout;

impl ListLayout {
    pub fn rows(items: &[ListItem]) -> Vec<ListRow> {
        let mut rows = Vec::new();
        let mut section: Option<&str> = None;
        for item in items {
            if item.section.as_deref() != section {
                section = item.section.as_deref();
                if !rows.is_empty() {
                    rows.push(ListRow::Gap);
                }
                if let Some(title) = section {
                    rows.push(ListRow::Section(title.to_string()));
                }
            }
            rows.push(ListRow::Item(item.clone()));
        }
        rows
    }

    /// The item drawn on the given row, if any (for mouse hit-testing).
    pub fn item_at(rows: &[ListRow], index: i32) -> Option<&ListItem> {
        if index < 0 {
            return None;
        }
        match rows.get(index as usize) {
            Some(ListRow::Item(item)) => Some(item),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_items_under_sections_with_gaps_between() {
        let items = vec![
            ListItem::new("a", "A").section("One"),
            ListItem::new("b", "B").section("One"),
            ListItem::new("c", "C").section("Two"),
        ];
        let rows = ListLayout::rows(&items);
        assert_eq!(rows.len(), 6);
        assert_eq!(rows[0], ListRow::Section("One".into()));
        assert_eq!(rows[3], ListRow::Gap);
        assert_eq!(ListLayout::item_at(&rows, 5).map(|i| i.id.as_str()), Some("c"));
        assert_eq!(ListLayout::item_at(&rows, 3), None);
    }
}
