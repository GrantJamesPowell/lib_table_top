use crate::messages::{ClientMsg, ServerMsg};
use lttcore::Play;
use std::error::Error;

pub struct ServerConnection<T> {
    authorized: bool,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Play> ServerConnection<T> {
    pub fn receive_msg(&self, msg: ClientMsg<T>) -> Result<Option<ServerMsg<T>>, Box<dyn Error>> {
        if let ClientMsg::Ping(msg) = msg {
            return Ok(Some(msg.opposite().into()));
        }

        todo!()
    }
}
