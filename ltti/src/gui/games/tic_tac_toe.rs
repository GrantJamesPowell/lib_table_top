// use tic_tac_toe::Marker::{self, *};
// use tic_tac_toe::{Board, Col, Row, Settings, Status};
// use tui::backend::Backend;
// use tui::layout::Constraint;
// use tui::terminal::Frame;
// use tui::text::Span;
// use tui::widgets::{self, Block, BorderType, Borders, Cell, Table, Widget};

use crate::gui::{GameUserInterface, UserInterfaceState};
use lttcore::Play;
use tic_tac_toe::TicTacToe;

use tui::backend::Backend;
use tui::layout::{Alignment, Constraint::*, Direction::*, Layout, Margin, Rect};
use tui::style::{Color::*, Style};
use tui::widgets::{Block, BorderType::*, Borders};
use tui::Frame;

#[derive(Default)]
pub struct UIState {}

impl UserInterfaceState for UIState {}

impl<B: Backend> GameUserInterface<B> for TicTacToe {
    type UIState = UIState;

    fn render_action_request(
        frame: &mut Frame<B>,
        rect: Rect,
        ui_state: &UIState,
        settings: &<Self as Play>::Settings,
        player_view: &<Self as Play>::PlayerView,
        spectator_view: &<Self as Play>::SpectatorView,
        action_request: &<Self as Play>::ActionRequest,
        submit: impl FnOnce(<Self as Play>::Action),
    ) {
        let board = Block::default()
            .title("Tic Tac Toe")
            .borders(Borders::ALL)
            .border_type(Thick);

        let inner_rect = board.inner(rect).inner(&Margin {
            horizontal: 2,
            vertical: 1,
        });

        frame.render_widget(board, rect);

        let cols = Layout::default()
            .direction(Horizontal)
            .constraints([Percentage(33), Percentage(33), Percentage(33)])
            .split(inner_rect);

        let squares = cols.iter().map(|&column_rect| {
            Layout::default()
                .direction(Vertical)
                .constraints([Percentage(33), Percentage(33), Percentage(33)])
                .split(column_rect)
        });

        for (col_num, col) in squares.into_iter().enumerate() {
            for (row_num, square) in col.into_iter().rev().enumerate() {
                let x = format!("({},{})", col_num, row_num);

                let block = Block::default()
                    .title(x)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(White))
                    .style(Style::default().bg(Black));

                frame.render_widget(block, square);
            }
        }
    }
}
