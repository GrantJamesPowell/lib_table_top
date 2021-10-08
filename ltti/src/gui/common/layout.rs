use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};

pub fn layout() -> Layout {
    Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(2)].as_ref())
}
