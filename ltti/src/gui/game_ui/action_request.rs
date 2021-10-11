use crossterm::event::KeyEvent;
use lttcore::{Play, Player};
use std::sync::mpsc::Sender;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;

pub trait ActionRequestState<T: Play> {
    fn on_input(
        &mut self,
        event: KeyEvent,
        player: Player,
        player_view: &<T as Play>::PlayerView,
        spectator_view: &<T as Play>::SpectatorView,
        action_request: &<T as Play>::ActionRequest,
        settings: &<T as Play>::Settings,
        send_action: impl FnOnce(<T as Play>::Action),
    );
}

pub trait ActionRequestInterface<B: Backend>: Play {
    type UIState: ActionRequestState<Self> + Default;

    fn render_action_request(
        frame: &mut Frame<B>,
        rect: Rect,
        player: Player,
        player_view: &<Self as Play>::PlayerView,
        spectator_view: &<Self as Play>::SpectatorView,
        action_request: &<Self as Play>::ActionRequest,
        settings: &<Self as Play>::Settings,
        ui_state: &Self::UIState,
    );
}
