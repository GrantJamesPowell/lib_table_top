use crate::connection::ConnectionIO;
use crate::messages::closed::Closed;
use crate::messages::conn_ctrl::{JoinAs, SubConnMode};
use lttcore::{encoder::Encoder, Play};
use lttruntime::Runtime;
use std::sync::Arc;

pub async fn run_server_sub_conn<T: Play, E: Encoder, C: ConnectionIO<E>>(
    mut conn: C,
    runtime: Arc<Runtime<T, E>>,
) -> Result<(), Closed> {
    match conn.next::<SubConnMode>().await? {
        SubConnMode::JoinGame(game_id, JoinAs::Observer) => {
            let _observer_connection = runtime
                .observe_game(game_id)
                .ok_or_else(|| Closed::ClientError(format!("{:?} not found", game_id)));

            todo!()
        }
        SubConnMode::JoinGame(game_id, JoinAs::Player(player)) => {
            let _player_connection = runtime
                .play_game(game_id, player)
                .ok_or_else(|| Closed::ClientError(format!("{:?} not found", game_id)));

            todo!()
        }
    }
}
