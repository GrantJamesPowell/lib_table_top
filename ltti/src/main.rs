#![allow(dead_code)]

mod server;
use anyhow::Result;
use server::start_ws_server;

#[tokio::main]
async fn main() -> Result<()> {
    tokio::spawn(start_ws_server("127.0.0.1:8080")).await??;
    Ok(())
}
