use crate::auth::{Authenticate, Authorize};
use crate::connection::ConnectionIO;
use crate::messages::hello::{ClientHello, ServerHello};
use crate::messages::mode::Mode;
use crate::messages::Closed;
use crate::{SupportedGames, User};

pub async fn server_connection<SG: SupportedGames>(
    _authenticate: &mut impl Authenticate,
    _authorize: &mut impl Authorize<SG>,
    _conn: &mut impl ConnectionIO,
) {
    todo!()
}

pub async fn authenticate(
    auth: &mut impl Authenticate,
    conn: &mut impl ConnectionIO,
) -> Result<User, Closed> {
    let ClientHello { credentials } = conn.next().await?;

    match auth.authenticate(&credentials).await {
        Some(user) => {
            let hello: Result<ServerHello, Closed> = Ok(ServerHello { user: user.clone() });
            conn.send(hello).await?;
            Ok(user)
        }
        None => {
            let err: Result<ServerHello, Closed> = Err(Closed::InvalidCredentials);
            conn.send(err).await?;
            Err(Closed::InvalidCredentials)
        }
    }
}

pub async fn choose_and_authorize_mode<SG: SupportedGames>(
    user: &User,
    auth: &mut impl Authorize<SG>,
    conn: &mut impl ConnectionIO,
) -> Result<Mode<SG>, Closed> {
    let mode: Mode<SG> = conn.next().await?;

    if auth.authorize(user, &mode).await {
        let msg: Result<Mode<SG>, Closed> = Ok(mode);
        conn.send(msg.clone()).await?;
        msg
    } else {
        let msg: Result<Mode<SG>, Closed> = Err(Closed::Unauthorized);
        conn.send(msg.clone()).await?;
        msg
    }
}
