mod messages;
mod types;

use crate::messages::{ClientMessage, ServerMessage};
use crate::types::AppIdentifiers;
use futures_util::SinkExt;
use futures_util::StreamExt;
use log;
use serde_json;
use std::error::Error;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::connect_async;
use url::Url;

/// A struct representing a WebSocket connection to a Microsoft Teams server.
///
/// # Fields
/// - `identifier`: An `AppIdentifiers` struct containing information about the app.
/// - `socket`: An optional WebSocket stream.
/// - `token`: An optional authentication token.
/// - `request_id`: A counter for request IDs.
/// - `url`: The URL of the WebSocket server.
///
/// # Methods
/// - `new`: Creates a new `TeamsWebsocket` instance.
/// - `connect`: Connects to the WebSocket server.
/// - `send`: Sends a `ClientMessage` to the server.
/// - `receive`: Receives a `ServerMessage` from the server.
/// - `close`: Closes the WebSocket connection.
///
/// # Example
/// ```rust
/// let identifier = AppIdentifiers {
///     protocol_version: "1.0",
///     manufacturer: "TestManufacturer",
///     device: "TestDevice",
///     app: "TestApp",
///     app_version: "1.0",
/// };
/// let mut websocket = TeamsWebsocket::new(identifier, None, None).await;
/// websocket.connect().await.unwrap();
/// let client_message = ClientMessage::new(messages::MeetingAction::BlurBackground, None);
/// websocket.send(client_message).await.unwrap();
/// let server_message = websocket.receive().await.unwrap();
/// websocket.close().await.unwrap();
/// ```
pub struct TeamsWebsocket {
    identifier: AppIdentifiers,
    socket: Option<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    >,
    token: Option<String>,
    request_id: i32,
    url: String,
}

const SOCKET_NOT_CONNECTED: &str = "socket not connected";

