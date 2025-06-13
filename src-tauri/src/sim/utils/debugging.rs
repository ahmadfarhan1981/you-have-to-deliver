use std::fmt;
use legion::system;

#[derive(Clone, Debug, Default)]
pub struct DebugDisplayComponent {
    // Each entry is a (label, value) pair for display
    pub entries: Vec<(String, String)>,
}

impl fmt::Display for DebugDisplayComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.entries.is_empty() {
            return writeln!(f, "No debug entries.");
        }

        // Determine the maximum width needed for the label column
        let max_label_width = self
            .entries
            .iter()
            .map(|(label, _)| label.len())
            .max()
            .unwrap_or(0);

        // Optional: Add a title for the debug output
        // writeln!(f, "--- Debug Information ---")?;

        for (label, value) in &self.entries {
            writeln!(f, "{:<width$} : {}", label, value, width = max_label_width)?;
        }

        Ok(())
    }
}

#[system(for_each)]
pub fn clear_debug_display(
    debug_display_component: &mut DebugDisplayComponent
){
  debug_display_component.entries.clear()
}