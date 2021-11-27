use lttcore::id::UserId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchMakerRequest {
    user_id: UserId,
}
