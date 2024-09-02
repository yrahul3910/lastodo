use crate::app::{App, TaskField};
use std::result::Result;

pub fn prev_field(app: &mut App) -> Result<(), String> {
    app.currently_editing_task
        .as_mut()
        .unwrap()
        .currently_editing = match app
        .currently_editing_task
        .as_ref()
        .unwrap()
        .currently_editing
    {
        Some(TaskField::Title) => Some(TaskField::Due),
        Some(TaskField::Description) => Some(TaskField::Title),
        Some(TaskField::Due) => Some(TaskField::Description),
        None => Some(TaskField::Title),
    };

    Ok(())
}
