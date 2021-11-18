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
use lttnetworking::ws::client::run_jobs;
use lttnetworking::ws::server::accept_connection;
use lttnetworking::{Token, User};
use std::sync::Arc;
use tokio::net::TcpListener;
use url::Url;

struct Auth;
type Games = ExampleSupportedGames<PrettyJsonEncoder>;

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
            SubCommand::with_name("connect")
                .about("connect to a running lttserver")
                .arg(
                    Arg::with_name("SERVER")
                        .short("s")
                        .long("server")
                        .takes_value(true)
                        .help("Which server to connect to (defaults to ws://localhost:8080)"),
                )
                .arg(
                    Arg::with_name("TOKEN")
                        .short("t")
                        .required(true)
                        .long("token")
                        .takes_value(true)
                        .help("Token used to connect to the server"),
                )
                .subcommand(SubCommand::with_name("whoami").about(
                    "Attempts to authenticate to the server and prints information about the user",
                ))
                .subcommand(
                    SubCommand::with_name("server-version")
                        .about("Connects to server and prints version info"),
                ),
        )
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

    if let Some(matches) = matches.subcommand_matches("connect") {
        // TODO: Load config from a file??
        let token = matches
            .value_of("TOKEN")
            .and_then(|token| token.parse().ok())
            .expect("TOKEN is required");

        let server = matches
            .value_of("SERVER")
            // TODO: // Handle bad urls
            .map(|server| Url::parse(server).unwrap())
            .unwrap_or(Url::parse("localhost:8080").unwrap());

        if matches.subcommand_matches("whoami").is_some() {
            let jobs = [].into_iter();
            run_jobs::<PrettyJsonEncoder, _>(server, token, 1, jobs).await?;
        };
    };

    if let Some(matches) = matches.subcommand_matches("server") {
        let port = matches
            .value_of("PORT")
            // TODO: // Handle invalid ports
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

            tokio::spawn(accept_connection::<Games, _, _>(
                Auth,
                server_info.clone(),
                runtimes.clone(),
                stream,
            ));
        }
    }

    Ok(())
}
