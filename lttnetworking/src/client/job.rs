use crate::connection::SubConnection;
use crate::messages::conn_ctrl::SubConnMode;
use async_trait::async_trait;

#[async_trait]
pub trait Job {
    async fn run(self, sub_conn: SubConnection);
    fn game_type(&self) -> &'static str;
    fn sub_conn_mode(&self) -> SubConnMode;
}
