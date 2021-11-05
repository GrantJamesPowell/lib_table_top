use lttcore::utilities::PlayerIndexedData as PID;
use lttcore::Play;
use smallvec::SmallVec;
use tokio::sync::mpsc::Sender;

type Connections<Values> = SmallVec<[Sender<Values>; 8]>;

struct Runtime<T: Play> {
    players: PID<Connections<()>>,
    observers: PID<Connections<()>>,
    phantom: std::marker::PhantomData<T>,
}
