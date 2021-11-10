use crate::{Token, User};
use crate::messages::hello::{ClientHello, ServerHello};
use crate::messages::Closed;
use crate::connection::ConnectionIO;

pub async fn authorize(
    credentials: Token,
    conn: &mut impl ConnectionIO,
) -> Result<User, Closed> {
    conn.send(ClientHello { credentials }).await?;
    conn.next::<Result<ServerHello, Closed>>.await??;
}
