use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Rect;
use ratatui::style::*;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Row, Table};
use ratatui::Frame;

use crate::app::{App, CurrentScreen, TaskEditMode, TaskEditState, TaskField};

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        // Overall layout. A header with a title, the main area, and a status bar
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(frame.size());

        // Set up the title section
        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());
        let title = Paragraph::new("Lastodo").block(title_block);
        frame.render_widget(title, chunks[0]);

        // Set up the main section
        let width: f64 = 100.0 / self.task_list.len() as f64;
        let constraints = vec![Constraint::Percentage(width.floor() as u16); self.task_list.len()];

        let table_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(&constraints)
            .split(chunks[1]);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        for (i, (status, tasks)) in self.task_list.iter().enumerate() {
            let rows = tasks.iter().map(|task| {
                let cur_task = self.get_cur_task();
                let style = if cur_task.is_none() || cur_task.as_ref().unwrap() == task {
                    active_style
                } else {
                    Style::default()
                };

                Row::new(vec![task.title.clone()]).style(style)
            });

            let table = Table::new(rows, &constraints)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(status.to_string()),
                )
                .widths([
                    Constraint::Percentage(40),
                    Constraint::Percentage(40),
                    Constraint::Percentage(20),
                ]);

            frame.render_widget(table, table_chunks[i]);
        }

        let cur_nav_text = {
            if let Some(cur_task) = self.get_cur_task() {
                format!(
                    "{} | Due: {}",
                    cur_task.title,
                    cur_task.due.format("%Y-%m-%d")
                )
            } else {
                "No task selected.".to_string()
            }
        };
        let mode_footer =
            Paragraph::new(Line::from(cur_nav_text)).block(Block::default().borders(Borders::ALL));

        let key_hints = {
            match self.current_screen {
                CurrentScreen::Main => vec![
                    Span::styled("(q)uit", Style::default().fg(Color::White)),
                    Span::styled(" | ", Style::default().fg(Color::White)),
                    Span::styled("(e)dit Task", Style::default().fg(Color::White)),
                    Span::styled(" | ", Style::default().fg(Color::White)),
                    Span::styled("h/j/k/l: Move", Style::default().fg(Color::White)),
                    Span::styled(" | ", Style::default().fg(Color::White)),
                    Span::styled("(a)dd Task", Style::default().fg(Color::White)),
                ],
                CurrentScreen::Editing => vec![
                    Span::styled("(q)uit", Style::default().fg(Color::White)),
                    Span::styled(" | ", Style::default().fg(Color::White)),
                    Span::styled("(s)ave", Style::default().fg(Color::White)),
                    Span::styled(" | ", Style::default().fg(Color::White)),
                    Span::styled("(c)ancel", Style::default().fg(Color::White)),
                ],
            }
        };
        let key_hints_footer =
            Paragraph::new(Line::from(key_hints)).block(Block::default().borders(Borders::ALL));

        let footer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[2]);

        frame.render_widget(mode_footer, footer_chunks[0]);
        frame.render_widget(key_hints_footer, footer_chunks[1]);

        if self.current_screen == CurrentScreen::Editing {
            let area = centered_rect(60, 50, frame.size());
            let gray_background = Style::default().bg(Color::Black);
            frame.render_widget(Block::default().style(gray_background), area);

            let new_state = TaskEditState {
                currently_editing: Some(TaskField::Title),
                cur_value: String::new(),
                is_new_task: true,
                has_changed: true,
                mode: TaskEditMode::Normal,
            };
            let state = self.currently_editing_task.as_ref().unwrap_or(&new_state);
            self.currently_editing_task = Some(state.clone());

            let currently_editing_field: TaskField = self
                .currently_editing_task
                .as_ref()
                .unwrap()
                .currently_editing
                .clone()
                .unwrap();

            let area = centered_rect(60, 50, frame.size());
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Percentage(80),
                    Constraint::Length(3),
                ])
                .split(area);

            let title_block = Block::default().title("Title").borders(Borders::ALL).style(
                if currently_editing_field == TaskField::Title {
                    active_style
                } else {
                    Style::default()
                },
            );
            let desc_block = Block::default()
                .title("Description")
                .borders(Borders::ALL)
                .style(if currently_editing_field == TaskField::Description {
                    active_style
                } else {
                    Style::default()
                });
            let due_block = Block::default().title("Due").borders(Borders::ALL).style(
                if currently_editing_field == TaskField::Due {
                    active_style
                } else {
                    Style::default()
                },
            );

            let cur_task = self.get_cur_task().unwrap();

            let title_text = Paragraph::new(cur_task.title).block(title_block);
            frame.render_widget(title_text, chunks[0]);

            let desc_text = Paragraph::new(cur_task.description).block(desc_block);
            frame.render_widget(desc_text, chunks[1]);

            let due_text = Paragraph::new(cur_task.due.to_string()).block(due_block);
            frame.render_widget(due_text, chunks[2]);
        }
    }
}
