use crate::app::App;
use std::result::Result;

pub fn save_task(app: &mut App) -> Result<(), String> {
    if app.cur_task.is_none() {
        return Err(String::from("No task was selected."));
    }

    let status = app.cur_task.as_ref().unwrap().status;

    if app.currently_editing_task.is_none() {
        return Err(String::from("No task is currently being edited."));
    }

    let cur_task = app.get_cur_task().unwrap();
    let task_list = app.task_list.entry(status.clone()).or_default();
    let index = task_list.iter().position(|task| *task == cur_task).unwrap();

    task_list[index] = cur_task;

    Ok(())
}
