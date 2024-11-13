pub mod nonblocking;

use std::{
    marker::PhantomData,
    net::TcpStream,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
    thread::{sleep, JoinHandle},
    time::Duration,
};

use crossbeam_channel::{unbounded, Receiver, Sender};
use nonblocking::PubsubClientError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{
    json,
    value::Value::{Number, Object},
    Map, Value,
};
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use url::Url;

use crate::service_base::sdk::solana::{
    commitment_config::CommitmentConfig,
    response::{Response as RpcResponse, RpcLogsResponse},
};

const SOL_SDK: &str = "sol_sdk";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RpcTransactionLogsFilter {
    All,
    AllWithVotes,
    Mentions(Vec<String>), // base58-encoded list of addresses
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionLogsConfig {
    #[serde(flatten)]
    pub commitment: Option<CommitmentConfig>,
}

/// A subscription.
///
/// The subscription is unsubscribed on drop, and note that unsubscription (and
/// thus drop) time is unbounded. See
/// [`PubsubClientSubscription::send_unsubscribe`].
pub struct PubsubClientSubscription<T>
where
    T: DeserializeOwned,
{
    message_type: PhantomData<T>,
    operation: &'static str,
    socket: Arc<RwLock<WebSocket<MaybeTlsStream<TcpStream>>>>,
    subscription_id: u64,
    t_cleanup: Option<JoinHandle<()>>,
    exit: Arc<AtomicBool>,
}

impl<T> Drop for PubsubClientSubscription<T>
where
    T: DeserializeOwned,
{
    fn drop(&mut self) {
        self.send_unsubscribe().unwrap_or_else(
            |_| tracing::warn!(target: SOL_SDK, "unable to unsubscribe from websocket"),
        );
        self.socket
            .write()
            .unwrap()
            .close(None)
            .unwrap_or_else(|_| tracing::warn!(target: SOL_SDK, "unable to close websocket"));
    }
}

impl<T> PubsubClientSubscription<T>
where
    T: DeserializeOwned,
{
    fn send_subscribe(
        writable_socket: &Arc<RwLock<WebSocket<MaybeTlsStream<TcpStream>>>>,
        body: String,
    ) -> Result<u64, PubsubClientError> {
        writable_socket.write().unwrap().send(Message::Text(body))?;
        let message = writable_socket.write().unwrap().read()?;
        Self::extract_subscription_id(message)
    }

    fn extract_subscription_id(message: Message) -> Result<u64, PubsubClientError> {
        let message_text = &message.into_text()?;

        if let Ok(json_msg) = serde_json::from_str::<Map<String, Value>>(message_text) {
            if let Some(Number(x)) = json_msg.get("result") {
                if let Some(x) = x.as_u64() {
                    return Ok(x);
                }
            }
        }

        Err(PubsubClientError::UnexpectedSubscriptionResponse(format!(
            "msg={message_text}"
        )))
    }

    /// Send an unsubscribe message to the server.
    ///
    /// Note that this will block as long as the internal subscription receiver
    /// is waiting on messages from the server, and this can take an unbounded
    /// amount of time if the server does not send any messages.
    ///
    /// If a pubsub client needs to shutdown reliably it should use
    /// the async client in [`crate::nonblocking::pubsub_client`].
    pub fn send_unsubscribe(&self) -> Result<(), PubsubClientError> {
        let method = format!("{}Unsubscribe", self.operation);
        self.socket
            .write()
            .unwrap()
            .send(Message::Text(
                json!({
                "jsonrpc":"2.0","id":1,"method":method,"params":[self.subscription_id]
                })
                .to_string(),
            ))
            .map_err(|err| err.into())
    }

    fn read_message(
        writable_socket: &Arc<RwLock<WebSocket<MaybeTlsStream<TcpStream>>>>,
    ) -> Result<Option<T>, PubsubClientError> {
        let message = writable_socket.write().unwrap().read()?;
        if message.is_ping() {
            return Ok(None);
        }
        let message_text = &message.into_text()?;
        if let Ok(json_msg) = serde_json::from_str::<Map<String, Value>>(message_text) {
            if let Some(Object(params)) = json_msg.get("params") {
                if let Some(result) = params.get("result") {
                    if let Ok(x) = serde_json::from_value::<T>(result.clone()) {
                        return Ok(Some(x));
                    }
                }
            }
        }

        Err(PubsubClientError::UnexpectedMessageError(format!(
            "msg={message_text}"
        )))
    }

    /// Shutdown the internel message receiver and wait for its thread to exit.
    ///
    /// Note that this will block as long as the subscription receiver is
    /// waiting on messages from the server, and this can take an unbounded
    /// amount of time if the server does not send any messages.
    ///
    /// If a pubsub client needs to shutdown reliably it should use
    /// the async client in [`crate::nonblocking::pubsub_client`].
    pub fn shutdown(&mut self) -> std::thread::Result<()> {
        if self.t_cleanup.is_some() {
            tracing::info!(target: SOL_SDK, "websocket thread - shutting down");
            self.exit.store(true, Ordering::Relaxed);
            let x = self.t_cleanup.take().unwrap().join();
            tracing::info!(target: SOL_SDK, "websocket thread - shut down.");
            x
        } else {
            tracing::warn!(target: SOL_SDK, "websocket thread - already shut down.");
            Ok(())
        }
    }
}

pub type PubsubLogsClientSubscription = PubsubClientSubscription<RpcResponse<RpcLogsResponse>>;
pub type LogsSubscription = (
    PubsubLogsClientSubscription,
    Receiver<RpcResponse<RpcLogsResponse>>,
);

/// A client for subscribing to messages from the RPC server.
///
/// See the [module documentation][self].
pub struct PubsubClient {}

fn connect_with_retry(
    url: Url,
) -> Result<WebSocket<MaybeTlsStream<TcpStream>>, tungstenite::Error> {
    let mut connection_retries = 5;
    loop {
        let result = connect(url.clone()).map(|(socket, _)| socket);
        if let Err(tungstenite::Error::Http(response)) = &result {
            if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS && connection_retries > 0
            {
                let mut duration = Duration::from_millis(500);
                if let Some(retry_after) = response.headers().get(reqwest::header::RETRY_AFTER) {
                    if let Ok(retry_after) = retry_after.to_str() {
                        if let Ok(retry_after) = retry_after.parse::<u64>() {
                            if retry_after < 120 {
                                duration = Duration::from_secs(retry_after);
                            }
                        }
                    }
                }

                connection_retries -= 1;
                tracing::debug!(target: SOL_SDK,
                    "Too many requests: server responded with {:?}, {} retries left, pausing for {:?}",
                    response, connection_retries, duration
                );

                sleep(duration);
                continue;
            }
        }
        return result;
    }
}

impl PubsubClient {
    /// Subscribe to transaction log events.
    ///
    /// Receives messages of type [`RpcLogsResponse`] when a transaction is committed.
    ///
    /// # RPC Reference
    ///
    /// This method corresponds directly to the [`logsSubscribe`] RPC method.
    ///
    /// [`logsSubscribe`]: https://solana.com/docs/rpc/websocket/logssubscribe
    pub fn logs_subscribe(
        url: &str,
        filter: RpcTransactionLogsFilter,
        config: RpcTransactionLogsConfig,
    ) -> Result<LogsSubscription, PubsubClientError> {
        let url = Url::parse(url)?;
        let socket = connect_with_retry(url)?;
        let (sender, receiver) = unbounded();

        let socket = Arc::new(RwLock::new(socket));
        let socket_clone = socket.clone();
        let exit = Arc::new(AtomicBool::new(false));
        let exit_clone = exit.clone();
        let body = json!({
            "jsonrpc":"2.0",
            "id":1,
            "method":"logsSubscribe",
            "params":[filter, config]
        })
        .to_string();

        let subscription_id = PubsubLogsClientSubscription::send_subscribe(&socket_clone, body)?;

        let t_cleanup = std::thread::spawn(move || {
            Self::cleanup_with_sender(exit_clone, &socket_clone, sender)
        });

        let result = PubsubClientSubscription {
            message_type: PhantomData,
            operation: "logs",
            socket,
            subscription_id,
            t_cleanup: Some(t_cleanup),
            exit,
        };

        Ok((result, receiver))
    }

    fn cleanup_with_sender<T>(
        exit: Arc<AtomicBool>,
        socket: &Arc<RwLock<WebSocket<MaybeTlsStream<TcpStream>>>>,
        sender: Sender<T>,
    ) where
        T: DeserializeOwned + Send + 'static,
    {
        let handler = move |message| match sender.send(message) {
            Ok(_) => (),
            Err(err) => {
                tracing::info!(target: SOL_SDK, "receive error: {:?}", err);
            }
        };
        Self::cleanup_with_handler(exit, socket, handler);
    }

    fn cleanup_with_handler<T, F>(
        exit: Arc<AtomicBool>,
        socket: &Arc<RwLock<WebSocket<MaybeTlsStream<TcpStream>>>>,
        handler: F,
    ) where
        T: DeserializeOwned,
        F: Fn(T) + Send + 'static,
    {
        loop {
            if exit.load(Ordering::Relaxed) {
                break;
            }

            match PubsubClientSubscription::read_message(socket) {
                Ok(Some(message)) => handler(message),
                Ok(None) => {
                    // Nothing useful, means we received a ping message
                }
                Err(err) => {
                    tracing::info!(target: SOL_SDK, "receive error: {:?}", err);
                    break;
                }
            }
        }

        tracing::info!(target: SOL_SDK, "websocket - exited receive loop");
    }
}
