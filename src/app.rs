use std::hash::Hash;

use chrono::DateTime;
use color_eyre::{eyre::WrapErr, Result};
use crossterm::event;
use ratatui::Frame;
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

#[derive(Debug)]
pub enum TaskField {
    Title,
    Description,
    Due
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
    pub cur_task: Task,
    pub task_list: IndexMap<KanbanStatus, Vec<Task>>,
    pub current_screen: CurrentScreen,
    pub currently_editing_list: Option<KanbanStatus>,
    pub currently_editing_task: Option<TaskEditState>,
    pub message: String,
    pub exit: bool,
}

impl App {
    pub fn new() -> App {
        let mut task_list = IndexMap::new();
        task_list.insert(KanbanStatus::Todo, Vec::new());
        task_list.insert(KanbanStatus::Partial, Vec::new());
        task_list.insert(KanbanStatus::Doing, Vec::new());
        task_list.insert(KanbanStatus::Done, Vec::new());
        task_list.insert(KanbanStatus::Blocked, Vec::new());

        App {
            cur_task: Task::default(),
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
        let status = &self.cur_task.kanban_status;

        if self.currently_editing_task.is_none() {
            return Err(String::from("No task is currently being edited."));
        }

        // If we're editing an existing task, update it. Otherwise, add a new one.
        if self.currently_editing_task.is_some() {
            self.task_list
                .entry(status.clone())
                .or_insert_with(Vec::new)
                .push(self.cur_task.clone());
        } else {
            let task_list = self
                .task_list
                .entry(status.clone())
                .or_insert_with(Vec::new);
            let index = task_list
                .iter()
                .position(|task| task == &self.cur_task)
                .unwrap();
            task_list[index] = self.cur_task.clone();
        }

        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match self.current_screen {
                    CurrentScreen::Exiting => match key_event.code {
                        KeyCode::Char('y') => {
                            self.exit = true;
                        }
                        KeyCode::Char('n') => {
                            self.current_screen = CurrentScreen::Main;
                        }
                        _ => {}
                    },
                    CurrentScreen::Editing => match key_event.code {
                        KeyCode::Char('w') => {
                            self.save_task();
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
                                match field.currently_editing {
                                    Some(TaskField::Title) => {
                                        self.cur_task.title.push(val);
                                    }
                                    Some(TaskField::Description) => {
                                        self.cur_task.description.push(val);
                                    }
                                    Some(TaskField::Due) => {
                                        self.cur_task.due = chrono::Local::now();
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
                            self.current_screen = CurrentScreen::Exiting;
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

