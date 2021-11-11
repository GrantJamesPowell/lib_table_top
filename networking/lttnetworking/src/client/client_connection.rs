use crate::connection::ConnectionIO;
use crate::messages::Closed;
use crate::messages::hello::{ClientHello, ServerHello};
use crate::messages::mode::Mode;
use crate::{Token, User};

pub async fn authenticate(
    credentials: Token,
    conn: &mut impl ConnectionIO,
) -> Result<User, Closed> {
    conn.send(ClientHello { credentials }).await?;
    conn.next::<Result<ServerHello, Closed>>.await??;
}

pub async fn choose_mode(mode: Mode) -> Result<(), Closed> {
    conn.send(mode).await?;
    let result: Result<Mode, Closed> = conn.next::<Result<Mode, Closed>>().await?;
}
