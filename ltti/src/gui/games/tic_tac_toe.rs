use tic_tac_toe::Marker::{self, *};
use tic_tac_toe::{Board, Col, Row, Settings, Status};
use tui::backend::Backend;
use tui::layout::Constraint;
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::text::Span;
use tui::widgets::{self, Block, BorderType, Borders, Cell, Table, Widget};

pub fn render(settings: &Settings, board: &Board) -> impl Widget {
    let table = (0..=2)
        .rev()
        .map(|col| {
            (0..=2).map(move |row| {
                let marker = board.at((col, row));
                make_cell((col, row), marker)
            })
        })
        .map(|cells| widgets::Row::new(cells).height(5).bottom_margin(1))
        .collect::<Vec<widgets::Row>>();

    Table::new(table)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .widths(&[
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
        ])
        .column_spacing(3)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Tic Tac Toe")
                .border_type(BorderType::Plain),
        )
}

fn make_cell((col, row): (usize, usize), marker: Option<Marker>) -> Cell<'static> {
    let marker_text = match marker {
        Some(X) => "X",
        Some(O) => "O",
        None => " ",
    };

    let contents = format!(
        concat!("{}\n\n", "  {}  \n\n", "{:?}",),
        col_and_row_to_phone_pad((col, row)),
        marker_text,
        (col, row)
    );

    let cell = Cell::from(contents);

    match marker {
        Some(X) => cell.style(Style::default().bg(Color::LightRed)),
        Some(O) => cell.style(Style::default().bg(Color::LightBlue)),
        None => cell.style(Style::default().bg(Color::DarkGray)),
    }
}

fn col_and_row_to_phone_pad(position: (usize, usize)) -> usize {
    4
}

// fn status_bar(status: &Status) -> impl Widget {
//     todo!()
// }
//
// fn board(board: &Board) -> impl Widget {
//     todo!()
// }
