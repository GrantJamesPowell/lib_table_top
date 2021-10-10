use crate::gui::{GameUserInterface, UserInterfaceState};

use lttcore::Play;
use tic_tac_toe::{
    Col,
    Marker::{self, *},
    Row, TicTacToe,
};

use crossterm::event::{KeyCode, KeyEvent};
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint::*, Direction::*, Layout, Margin, Rect};
use tui::style::{Color::*, Style};
use tui::widgets::{Block, BorderType::*, Borders, Widget};
use tui::Frame;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct UIState {
    selected_row: Row,
    selected_col: Col,
}

impl UserInterfaceState for UIState {
    fn on_input(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Up => self.selected_row = self.selected_row.next(),
            KeyCode::Right => self.selected_col = self.selected_col.next(),
            KeyCode::Left => self.selected_col = self.selected_col.previous(),
            KeyCode::Down => self.selected_row = self.selected_row.previous(),
            _ => {}
        }
    }
}

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
            .constraints([Ratio(1, 3), Ratio(1, 3), Ratio(1, 3)])
            .split(inner_rect);

        let squares = cols.iter().map(|&column_rect| {
            Layout::default()
                .direction(Vertical)
                .constraints([Ratio(1, 3), Ratio(1, 3), Ratio(1, 3)])
                .split(column_rect)
        });

        for (col_num, col) in squares.into_iter().enumerate() {
            for (row_num, square) in col.into_iter().rev().enumerate() {
                let pos = (col_num, row_num);
                let is_selected =
                    pos == (ui_state.selected_col.into(), ui_state.selected_row.into());
                draw_square(
                    frame,
                    square,
                    spectator_view.board().at(pos),
                    pos,
                    is_selected,
                );
            }
        }
    }
}

fn draw_square<B: Backend>(
    frame: &mut Frame<B>,
    square: Rect,
    marker: Option<Marker>,
    (col, row): (usize, usize),
    selected: bool,
) {
    let background_color = match marker {
        Some(X) => Red,
        Some(O) => Blue,
        None => Black,
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(if selected { Thick } else { Plain })
        .border_style(Style::default().fg(if selected { Yellow } else { White }));

    let block = if selected {
        block.title(format!("{}, {}", col, row))
    } else {
        block
    };

    let inner_rect = block.inner(square).inner(&Margin {
        horizontal: 1,
        vertical: 0,
    });

    frame.render_widget(block, square);

    let block = Block::default().style(Style::default().fg(White).bg(background_color));

    frame.render_widget(block, inner_rect);
}