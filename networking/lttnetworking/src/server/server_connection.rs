use crate::auth::{Authenticate, Authorize};
use crate::connection::ConnectionIO;
use crate::messages::hello::{ClientHello, ServerHello};
use crate::messages::mode::Mode;
use crate::messages::Closed;
use crate::User;

pub async fn server_connection(
    _authenticate: &mut impl Authenticate,
    _authorize: &mut impl Authorize,
    _conn: &mut impl ConnectionIO,
) {
    todo!()
}

pub async fn authenticate(
    auth: &mut impl Authenticate,
    conn: &mut impl ConnectionIO,
) -> Result<User, Closed> {
    let ClientHello { credentials } = conn.next::<ClientHello>().await?;

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

pub async fn choose_and_authorize_mode(
    user: &User,
    auth: &mut impl Authorize,
    conn: &mut impl ConnectionIO,
) -> Result<Mode, Closed> {
    let mode: Mode = conn.next::<Mode>().await?;

    if auth.authorize(user, &mode).await {
        let msg: Result<Mode, Closed> = Ok(mode);
        conn.send(msg.clone()).await?;
        msg
    } else {
        let msg: Result<Mode, Closed> = Err(Closed::Unauthorized);
        conn.send(msg.clone()).await?;
        msg
    }
}
