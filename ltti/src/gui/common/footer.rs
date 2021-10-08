use tui::layout::Alignment;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, BorderType, Borders, Paragraph, Widget};

pub fn footer() -> impl Widget {
    Paragraph::new("Lib Table Top Interactive (c) - (ltti)")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded),
        )
}
