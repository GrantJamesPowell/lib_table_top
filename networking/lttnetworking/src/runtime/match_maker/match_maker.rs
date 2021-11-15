use lttcore::play::{LttSettings, Mode};
use std::collections::HashMap;
use smallvec::SmallVec;
use super::messages::MatchMakerRequestReceiver;

pub async fn run_match_maker<T: LttSettings>(
    mailbox: MatchMakerRequestReceiver<T>
) {
    todo!()
}
