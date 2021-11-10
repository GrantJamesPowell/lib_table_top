use crate::auth::Auth;
use crate::connection::ConnectionIO;
use crate::messages::hello::{ClientHello, ServerHello};
use crate::messages::Closed;
use crate::User;

pub async fn authorize(auth: &mut impl Auth, conn: &mut impl ConnectionIO) -> Result<User, Closed> {
    let ClientHello { credentials } = conn.next::<ClientHello>().await?;

    match auth.authorize(credentials).await {
        Some(user) => {
            let hello: Result<ServerHello, Closed> = Ok(ServerHello { user: user.clone() });
            conn.send(hello).await?;
            Ok(user)
        }
        None => {
            let err: Result<ServerHello, Closed> = Err(Closed::Unauthorized);
            conn.send(err).await?;
            Err(Closed::Unauthorized)
        }
    }
}
