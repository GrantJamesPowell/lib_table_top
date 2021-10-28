use crate::{Play, Seed};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct Scenario<T: Play> {
    settings: Arc<<T as Play>::Settings>,
    initial_state: Arc<T>,
    seed: Arc<Seed>,
}
