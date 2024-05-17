use ratatui::prelude::{Buffer, Rect};
use ratatui::layout::{Layout, Alignment, Constraint, Direction};
use ratatui::symbols::border;
use ratatui::style::*;
use ratatui::Frame;
use ratatui::text::{Line, Text};
use ratatui::widgets::{block::title::Title, block::Position, Block, Borders, Paragraph, Widget};

use crate::App;

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

impl Widget for &App {
   fn render(self, area: Rect, buf: &mut Buffer) {

    } 
}