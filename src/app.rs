use std::hash::Hash;

use chrono::DateTime;
use color_eyre::{eyre::WrapErr, Result};
use crossterm::event;
use crossterm::event::{KeyCode, KeyEventKind};
use indexmap::IndexMap;

use crate::tui::Tui;

#[derive(Debug, Default)]
pub enum CurrentScreen {
    #[default]
    Main,
    Editing,
    Exiting,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum KanbanStatus {
    #[default]
    Todo,
    Partial,
    Doing,
    Done,
    Blocked,
}

impl std::fmt::Display for KanbanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            KanbanStatus::Todo => write!(f, "Todo"),
            KanbanStatus::Partial => write!(f, "Partial"),
            KanbanStatus::Doing => write!(f, "Doing"),
            KanbanStatus::Done => write!(f, "Done"),
            KanbanStatus::Blocked => write!(f, "Blocked"),
        }
    }
}

#[derive(Debug)]
pub enum TaskField {
    Title,
    Description,
    Due,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Task {
    pub title: String,
    pub kanban_status: KanbanStatus,
    pub description: String,
    pub due: DateTime<chrono::Local>,
}

#[derive(Debug)]
pub struct TaskEditState {
    pub currently_editing: Option<TaskField>,
    pub cur_value: String,
    pub is_new_task: bool,
    pub has_changed: bool,
}

#[derive(Debug, Default)]
pub struct App {
    pub cur_task: Option<Task>, // The currently highlighted task
    pub task_list: IndexMap<KanbanStatus, Vec<Task>>, // The list of tasks
    pub current_screen: CurrentScreen, // The current screen
    pub currently_editing_list: Option<KanbanStatus>, // The list currently being edited
    pub currently_editing_task: Option<TaskEditState>, // The task currently being edited
    pub message: String,        // Status message
    pub exit: bool,             // Whether to exit the application
}

impl App {
    pub fn new() -> App {
        let mut task_list = IndexMap::new();
        let task1 = Task {
            title: "Task 1".to_string(),
            kanban_status: KanbanStatus::Todo,
            description: "This is a task".to_string(),
            due: chrono::Local::now(),
        };
        let task2 = Task {
            title: "Task 2".to_string(),
            kanban_status: KanbanStatus::Partial,
            description: "This is another task".to_string(),
            due: chrono::Local::now(),
        };
        task_list.insert(KanbanStatus::Todo, vec![task1.clone()]);
        task_list.insert(KanbanStatus::Partial, vec![task2]);
        task_list.insert(KanbanStatus::Doing, Vec::new());
        task_list.insert(KanbanStatus::Done, Vec::new());
        task_list.insert(KanbanStatus::Blocked, Vec::new());

        App {
            cur_task: Some(task1),
            task_list,
            current_screen: CurrentScreen::Main,
            currently_editing_list: None,
            currently_editing_task: None,
            message: String::new(),
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }

        Ok(())
    }

    pub fn save_task(&mut self) -> std::result::Result<(), String> {
        if self.cur_task.is_none() {
            return Err(String::from("No task was selected."));
        }

        let status = &self.cur_task.as_ref().unwrap().kanban_status;

        if self.currently_editing_task.is_none() {
            return Err(String::from("No task is currently being edited."));
        }

        let cur_task = self.cur_task.as_ref().unwrap();

        let task_list = self
            .task_list
            .entry(status.clone())
            .or_insert_with(Vec::new);
        let index = task_list.iter().position(|task| task == cur_task).unwrap();
        task_list[index] = self.cur_task.clone().unwrap();

        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match self.current_screen {
                    CurrentScreen::Exiting => {
                        self.exit = true;
                    }
                    CurrentScreen::Editing => match key_event.code {
                        KeyCode::Char('w') => {
                            let _ = self.save_task();
                        }
                        KeyCode::Char('q') | KeyCode::Esc => {
                            if let Some(cur_task) = &self.currently_editing_task {
                                if cur_task.has_changed {
                                    self.message =
                                    "You have unsaved changes. Use 'w' to save or 'x' to discard."
                                        .to_string();
                                } else {
                                    self.current_screen = CurrentScreen::Main;
                                }
                            } else {
                                self.current_screen = CurrentScreen::Main;
                            }
                        }
                        KeyCode::Char('x') => {
                            self.current_screen = CurrentScreen::Main;
                        }
                        // Handle editing case
                        KeyCode::Char(val) => {
                            if let Some(field) = &self.currently_editing_task.as_mut() {
                                let cur_task = self.cur_task.as_mut().unwrap();

                                match field.currently_editing {
                                    Some(TaskField::Title) => {
                                        cur_task.title.push(val);
                                    }
                                    Some(TaskField::Description) => {
                                        cur_task.description.push(val);
                                    }
                                    Some(TaskField::Due) => {
                                        cur_task.due = chrono::Local::now();
                                    }
                                    _ => {}
                                }
                            }
                        }
                        // TODO: Other keys, such as moving across fields.
                        _ => {}
                    },
                    CurrentScreen::Main => match key_event.code {
                        KeyCode::Char('q') => {
                            self.exit = true;
                        }
                        KeyCode::Char('w') => {}
                        KeyCode::Char('i') => {
                            self.current_screen = CurrentScreen::Editing;

                            // TODO: Check if we are highlighting a task
                        }
                        _ => {}
                    },
                }
            }
            _ => {}
        }
        Ok(())
    }
}
