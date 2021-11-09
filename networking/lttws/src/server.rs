use futures_util::{SinkExt, StreamExt};
use lttnetworking::{
    interface::{ParseError, ReaderWriter},
    messages::{JoinError},
    Token, User,
};




use tokio::net::TcpStream;

use tokio_tungstenite::tungstenite::Message;


async fn data_to_ws_message(
    data: Vec<u8>,
) -> Result<Message, tokio_tungstenite::tungstenite::Error> {
    Ok(Message::binary(data))
}

async fn accept_connection(
    stream: TcpStream,
    _authorize: impl FnMut(Token) -> Result<User, JoinError>,
) -> anyhow::Result<()> {
    let ws = tokio_tungstenite::accept_async(stream).await?;
    let (write, read) = ws.split();

    let _rw = ReaderWriter {
        read: read.map(|msg| msg.map(|msg| msg.into_data()).map_err(|_| ParseError)),
        write: write.with(data_to_ws_message),
    };
    Ok(())
}

// async fn run_authorize<S>(
//     ws: &mut WebSocketStream<S>,
//     mut authorize: impl FnMut(Token) -> Result<User, JoinError>
// ) -> Option<User> where S: AsyncRead + AsyncWrite + Unpin {
//     while let Some(Ok(msg)) = ws.next().await {
//         match bincode::deserialize::<ClientHello>(&msg.into_data()) {
//             Ok(ClientHello { credentials }) => {
//                 match authorize(credentials) {
//                     Ok(user) => return Some(user),
//                     Err(err) => send(ws, &err)
//                 };
//             }
//             Err(_) => {
//                 send(ws, &JoinError::UnparseableClientHello).await?;
//             }
//         }
//     };
//
//     None
// }
//
// async fn send<T: Serialize, S: AsyncRead + AsyncWrite + Unpin>(ws: &mut WebSocketStream<S>, msg: &T) -> anyhow::Result<()> {
//     let msg = bincode::serialize(msg).unwrap();
//     Ok(ws.send(Message::binary(msg)).await?)
// }
