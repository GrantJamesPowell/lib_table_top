use lttcore::Play;
use tui::backend::Backend;
use tui::terminal::Frame;
use tui::layout::Rect;

pub trait GameUserInterface<B: Backend> {
    type Game : Play;
    type State;

    fn action_request(
        frame: &mut Frame<B>,
        rect: Rect,
        player_view: <Self::Game as Play>::PlayerView,
        spectator_view: <Self::Game as Play>::SpectatorView,
        action_request: <Self::Game as Play>::ActionRequest,
        submit: impl FnOnce(<Self::Game as Play>::Action),
    );
}
