/// Helper function to have `tic_tac_toe::Board` literals
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
/// let board = ttt!([
///   - O -
///   - O -
///   X - X
/// ]);
///
/// assert_eq!(board.at((0, 0)), Some(X));
/// assert_eq!(board.at((1, 1)), Some(O));
/// assert_eq!(board.at((1, 2)), Some(O));
/// assert_eq!(board.at((0, 2)), None);
/// assert_eq!(board.at((2, 0)), Some(X));
/// assert_eq!(board.at((2, 2)), None);
/// ```
#[macro_export]
macro_rules! ttt {
    ([ $a:tt $b:tt $c:tt $d:tt $e:tt $f:tt $g:tt $h:tt $i:tt ]) => {
        $crate::examples::tic_tac_toe::Board::from([
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