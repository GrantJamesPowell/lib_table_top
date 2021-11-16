use crate::connection::ConnectionIO;
use crate::messages::closed::Closed;
use crate::messages::conn_ctrl::SubConnMode;
use lttcore::{encoder::Encoder, Play};
use lttruntime::Runtime;
use std::sync::Arc;

pub async fn run_server_sub_conn<T: Play, E: Encoder, C: ConnectionIO<E>>(
    mut conn: C,
    _runtime: Arc<Runtime<T, E>>,
) -> Result<(), Closed> {
    let _mode = conn.next::<SubConnMode>().await?;

    Ok(())
}
