use crate::gui::{GameUserInterface, UserInterfaceState};
use lttcore::Play;
use tic_tac_toe::{
    Marker::{self, *},
    TicTacToe,
};

use tui::backend::Backend;
use tui::layout::{Alignment, Constraint::*, Direction::*, Layout, Margin, Rect};
use tui::style::{Color::*, Style};
use tui::widgets::{Block, BorderType::*, Borders, Widget};
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
            .border_type(Rounded);

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
            for (row_num, square) in col.into_iter().enumerate().rev() {
                let pos = (col_num, row_num);
                draw_square(frame, square, spectator_view.board().at(pos), pos);
            }
        }
    }
}

fn draw_square<B: Backend>(
    frame: &mut Frame<B>,
    square: Rect,
    marker: Option<Marker>,
    (col, row): (usize, usize),
) {
    let background_color = match marker {
        Some(X) => Red,
        Some(O) => Blue,
        None => Black,
    };

    let block = Block::default()
        .title(format!("{}, {}", col, row))
        .borders(Borders::ALL)
        .border_type(Plain)
        .border_style(Style::default().fg(White));

    let inner_rect = block.inner(square).inner(&Margin {
        horizontal: 1,
        vertical: 0,
    });

    frame.render_widget(block, square);

    let block = Block::default().style(Style::default().fg(White).bg(background_color));

    frame.render_widget(block, inner_rect);
}
