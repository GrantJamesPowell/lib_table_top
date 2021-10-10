use crossterm::event::KeyEvent;
use lttcore::{Play, Player};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;

pub struct ActionRequestContext<T>
where
    T: Play,
{
    pub player: Player,
    pub player_view: <T as Play>::PlayerView,
    pub spectator_view: <T as Play>::SpectatorView,
    pub action_request: <T as Play>::ActionRequest,
}

pub trait ActionRequestState<T: Play> {
    fn on_input(
        &mut self,
        event: KeyEvent,
        context: &ActionRequestContext<T>,
        settings: &<T as Play>::Settings,
        submit_action: impl FnOnce(<T as Play>::Action) -> (),
    );
}

pub trait ActionRequestInterface<B: Backend>: Play {
    type UIState: ActionRequestState<Self> + Default;

    fn render_action_request(
        frame: &mut Frame<B>,
        rect: Rect,
        context: &ActionRequestContext<Self>,
        settings: &<Self as Play>::Settings,
        ui_state: &Self::UIState,
    );
}
