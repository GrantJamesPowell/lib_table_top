use crate::{Token, User};
use async_trait::async_trait;

#[async_trait]
pub trait Authenticate {
    async fn authenticate(&self, token: &Token) -> Option<User>;
}
