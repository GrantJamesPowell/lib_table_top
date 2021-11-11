use crate::connection::ConnectionIO;
use crate::messages::Closed;
use crate::messages::hello::{ClientHello, ServerHello};
use crate::messages::mode::{Mode::*, JoinAs};
use crate::{Token, User};

pub async fn observe_game(
    token: Token,
    game_id: GameId,
    conn: &mut impl ConnectionIO,
) -> Result<(), Closed> {
    let user = authenticate(conn, token)?;
    choose_mode(
        conn,
        JoinInProgressGame(game_id, JoinAs::Observer)
    )?;
    todo!()
}

pub async fn authenticate(
    conn: &mut impl ConnectionIO,
    credentials: Token,
) -> Result<User, Closed> {
    conn.send(ClientHello { credentials }).await?;
    conn.next::<Result<ServerHello, Closed>>.await??;
}

pub async fn choose_mode(
    conn: &mut impl ConnectionIO,
    mode: Mode
) -> Result<(), Closed> {
    conn.send(mode).await?;
    conn.next::<Result<Mode, Closed>>().await
}
