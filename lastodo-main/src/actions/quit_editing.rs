use crate::app::{App, CurrentScreen};
use std::result::Result;

pub fn quit_editing(app: &mut App) -> Result<(), String> {
    if let Some(cur_task) = &app.currently_editing_task {
        if cur_task.has_changed {
            app.message =
                "You have unsaved changes. Use 'w' to save or 'x' to discard.".to_string();
        } else {
            app.current_screen = CurrentScreen::Main;
        }
    } else {
        app.current_screen = CurrentScreen::Main;
    }

    Ok(())
}
