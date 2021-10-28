use crate::{Play, Seed};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct Scenario<T: Play> {
    pub(crate) settings: Arc<<T as Play>::Settings>,
    pub(crate) initial_state: Arc<T>,
    pub(crate) seed: Arc<Seed>,
}
