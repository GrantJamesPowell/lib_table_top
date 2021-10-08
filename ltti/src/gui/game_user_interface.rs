use crossterm::event::KeyEvent;
use lttcore::Play;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;

pub trait UserInterfaceState {
    fn on_input(&mut self, event: KeyEvent) {}
}

pub trait GameUserInterface<B: Backend>: Play {
    type UIState: UserInterfaceState + Default;

    fn render_action_request(
        frame: &mut Frame<B>,
        rect: Rect,
        ui_state: &Self::UIState,
        settings: &<Self as Play>::Settings,
        player_view: &<Self as Play>::PlayerView,
        spectator_view: &<Self as Play>::SpectatorView,
        action_request: &<Self as Play>::ActionRequest,
    );
}
