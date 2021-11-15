pub mod match_maker;
pub use match_maker::run_match_maker;

mod channels;
pub use channels::{GameRequestTicket, MatchMakerRequestReceiver, MatchMakerRequestSender};
