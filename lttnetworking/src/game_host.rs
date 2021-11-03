use lttcore::Play;

pub struct GameHost<T: Play> {
    phantom: std::marker::PhantomData<T>,
}
