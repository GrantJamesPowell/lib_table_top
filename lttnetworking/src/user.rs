use serde::{Deserialize, Serialize};
use lttcore::id::UserId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub user_id: UserId,
}
