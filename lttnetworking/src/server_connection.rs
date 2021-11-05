use futures_util::Stream;
use lttcore::{Play, TurnNum};

pub enum ClientMsgs {
    SubmitAction { turn_num: TurnNum },
}

pub enum ServerConnectionMsg {}

pub async fn server_connection<T: Play>(_client_msgs: impl Stream<Item = ClientMsgs>) {
    todo!()
}
