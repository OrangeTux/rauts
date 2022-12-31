use rauts::extract::{ChargerId, Request};
use rauts::handler::IntoHandler;
use rauts::middleware::Logger;
use rauts::ocpp::v16::{
    call::{Call, Heartbeat},
    call_result, Action,
};
use rauts::Router;

use chrono::prelude::*;
use std::io::Error as IoError;

use futures_util::{future, stream::TryStreamExt, SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};

/// A handler function must have 0 or more arguments. Each argument must implement
/// `rauts::extract::FromRequest`.
///
/// This handler function doesn't receive any arguments.
fn heartbeat() -> call_result::Heartbeat {
    call_result::Heartbeat {
        current_time: Utc::now(),
    }
}

//fn main() {
//let router: Router<Action> =
//Router::new().register(Logger(heartbeat.into_handler()), Action::Heartbeat);

//let call: Call = Heartbeat {}.into();
//let request = Request {
//call,
//charger_id: ChargerId("Alfen 123".to_string()),
//};

//dbg!(router.route(&request).unwrap());
//}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    // Create the event loop and TCP listener we'll accept connections on.
    let addr = "127.0.0.1:9000";
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, _)) = listener.accept().await {
        let router: Router =
            Router::new().register(Logger(heartbeat.into_handler()), Action::Heartbeat);
        accept_connection(stream, router).await;
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream, router: Router) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let (mut write, mut read) = ws_stream.split();

    loop {
        tokio::select! {
            msg = read.next() => {
            match msg {
                Some(msg) => {
                    let msg = msg.unwrap();
                    if msg.is_text() ||msg.is_binary() {
                        let v: Call = serde_json::from_str(&msg.clone().into_text().unwrap()).unwrap();
                        let response = router.route(&Request{call: v, charger_id: ChargerId("123".to_string())}).unwrap();
                        let resp = serde_json::to_string(&response).unwrap();
                        write.send(tokio_tungstenite::tungstenite::Message::Text(resp)).await.unwrap();
                    } else if msg.is_close() {
                        break;
                    }
                }
                None => break,
            }
        }
        }
    }
    //future::ok(());
}