impl TeamsWebsocket {
    pub async fn new(
        identifier: AppIdentifiers,
        token: Option<String>,
        url: Option<String>,
    ) -> Self {
        Self {
            identifier,
            socket: None,
            token,
            request_id: 0,
            url: url.unwrap_or_else(|| "ws://127.0.0.1:8124".to_string()),
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let url = Url::parse_with_params(
            &self.url,
            &[
                ("protocol-version", self.identifier.protocol_version),
                ("manufacturer", self.identifier.manufacturer),
                ("device", self.identifier.device),
                ("app", self.identifier.app),
                ("app-version", self.identifier.app_version),
                ("token", self.token.as_deref().unwrap_or("")),
            ],
        );
        if let Err(e) = url {
            log::warn!("Error parsing url: {}", e);
            return Err(Box::new(e));
        }
        let url = url.unwrap();

        let (socket, response) = match connect_async(url.as_str()).await {
            Ok((socket, response)) => (socket, response),
            Err(e) => {
                log::warn!("Error: {}", e);
                return Err(Box::new(e));
            }
        };

        if log::log_enabled!(log::Level::Debug) {
            log::debug!("Connected to the server");
            log::debug!("Response HTTP code: {}", response.status());
            log::debug!("Response contains the following headers:");
            for (header, _value) in response.headers() {
                log::trace!("* {header}");
            }
        }
        self.socket = Some(socket);
        Ok(())
    }

    pub async fn send(&mut self, message: ClientMessage) -> Result<(), Box<dyn Error>> {
        if let Some(socket) = &mut self.socket {
            let mut message = message;
            message.request_id = Some(self.request_id);
            self.request_id += 1;
            let serialized_message = serde_json::to_string(&message);
            log::debug!("Sending message: {:?}", serialized_message);
            match serialized_message {
                Ok(msg) => {
                    if let Err(e) = socket
                    .send(tungstenite::Message::Text(msg))
                    .await
                    {
                        log::warn!("Error sending message: {}", e);
                        return Err(Box::new(e));
                    }
                }
                Err(e) => {
                    log::warn!("Error serializing message: {}", e);
                    return Err(Box::new(e));
                }
            } 
            return Ok(());
        }
        log::warn!("{}", SOCKET_NOT_CONNECTED);
        Err(Box::from(SOCKET_NOT_CONNECTED))
        
    }

    pub async fn receive(&mut self) -> Result<ServerMessage, Box<dyn Error>> {
        if let Some(socket) = &mut self.socket {
            match timeout(Duration::from_millis(10), socket.next()).await {
                Err(e) => {
                    return Err(Box::new(e));
                }
                Ok(None) => {
                    log::info!("Socket closed");
                    return Err(Box::from("socket closed"));
                }
                Ok(Some(msg)) => match msg {
                    Ok(msg) => {
                        let server_message =
                            serde_json::from_str::<ServerMessage>(&msg.to_text().unwrap());
                        match server_message {
                            Ok(json) => {
                                return Ok(json);
                            }
                            Err(e) => {
                                log::warn!("Error parsing json : {}", e);
                                return Err(Box::new(e));
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!("Error reading from socket {}", e);
                        return Err(Box::new(e));
                    }
                },
            }
        } else {
            log::warn!("{}", SOCKET_NOT_CONNECTED);
            return Err(Box::from(SOCKET_NOT_CONNECTED));
        }
    }

    pub async fn close(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(socket) = &mut self.socket {
            if let Err(e) = socket.close(None).await {
                log::warn!("Error closing socket: {}", e);
                return Err(Box::new(e));
            }
            log::info!("Connection closed");
            Ok(())
        } else {
            log::warn!("{}", SOCKET_NOT_CONNECTED);
            return Err(Box::from(SOCKET_NOT_CONNECTED));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::net::SocketAddr;
    use tokio::net::TcpListener;
    use tokio::runtime::Runtime;
    use tokio_tungstenite::accept_async;
    use tokio_tungstenite::tungstenite::protocol::Message;

    #[test]
    fn test_teams_websocket_new() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let identifier = AppIdentifiers {
                protocol_version: "1.0",
                manufacturer: "TestManufacturer",
                device: "TestDevice",
                app: "TestApp",
                app_version: "1.0",
            };
            let websocket = TeamsWebsocket::new(identifier.clone(), None, None).await;
            assert_eq!(websocket.identifier, identifier);
            assert!(websocket.socket.is_none());
            assert!(websocket.token.is_none());
            assert_eq!(websocket.request_id, 0);
        });
    }
    async fn start_test_server() -> SocketAddr {
        let mut rng = rand::thread_rng();
        let port: u16 = rng.gen_range(1024..65535);
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
            .await
            .unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                let ws_stream = accept_async(stream).await.unwrap();
                let (mut write, mut read) = ws_stream.split();
                tokio::spawn(async move {
                    while let Some(Ok(msg)) = read.next().await {
                        if let Message::Text(text) = msg {
                            let client_message: ClientMessage =
                                serde_json::from_str(&text).unwrap();
                            let server_message = ServerMessage {
                                request_id: client_message.request_id,
                                response: Some(format!("Echo: {}", text)),
                                error_msg: None,
                                token_refresh: None,
                                meeting_update: None,
                            };
                            let response = serde_json::to_string(&server_message).unwrap();
                            write.send(Message::Text(response)).await.unwrap();
                        }
                    }
                });
            }
        });
        addr
    }

    #[test]
    fn test_teams_websocket_connect() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let identifier = AppIdentifiers {
                protocol_version: "1.0",
                manufacturer: "TestManufacturer",
                device: "TestDevice",
                app: "TestApp",
                app_version: "1.0",
            };
            let addr = start_test_server().await;
            let url = format!("ws://{}", addr);
            let mut websocket = TeamsWebsocket::new(identifier.clone(), None, Some(url)).await;
            let result = websocket.connect().await;
            assert!(result.is_ok());
            assert!(websocket.socket.is_some());
        });
    }

    #[test]
    fn test_teams_websocket_send_receive() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let identifier = AppIdentifiers {
                protocol_version: "1.0",
                manufacturer: "TestManufacturer",
                device: "TestDevice",
                app: "TestApp",
                app_version: "1.0",
            };
            let addr = start_test_server().await;
            let url = format!("ws://{}", addr);
            let mut websocket = TeamsWebsocket::new(identifier.clone(), None, Some(url)).await;
            websocket.connect().await.unwrap();

            let client_message = ClientMessage::new(messages::MeetingAction::BlurBackground, None);
            websocket.send(client_message).await.unwrap();

            let server_message = websocket.receive().await.unwrap();
            assert_eq!(
                server_message.response,
                Some(
                    "Echo: {\"action\":\"blur-background\",\"parameters\":null,\"requestId\":0}"
                        .to_string()
                )
            );
        });
    }
}
