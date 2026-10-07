/// Cursor movement over a list of ids, clamped at both ends.
pub struct Selection;

impl Selection {
    pub fn move_by(ids: &[String], selected: Option<&str>, delta: i32) -> Option<String> {
        if ids.is_empty() {
            return None;
        }
        let index = selected.and_then(|id| ids.iter().position(|candidate| candidate == id)).unwrap_or(0) as i32;
        let next = (index + delta).clamp(0, ids.len() as i32 - 1) as usize;
        Some(ids[next].clone())
    }

    pub fn ensure(ids: &[String], selected: Option<&str>) -> Option<String> {
        match selected {
            Some(id) if ids.iter().any(|candidate| candidate == id) => Some(id.to_string()),
            _ => ids.first().cloned(),
        }
    }
}
