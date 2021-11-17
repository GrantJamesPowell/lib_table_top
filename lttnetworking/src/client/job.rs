use crate::connection::SubConnection;
use crate::messages::conn_ctrl::SubConnMode;
use async_trait::async_trait;
use lttcore::encoder::Encoder;

#[async_trait]
pub trait Job<E: Encoder> {
    async fn run(self, sub_conn: SubConnection<E>);
    fn game_type(&self) -> &'static str;
    fn sub_conn_mode(&self) -> SubConnMode;
}
