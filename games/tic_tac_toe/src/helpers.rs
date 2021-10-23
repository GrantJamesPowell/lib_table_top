use lttcore::Player;

#[macro_export]
macro_rules! ttt {
    ([ $a:tt $b:tt $c:tt $d:tt $e:tt $f:tt $g:tt $h:tt $i:tt ]) => {
        ::tic_tac_toe::TicTacToe::from(
            ::tic_tac_toe::Board::from_ints([
                [ttt!(@$a), ttt!(@$b), ttt!(@$c)],
                [ttt!(@$d), ttt!(@$e), ttt!(@$f)],
                [ttt!(@$g), ttt!(@$h), ttt!(@$i)]
            ])
        )
    };
    (@-) => { 0 };
    (@X) => { 1 };
    (@x) => { 1 };
    (@O) => { 2 };
    (@o) => { 2 };
    ($_:tt) => {
        compile_error!("ttt! only accepts exactly nine of either X, O, -")
    };
}

/// Returns the opponent of a player in TicTacToe
///
/// ```
/// use lttcore::Player;
/// use tic_tac_toe::{Marker::*, helpers::opponent};
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
/// use tic_tac_toe::helpers::opponent;
///
/// let p3: Player = 3.into();
/// opponent(p3);
/// ```
pub fn opponent(player: impl Into<Player>) -> Player {
    match player.into().as_u8() {
        0 => 1.into(),
        1 => 0.into(),
        _ => panic!("Invalid Player for TicTacToe"),
    }
}
