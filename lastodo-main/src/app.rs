use std::hash::Hash;

use chrono::DateTime;
use color_eyre::{eyre::WrapErr, Result};
use crossterm::event;
use crossterm::event::{KeyCode, KeyEventKind};
use indexmap::IndexMap;

use crate::actions;
use crate::tui::Tui;
use std::cmp;

#[derive(Debug, Default, PartialEq)]
pub enum CurrentScreen {
    #[default]
    Main,
    Editing,
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

impl KanbanStatus {
    pub fn next(&self) -> KanbanStatus {
        match self {
            KanbanStatus::Todo => KanbanStatus::Partial,
            KanbanStatus::Partial => KanbanStatus::Doing,
            KanbanStatus::Doing => KanbanStatus::Done,
            KanbanStatus::Done => KanbanStatus::Blocked,
            KanbanStatus::Blocked => KanbanStatus::Todo,
        }
    }

    pub fn prev(&self) -> KanbanStatus {
        match self {
            KanbanStatus::Todo => KanbanStatus::Blocked,
            KanbanStatus::Partial => KanbanStatus::Todo,
            KanbanStatus::Doing => KanbanStatus::Partial,
            KanbanStatus::Done => KanbanStatus::Doing,
            KanbanStatus::Blocked => KanbanStatus::Done,
        }
    }
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

#[derive(PartialEq, Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct CurrentTask {
    pub status: KanbanStatus,
    pub index: i16,
}

#[derive(Clone, Debug)]
pub enum TaskEditMode {
    Normal,
    Insert,
}

#[derive(Clone, Debug)]
pub struct TaskEditState {
    pub currently_editing: Option<TaskField>,
    pub cur_value: String,
    pub is_new_task: bool,
    pub has_changed: bool,
    pub mode: TaskEditMode,
}

#[derive(Debug, Default)]
pub struct App {
    pub cur_task: Option<CurrentTask>, // The currently highlighted task
    pub task_list: IndexMap<KanbanStatus, Vec<Task>>, // The list of tasks
    pub current_screen: CurrentScreen, // The current screen
    pub currently_editing_task: Option<TaskEditState>, // The task currently being edited
    pub message: String,               // Status message
    pub exit: bool,                    // Whether to exit the application
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
        let cur_task = CurrentTask {
            status: KanbanStatus::Todo,
            index: 0,
        };
        task_list.insert(KanbanStatus::Todo, vec![task1.clone()]);
        task_list.insert(KanbanStatus::Partial, vec![task2]);
        task_list.insert(KanbanStatus::Doing, Vec::new());
        task_list.insert(KanbanStatus::Done, Vec::new());
        task_list.insert(KanbanStatus::Blocked, Vec::new());

        App {
            cur_task: Some(cur_task),
            task_list,
            current_screen: CurrentScreen::Main,
            currently_editing_task: None,
            message: String::new(),
            exit: false,
        }
    }

    pub fn get_cur_task(&self) -> Option<Task> {
        self.cur_task.as_ref()?;

        let cur_task_status = self.cur_task.as_ref().unwrap().status.clone();
        let cur_task_index = self.cur_task.as_ref().unwrap().index as usize;
        let cur_task = self.task_list[&cur_task_status][cur_task_index].clone();

        Some(cur_task)
    }

    pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }

        Ok(())
    }

    fn handle_normal_mode(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('i') => {
                self.currently_editing_task.as_mut().unwrap().mode = TaskEditMode::Insert;
            }
            KeyCode::Char('w') => {
                actions::save_task::save_task(self);
            }
            KeyCode::Char('q') => {
                actions::quit_editing::quit_editing(self);
            }
            KeyCode::Char('x') => {
                actions::force_quit_editing::force_quit_editing(self);
            }
            KeyCode::Tab => {
                actions::next_field::next_field(self);
            }
            KeyCode::BackTab => {
                actions::prev_field::prev_field(self);
            }
            _ => {}
        }
    }

    fn handle_insert_mode(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Esc => {
                self.currently_editing_task.as_mut().unwrap().mode = TaskEditMode::Normal;
            }
            KeyCode::Backspace => {
                if let Some(field) = &self.currently_editing_task.as_mut() {
                    let cur_task_status = self.cur_task.as_ref().unwrap().status.clone();
                    let cur_task_index = self.cur_task.as_ref().unwrap().index as usize;

                    match field.currently_editing {
                        Some(TaskField::Title) => {
                            self.task_list[&cur_task_status][cur_task_index].title.pop();
                        }
                        Some(TaskField::Description) => {
                            self.task_list[&cur_task_status][cur_task_index]
                                .description
                                .pop();
                        }
                        Some(TaskField::Due) => {}
                        None => {}
                    }
                }
            }
            KeyCode::Char(val) => {
                if let Some(field) = &self.currently_editing_task.as_mut() {
                    let cur_task_status = self.cur_task.as_ref().unwrap().status.clone();
                    let cur_task_index = self.cur_task.as_ref().unwrap().index as usize;

                    match field.currently_editing {
                        Some(TaskField::Title) => {
                            self.task_list[&cur_task_status][cur_task_index]
                                .title
                                .push(val);
                        }
                        Some(TaskField::Description) => {
                            self.task_list[&cur_task_status][cur_task_index]
                                .description
                                .push(val);
                        }
                        Some(TaskField::Due) => {
                            // TODO: Fix later
                            self.task_list[&cur_task_status][cur_task_index].due =
                                chrono::Local::now();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match self.current_screen {
                    CurrentScreen::Editing => {
                        match self.currently_editing_task.as_ref().unwrap().mode {
                            TaskEditMode::Normal => self.handle_normal_mode(key_event.code),
                            TaskEditMode::Insert => self.handle_insert_mode(key_event.code),
                        }
                    }
                    CurrentScreen::Main => match key_event.code {
                        KeyCode::Char('q') => {
                            self.exit = true;
                        }
                        KeyCode::Char('w') => {
                            // TODO
                        }
                        KeyCode::Char('h') => {
                            if let Some(cur_task) = &self.cur_task {
                                let cur_task_status = cur_task.status.clone();
                                let cur_task_index = cur_task.index;

                                let new_status = cur_task_status.prev();
                                let new_idx = cmp::min(
                                    cur_task_index,
                                    self.task_list[&new_status].len() as i16 - 1,
                                );

                                if new_idx == -1 {
                                    return Ok(());
                                }

                                self.cur_task = Some(CurrentTask {
                                    status: new_status,
                                    index: new_idx,
                                });
                            }
                        }
                        KeyCode::Char('l') => {
                            if let Some(cur_task) = &self.cur_task {
                                let cur_task_status = cur_task.status.clone();
                                let cur_task_index = cur_task.index;

                                let new_status = cur_task_status.next();
                                let new_idx = cmp::min(
                                    cur_task_index,
                                    self.task_list[&new_status].len() as i16 - 1,
                                );

                                if new_idx == -1 {
                                    return Ok(());
                                }

                                self.cur_task = Some(CurrentTask {
                                    status: new_status,
                                    index: new_idx,
                                });
                            }
                        }
                        KeyCode::Char('j') => {
                            /* Yes, I know j moves down in vim. I prefer it the other way around.
                             * It's my app, my rules. (But I'll probably make this configurable) */
                            if let Some(cur_task) = &self.cur_task {
                                let cur_task_status = cur_task.status.clone();
                                let cur_task_index = cur_task.index;
                                let num_tasks = self.task_list[&cur_task_status].len() as i16;

                                let new_idx = (cur_task_index - 1) % num_tasks;
                                let new_status = cur_task_status;

                                if new_idx >= self.task_list[&new_status].len() as i16 {
                                    return Ok(());
                                }

                                self.cur_task = Some(CurrentTask {
                                    status: new_status,
                                    index: new_idx,
                                });
                            }
                        }
                        KeyCode::Char('k') => {
                            if let Some(cur_task) = &self.cur_task {
                                let cur_task_status = cur_task.status.clone();
                                let cur_task_index = cur_task.index;
                                let num_tasks = self.task_list[&cur_task_status].len() as i16;

                                let new_idx = (cur_task_index + 1) % num_tasks;
                                let new_status = cur_task_status;

                                if new_idx >= self.task_list[&new_status].len() as i16 {
                                    return Ok(());
                                }

                                self.cur_task = Some(CurrentTask {
                                    status: new_status,
                                    index: new_idx,
                                });
                            }
                        }
                        KeyCode::Char('i') => {
                            self.current_screen = CurrentScreen::Editing;

                            self.currently_editing_task =
                                if let Some(cur_task) = self.get_cur_task() {
                                    Some(TaskEditState {
                                        currently_editing: Some(TaskField::Title),
                                        cur_value: cur_task.title,
                                        is_new_task: false,
                                        has_changed: false,
                                        mode: TaskEditMode::Normal,
                                    })
                                } else {
                                    Some(TaskEditState {
                                        currently_editing: Some(TaskField::Title),
                                        cur_value: String::new(),
                                        is_new_task: true,
                                        has_changed: false,
                                        mode: TaskEditMode::Normal,
                                    })
                                }
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
