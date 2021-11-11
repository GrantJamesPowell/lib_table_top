use crate::messages::mode::Mode;
use crate::{SupportedGames, Token, User};
use async_trait::async_trait;

#[async_trait]
pub trait Authenticate {
    async fn authenticate(&mut self, token: &Token) -> Option<User>;
}

#[async_trait]
pub trait Authorize<SG: SupportedGames> {
    async fn authorize(&mut self, user: &User, mode: &Mode<SG>) -> bool;
}
