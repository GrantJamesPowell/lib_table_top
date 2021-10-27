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
        player_secret_info: &<T as Play>::PlayerSecretInfo,
        public_info: &<T as Play>::PublicInfo,
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
        player_secret_info: &<Self as Play>::PlayerSecretInfo,
        public_info: &<Self as Play>::PublicInfo,
        settings: &<Self as Play>::Settings,
        ui_state: &Self::UIState,
    );
}
