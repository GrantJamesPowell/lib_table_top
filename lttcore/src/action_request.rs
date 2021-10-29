use crate::{Play, Player};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionRequest<'a, T: Play> {
    pub turn_num: u64,
    pub player: Player,
    pub settings: &'a <T as Play>::Settings,
    pub secret_info: &'a <T as Play>::PlayerSecretInfo,
    pub public_info: &'a <T as Play>::PublicInfo,
}

pub trait ActionRequestSource<T: Play> {
    fn next_action_request(&self) -> Option<ActionRequest<'_, T>>;
}
