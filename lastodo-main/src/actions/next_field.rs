use crate::app::{App, TaskField};
use std::result::Result;

pub fn next_field(app: &mut App) -> Result<(), String> {
    app.currently_editing_task
        .as_mut()
        .unwrap()
        .currently_editing = match app
        .currently_editing_task
        .as_ref()
        .unwrap()
        .currently_editing
    {
        Some(TaskField::Title) => Some(TaskField::Description),
        Some(TaskField::Description) => Some(TaskField::Due),
        Some(TaskField::Due) => Some(TaskField::Title),
        None => Some(TaskField::Title),
    };

    Ok(())
}
