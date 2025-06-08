use ansi_term::Colour;

/// Theme for CLI colors
pub struct Theme {
    /// Color for prompts and command input
    pub prompt: Colour,
    /// Color for move notes and annotations
    pub note: Colour,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            prompt: Colour::Blue,
            note: Colour::Green,
        }
    }

    /// Formats a prompt with the theme's prompt color
    pub fn format_prompt(&self, prompt: &str) -> String {
        self.prompt.paint(prompt).to_string()
    }

    /// Formats a note with the theme's note color
    pub fn format_note(&self, note: &str) -> String {
        self.note.paint(note).to_string()
    }
} 