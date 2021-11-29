use crate::connection::ConnectionIO;
use crate::messages::closed::Closed;
use crate::messages::conn_ctrl::{JoinAs, SubConnMode};
use lttcore::play::Play;
use lttruntime::Runtime;
use std::sync::Arc;

pub async fn run_server_sub_conn<T: Play, C: ConnectionIO>(
    mut conn: C,
    runtime: Arc<Runtime<T>>,
) -> Result<(), Closed> {
    match conn.next::<SubConnMode>().await? {
        SubConnMode::JoinGame(game_id, JoinAs::Observer) => {
            let _observer_connection = runtime
                .observe_game(game_id, conn.encoding())
                .ok_or_else(|| Closed::ClientError(format!("{:?} not found", game_id)));

            todo!()
        }
        SubConnMode::JoinGame(game_id, JoinAs::Player(player)) => {
            let _player_connection = runtime
                .play_game(game_id, player, conn.encoding())
                .ok_or_else(|| Closed::ClientError(format!("{:?} not found", game_id)));

            todo!()
        }
    }
}
