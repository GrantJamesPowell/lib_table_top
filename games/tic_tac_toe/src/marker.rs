#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Marker {
    X,
    O,
}


impl Marker {
    pub fn opponent(&self) -> Self {
        use Marker::*;

        match self {
            X => O,
            O => X,
        }
    }
}
