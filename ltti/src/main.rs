#![allow(dead_code)]

use anyhow::Result;
use async_trait::async_trait;
use clap::{App, Arg, SubCommand};
use lttcore::encoder::json::PrettyJsonEncoder;
use lttcore::id::UserId;
use lttnetworking::auth::Authenticate;
use lttnetworking::example_supported_games::{
    ExampleSupportedGames, ExampleSupportedGamesRuntimes as Runtimes,
};
use lttnetworking::messages::hello::ServerInfo;
use lttnetworking::ws::server::accept_connection;
use lttnetworking::{Token, User};
use std::sync::Arc;
use tokio::net::TcpListener;

struct Auth;

#[async_trait]
impl Authenticate for Auth {
    async fn authenticate(&self, _token: &Token) -> Option<User> {
        Some(User {
            username: "GrantPowell".into(),
            user_id: UserId::new(),
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("LibTableTop Interactive (ltti)")
        .subcommand(
            SubCommand::with_name("server")
                .about("runs ltti as a lttserver")
                .arg(
                    Arg::with_name("PORT")
                        .short("p")
                        .long("port")
                        .takes_value(true)
                        .value_name("PORT")
                        .help("Sets the port to start the server on"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("server") {
        let port = matches
            .value_of("PORT")
            .and_then(|port| port.parse().ok())
            .unwrap_or(8080);

        println!("Starting server on port {}", port);

        let runtimes = Runtimes::<PrettyJsonEncoder>::init();
        let listener = TcpListener::bind(("localhost", port)).await?;

        let server_info = Arc::new(ServerInfo {
            max_sub_connections: 4,
        });

        while let Ok((stream, remote_addr)) = listener.accept().await {
            println!("Accepted Connection {:?}", remote_addr);

            tokio::spawn(accept_connection::<
                ExampleSupportedGames<PrettyJsonEncoder>,
                _,
                _,
            >(Auth, server_info.clone(), runtimes.clone(), stream));
        }
    }

    Ok(())
}
