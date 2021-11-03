use crate::{Token, User};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientHello {
    pub credentials: Token,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerHello<'a> {
    pub user: Cow<'a, User>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum JoinError {
    UnparseableClientHello,
    Unauthorized,
    UnsupportedVersion,
}

#[async_trait]
pub trait Auth {
    async fn authorize(token: Token) -> Option<User>;
}

pub async fn process_client_hello<Authorizer: Auth>(
    data: &[u8],
    mut output: Vec<u8>,
) -> Result<(User, Vec<u8>), Vec<u8>> {
    output.clear();

    match bincode::deserialize::<ClientHello>(data) {
        Ok(ClientHello { credentials }) => match Authorizer::authorize(credentials).await {
            Some(user) => {
                bincode::serialize_into(
                    &mut output,
                    &ServerHello {
                        user: Cow::Borrowed(&user),
                    },
                )
                .expect("can't fail serializing a server hello");
                return Ok((user, output));
            }
            None => {
                bincode::serialize_into(&mut output, &JoinError::Unauthorized)
                    .expect("can't fail serializing a join error");
                return Err(output);
            }
        },
        Err(_err) => {
            bincode::serialize_into(&mut output, &JoinError::UnparseableClientHello)
                .expect("can't fail serializing a join error");
            return Err(output);
        }
    }
}
