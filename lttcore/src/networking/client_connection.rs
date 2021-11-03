use super::messages::{ClientMsg, ServerMsg};
use crate::Play;
use std::error::Error;

enum Status {
    Authorizing,
    InGame,
}

pub struct ClientConnection<T> {
    status: Status,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Play> ClientConnection<T> {
    pub fn receive_msg(&self, msg: ServerMsg<T>) -> Result<Option<ClientMsg<T>>, Box<dyn Error>> {
        match msg {
            ServerMsg::Ping(msg) => Ok(Some(msg.opposite().into())),
            _ => {
                todo!()
            }
        }
    }
}
