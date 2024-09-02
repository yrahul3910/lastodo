use crate::app::{App, CurrentScreen};
use std::result::Result;

pub fn force_quit_editing(app: &mut App) -> Result<(), String> {
    app.current_screen = CurrentScreen::Main;
    Ok(())
}
