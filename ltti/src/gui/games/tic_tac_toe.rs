use crate::gui::game_ui::action_request::{ActionRequestInterface, ActionRequestState};
use std::sync::mpsc::Sender;

use crossterm::event::{KeyCode, KeyEvent};
use lttcore::{Play, Player};
use tic_tac_toe::{
    helpers::opponent,
    Action, Col, Row,
    Status::{self, *},
    TicTacToe,
};

use tui::backend::Backend;
use tui::layout::{Alignment, Constraint::*, Direction::*, Layout, Margin, Rect};
use tui::style::{Color::*, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType::*, Borders, Paragraph, Widget};
use tui::Frame;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct UIState {
    selected_row: Row,
    selected_col: Col,
}

impl ActionRequestState<TicTacToe> for UIState {
    fn on_input(
        &mut self,
        event: KeyEvent,
        _player: Player,
        _player_view: &<TicTacToe as Play>::PlayerSecretInfo,
        _public_info: &<TicTacToe as Play>::PublicInfo,
        _settings: &<TicTacToe as Play>::Settings,
        send_action: impl FnOnce(<TicTacToe as Play>::Action),
    ) {
        match event.code {
            KeyCode::Up => self.selected_row = self.selected_row.next(),
            KeyCode::Right => self.selected_col = self.selected_col.next(),
            KeyCode::Left => self.selected_col = self.selected_col.previous(),
            KeyCode::Down => self.selected_row = self.selected_row.previous(),
            KeyCode::Enter => send_action(Action {
                position: (self.selected_col, self.selected_row),
            }),
            _ => {}
        }
    }
}

impl<B: Backend> ActionRequestInterface<B> for TicTacToe {
    type UIState = UIState;

    fn render_action_request(
        frame: &mut Frame<B>,
        rect: Rect,
        _player: Player,
        _player_secret_information: &<Self as Play>::PlayerSecretInfo,
        public_info: &<Self as Play>::PublicInfo,
        _settings: &<Self as Play>::Settings,
        ui_state: &Self::UIState,
    ) {
        let board = Block::default()
            .title("Tic Tac Toe")
            .borders(Borders::ALL)
            .border_type(Rounded);

        let inner_rect = board.inner(rect).inner(&Margin {
            horizontal: 2,
            vertical: 1,
        });

        let layout = Layout::default()
            .direction(Vertical)
            .constraints([Min(20), Length(1)])
            .split(inner_rect);

        frame.render_widget(board, rect);
        frame.render_widget(status(public_info.status()), layout[1]);

        let cols = Layout::default()
            .direction(Horizontal)
            .constraints([Ratio(1, 3), Ratio(1, 3), Ratio(1, 3)])
            .split(layout[0]);

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
                draw_square(frame, square, public_info.at(pos), pos, is_selected);
            }
        }
    }
}

fn status(status: Status) -> impl Widget {
    match status {
        InProgress { next_up } => Paragraph::new(Spans::from(vec![
            styled_marker_span(next_up),
            Span::styled("'s turn", Style::default()),
        ])),
        Draw => Paragraph::new("Draw").style(Style::default()),
        WinByResignation { winner } => Paragraph::new(Spans::from(vec![
            styled_marker_span(winner),
            Span::raw(" wins because "),
            styled_marker_span(opponent(winner)),
            Span::raw(" resigned"),
        ])),
        Win { winner, positions } => Paragraph::new(Spans::from(vec![
            styled_marker_span(winner),
            Span::raw(" wins with positions "),
            Span::raw(format!(
                "{:?}",
                positions.map(|(c, r)| -> (u8, u8) { (c.into(), r.into()) })
            )),
        ])),
    }
}

fn styled_marker_span(player: Player) -> Span<'static> {
    let (color, text) = match player.as_u8() {
        0 => (Red, "X"),
        1 => (Blue, "O"),
        _ => panic!("Invalid player for tic tac toe!"),
    };

    Span::styled(
        text,
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    )
}

fn draw_square<B: Backend>(
    frame: &mut Frame<B>,
    square: Rect,
    player: Option<Player>,
    (col, row): (usize, usize),
    selected: bool,
) {
    let background_color = match player.map(|p| p.as_u8()) {
        Some(0) => Red,
        Some(1) => Blue,
        Some(_) => panic!("Invalid Player"),
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
