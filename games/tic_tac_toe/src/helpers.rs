#[macro_export]
macro_rules! ttt {
    ([ $a:tt $b:tt $c:tt $d:tt $e:tt $f:tt $g:tt $h:tt $i:tt ]) => {
        ::tic_tac_toe::Board::from_ints([
            [ttt!(@$a), ttt!(@$b), ttt!(@$c)],
            [ttt!(@$d), ttt!(@$e), ttt!(@$f)],
            [ttt!(@$g), ttt!(@$h), ttt!(@$i)]
        ])
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
