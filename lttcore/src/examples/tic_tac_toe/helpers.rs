use crate::Player;

/// Helper function to have tic tac toe game literals
///
/// ## Notes
///
/// Visually ttt! uses Cartesian coordinates for representing games ((0, 0) is in the bottom left
/// corner). In memory the actual `TicTacToe::Board` struct represents the board with (0, 0) in the
/// top left corner.
/// ```
/// use lttcore::ttt;
/// use lttcore::examples::tic_tac_toe::Marker::*;
///
/// let game = ttt!([
///   X O -
///   O O -
///   X - X
/// ]);
///
/// assert_eq!(game.at((0, 0)).unwrap(), X);
/// assert_eq!(game.at((1, 1)).unwrap(), O);
/// assert_eq!(game.at((2, 2)), None);
/// assert!(game.resigned().is_empty());
/// ```
#[macro_export]
macro_rules! ttt {
    ([ $a:tt $b:tt $c:tt $d:tt $e:tt $f:tt $g:tt $h:tt $i:tt ]) => {
        $crate::examples::TicTacToe::from_markers([
            [ttt!(@$g), ttt!(@$h), ttt!(@$i)],
            [ttt!(@$d), ttt!(@$e), ttt!(@$f)],
            [ttt!(@$a), ttt!(@$b), ttt!(@$c)],
        ])
    };
    (@-) => { None };
    (@X) => { Some($crate::examples::tic_tac_toe::Marker::X) };
    (@x) => { Some($crate::examples::tic_tac_toe::Marker::X) };
    (@O) => { Some($crate::examples::tic_tac_toe::Marker::O) };
    (@o) => { Some($crate::examples::tic_tac_toe::Marker::O) };
    ($_:tt) => {
        compile_error!("ttt! only accepts exactly nine of either X, O, -")
    };
}

/// Returns the opponent of a player in `TicTacToe`
///
/// ```
/// use lttcore::Player;
/// use lttcore::ttt;
/// use lttcore::examples::tic_tac_toe::{Marker::*, helpers::opponent};
///
/// let p0: Player = 0.into();
/// let p1: Player = 1.into();
///
/// assert_eq!(opponent(p0), p1);
/// assert_eq!(opponent(p1), p0);
/// assert_eq!(opponent(X), p1);
/// assert_eq!(opponent(O), p0);
/// ```
///
/// # Panics
///
/// This panics with a player not in [0, 1]
///
/// ```should_panic
/// use lttcore::Player;
/// use lttcore::examples::tic_tac_toe::helpers::opponent;
///
/// let p3: Player = 3.into();
/// opponent(p3);
/// ```
pub fn opponent(player: impl Into<Player>) -> Player {
    let player = player.into();
    match u8::from(player) {
        0 => 1.into(),
        1 => 0.into(),
        _ => panic!("Invalid Player for TicTacToe"),
    }
}
