use crate::connection::ConnectionIO;
use crate::{Token, User, SupportedGames};
use lttruntime::messages::{
    Closed, 
    mode::{Mode::*, JoinAs},
    hello::{ClientHello, ServerHello}
};

pub async fn observe_game<SG: SupportedGames>(
    token: Token,
    game_id: GameId,
    conn: &mut impl ConnectionIO,
) -> Result<(), Closed> {
    let user = authenticate(conn, token)?;
    choose_mode(
        conn,
        JoinInProgressGame((SG, game_id), JoinAs::Observer)
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

pub async fn choose_mode<SG: SupportedGames>(
    conn: &mut impl ConnectionIO,
    mode: Mode<SG>
) -> Result<(), Closed> {
    conn.send(mode).await?;
    conn.next::<Result<Mode<SG>, Closed>>().await
}
