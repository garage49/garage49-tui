use iocraft::prelude::*;

/// The name of a key press as bindings spell it: "up", "enter", "esc", "space", "shift+tab",
/// "ctrl+p", "alt+f", or the printable character itself ("q", "?", "k").
pub struct KeyChord;

impl KeyChord {
    pub fn of(event: &KeyEvent) -> Option<String> {
        let named = match event.code {
            KeyCode::Up => Some("up"),
            KeyCode::Down => Some("down"),
            KeyCode::Left => Some("left"),
            KeyCode::Right => Some("right"),
            KeyCode::PageUp => Some("pageup"),
            KeyCode::PageDown => Some("pagedown"),
            KeyCode::Home => Some("home"),
            KeyCode::End => Some("end"),
            KeyCode::Enter => Some("enter"),
            KeyCode::Esc => Some("esc"),
            KeyCode::Tab => Some("tab"),
            KeyCode::BackTab => Some("shift+tab"),
            KeyCode::Backspace => Some("backspace"),
            KeyCode::Delete => Some("delete"),
            _ => None,
        };
        let base = match (named, event.code) {
            (Some(name), _) => name.to_string(),
            (None, KeyCode::Char(' ')) => "space".to_string(),
            (None, KeyCode::Char(c)) => c.to_string(),
            _ => return None,
        };
        let mut prefix = String::new();
        if event.modifiers.contains(KeyModifiers::CONTROL) {
            prefix.push_str("ctrl+");
        }
        if event.modifiers.contains(KeyModifiers::ALT) {
            prefix.push_str("alt+");
        }
        if event.modifiers.contains(KeyModifiers::SHIFT) && named.is_some() && named != Some("shift+tab") {
            prefix.push_str("shift+");
        }
        Some(prefix + &base)
    }

    /// Printable text that a text field would insert: a character without ctrl/alt that is not a named key.
    pub fn text(event: &KeyEvent) -> Option<String> {
        if event.modifiers.intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) {
            return None;
        }
        match event.code {
            KeyCode::Char(c) => Some(c.to_string()),
            _ => None,
        }
    }
}

/// One key binding: any of `keys` runs `run`.
pub struct Binding {
    pub keys: &'static [&'static str],
    pub run: Box<dyn FnMut() + Send + Sync + 'static>,
}

impl Binding {
    pub fn new(keys: &'static [&'static str], run: impl FnMut() + Send + Sync + 'static) -> Self {
        Self { keys, run: Box::new(run) }
    }
}

/// Keyboard input for components, as a table of bindings. Components never read terminal events directly.
pub trait UseKeys {
    /// Runs the first binding whose keys match a press while `active`; unclaimed printable text goes to `on_text`.
    fn use_keys(&mut self, active: bool, bindings: Vec<Binding>, on_text: Option<Box<dyn FnMut(String) + Send + Sync + 'static>>);
}

impl UseKeys for Hooks<'_, '_> {
    fn use_keys(&mut self, active: bool, mut bindings: Vec<Binding>, mut on_text: Option<Box<dyn FnMut(String) + Send + Sync + 'static>>) {
        self.use_terminal_events(move |event| {
            if !active {
                return;
            }
            let TerminalEvent::Key(key) = event else { return };
            if key.kind == KeyEventKind::Release {
                return;
            }
            let chord = KeyChord::of(&key);
            if let Some(chord) = chord {
                if let Some(binding) = bindings.iter_mut().find(|b| b.keys.contains(&chord.as_str())) {
                    (binding.run)();
                    return;
                }
            }
            if let (Some(text), Some(on_text)) = (KeyChord::text(&key), on_text.as_mut()) {
                on_text(text);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        let mut event = KeyEvent::new(KeyEventKind::Press, code);
        event.modifiers = modifiers;
        event
    }

    #[test]
    fn names_arrows_enter_esc_and_space() {
        assert_eq!(KeyChord::of(&key(KeyCode::Up, KeyModifiers::empty())).as_deref(), Some("up"));
        assert_eq!(KeyChord::of(&key(KeyCode::Enter, KeyModifiers::empty())).as_deref(), Some("enter"));
        assert_eq!(KeyChord::of(&key(KeyCode::Char(' '), KeyModifiers::empty())).as_deref(), Some("space"));
    }

    #[test]
    fn prefixes_modifiers_and_keeps_printable_characters() {
        assert_eq!(KeyChord::of(&key(KeyCode::Char('p'), KeyModifiers::CONTROL)).as_deref(), Some("ctrl+p"));
        assert_eq!(KeyChord::of(&key(KeyCode::Char('f'), KeyModifiers::ALT)).as_deref(), Some("alt+f"));
        assert_eq!(KeyChord::of(&key(KeyCode::BackTab, KeyModifiers::SHIFT)).as_deref(), Some("shift+tab"));
        assert_eq!(KeyChord::of(&key(KeyCode::Char('?'), KeyModifiers::empty())).as_deref(), Some("?"));
        assert_eq!(KeyChord::of(&key(KeyCode::Char('한'), KeyModifiers::empty())).as_deref(), Some("한"));
    }

    #[test]
    fn treats_plain_characters_as_text_but_not_control_chords() {
        assert_eq!(KeyChord::text(&key(KeyCode::Char('q'), KeyModifiers::empty())).as_deref(), Some("q"));
        assert_eq!(KeyChord::text(&key(KeyCode::Char('p'), KeyModifiers::CONTROL)), None);
        assert_eq!(KeyChord::text(&key(KeyCode::Up, KeyModifiers::empty())), None);
    }
}
