use std::collections::BTreeMap;

use futures_util::{
    future::{ready, BoxFuture, FutureExt},
    sink::SinkExt,
    stream::{BoxStream, StreamExt},
};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::{json, Map, Value};
use thiserror::Error;
use tokio::{
    net::TcpStream,
    sync::{mpsc, oneshot},
    task::JoinHandle,
    time::{sleep, Duration},
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        protocol::frame::{coding::CloseCode, CloseFrame},
        Message,
    },
    MaybeTlsStream, WebSocketStream,
};
use url::Url;

use crate::service_base::sdk::solana::{
    pubsub_client::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
    response::{Response as RpcResponse, RpcLogsResponse},
};

pub type PubsubClientResult<T = ()> = Result<T, PubsubClientError>;

#[derive(Debug, Error)]
pub enum PubsubClientError {
    #[error("url parse error")]
    UrlParseError(#[from] url::ParseError),

    #[error("unable to connect to server")]
    ConnectionError(tokio_tungstenite::tungstenite::Error),

    #[error("websocket error")]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("connection closed (({0})")]
    ConnectionClosed(String),

    #[error("json parse error")]
    JsonParseError(#[from] serde_json::error::Error),

    #[error("subscribe failed: {reason}")]
    SubscribeFailed { reason: String, message: String },

    #[error("unexpected message format: {0}")]
    UnexpectedMessageError(String),

    #[error("request failed: {reason}")]
    RequestFailed { reason: String, message: String },

    #[error("request error: {0}")]
    RequestError(String),

    #[error("could not find subscription id: {0}")]
    UnexpectedSubscriptionResponse(String),

    #[error("could not find node version: {0}")]
    UnexpectedGetVersionResponse(String),
}

type UnsubscribeFn = Box<dyn FnOnce() -> BoxFuture<'static, ()> + Send>;
type SubscribeResponseMsg =
    Result<(mpsc::UnboundedReceiver<Value>, UnsubscribeFn), PubsubClientError>;
type SubscribeRequestMsg = (String, Value, oneshot::Sender<SubscribeResponseMsg>);
type SubscribeResult<'a, T> = PubsubClientResult<(BoxStream<'a, T>, UnsubscribeFn)>;
type RequestMsg = (
    String,
    Value,
    oneshot::Sender<Result<Value, PubsubClientError>>,
);

const SOL_SDK: &str = "sol_sdk";

#[derive(Deserialize, Debug)]
pub struct RpcErrorObject {
    pub code: i64,
    pub message: String,
}

/// A client for subscribing to messages from the RPC server.
///
/// See the [module documentation][self].
#[derive(Debug)]
pub struct PubsubClient {
    subscribe_sender: mpsc::UnboundedSender<SubscribeRequestMsg>,
    _request_sender: mpsc::UnboundedSender<RequestMsg>,
    shutdown_sender: oneshot::Sender<()>,
    ws: JoinHandle<PubsubClientResult>,
}

impl PubsubClient {
    pub async fn new(url: &str) -> PubsubClientResult<Self> {
        let url = Url::parse(url)?;
        let (ws, _response) = connect_async(url.to_string()).await?;

        let (subscribe_sender, subscribe_receiver) = mpsc::unbounded_channel();
        let (_request_sender, request_receiver) = mpsc::unbounded_channel();
        let (shutdown_sender, shutdown_receiver) = oneshot::channel();

        #[allow(clippy::used_underscore_binding)]
        Ok(Self {
            subscribe_sender,
            _request_sender,
            shutdown_sender,
            ws: tokio::spawn(PubsubClient::run_ws(
                ws,
                subscribe_receiver,
                request_receiver,
                shutdown_receiver,
            )),
        })
    }

    pub async fn shutdown(self) -> PubsubClientResult {
        let _ = self.shutdown_sender.send(());
        self.ws.await.unwrap() // WS future should not be cancelled or panicked
    }

    #[deprecated(since = "2.0.2", note = "PubsubClient::node_version is no longer used")]
    pub async fn set_node_version(&self, _version: semver::Version) -> Result<(), ()> {
        Ok(())
    }

    async fn subscribe<'a, T>(&self, operation: &str, params: Value) -> SubscribeResult<'a, T>
    where
        T: DeserializeOwned + Send + 'a,
    {
        let (response_sender, response_receiver) = oneshot::channel();
        self.subscribe_sender
            .send((operation.to_string(), params, response_sender))
            .map_err(|err| PubsubClientError::ConnectionClosed(err.to_string()))?;

        let (notifications, unsubscribe) = response_receiver
            .await
            .map_err(|err| PubsubClientError::ConnectionClosed(err.to_string()))??;
        Ok((
            UnboundedReceiverStream::new(notifications)
                .filter_map(|value| ready(serde_json::from_value::<T>(value).ok()))
                .boxed(),
            unsubscribe,
        ))
    }

    /// Subscribe to transaction log events.
    ///
    /// Receives messages of type [`RpcLogsResponse`] when a transaction is committed.
    ///
    /// # RPC Reference
    ///
    /// This method corresponds directly to the [`logsSubscribe`] RPC method.
    ///
    /// [`logsSubscribe`]: https://solana.com/docs/rpc/websocket#logssubscribe
    pub async fn logs_subscribe(
        &self,
        filter: RpcTransactionLogsFilter,
        config: RpcTransactionLogsConfig,
    ) -> SubscribeResult<'_, RpcResponse<RpcLogsResponse>> {
        self.subscribe("logs", json!([filter, config])).await
    }

    async fn run_ws(
        mut ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
        mut subscribe_receiver: mpsc::UnboundedReceiver<SubscribeRequestMsg>,
        mut request_receiver: mpsc::UnboundedReceiver<RequestMsg>,
        mut shutdown_receiver: oneshot::Receiver<()>,
    ) -> PubsubClientResult {
        let mut request_id: u64 = 0;

        let mut requests_subscribe = BTreeMap::new();
        let mut requests_unsubscribe = BTreeMap::<u64, oneshot::Sender<()>>::new();
        let mut other_requests = BTreeMap::new();
        let mut subscriptions = BTreeMap::new();
        let (unsubscribe_sender, mut unsubscribe_receiver) = mpsc::unbounded_channel();

        loop {
            tokio::select! {
                // Send close on shutdown signal
                _ = (&mut shutdown_receiver) => {
                    let frame = CloseFrame { code: CloseCode::Normal, reason: "".into() };
                    ws.send(Message::Close(Some(frame))).await?;
                    ws.flush().await?;
                    break;
                },
                // Send `Message::Ping` each 10s if no any other communication
                () = sleep(Duration::from_secs(10)) => {
                    ws.send(Message::Ping(Vec::new())).await?;
                },
                // Read message for subscribe
                Some((operation, params, response_sender)) = subscribe_receiver.recv() => {
                    request_id += 1;
                    let method = format!("{operation}Subscribe");
                    let text = json!({"jsonrpc":"2.0","id":request_id,"method":method,"params":params}).to_string();
                    ws.send(Message::Text(text)).await?;
                    requests_subscribe.insert(request_id, (operation, response_sender));
                },
                // Read message for unsubscribe
                Some((operation, sid, response_sender)) = unsubscribe_receiver.recv() => {
                    subscriptions.remove(&sid);
                    request_id += 1;
                    let method = format!("{operation}Unsubscribe");
                    let text = json!({"jsonrpc":"2.0","id":request_id,"method":method,"params":[sid]}).to_string();
                    ws.send(Message::Text(text)).await?;
                    requests_unsubscribe.insert(request_id, response_sender);
                },
                // Read message for other requests
                Some((method, params, response_sender)) = request_receiver.recv() => {
                    request_id += 1;
                    let text = json!({"jsonrpc":"2.0","id":request_id,"method":method,"params":params}).to_string();
                    ws.send(Message::Text(text)).await?;
                    other_requests.insert(request_id, response_sender);
                }
                // Read incoming WebSocket message
                next_msg = ws.next() => {
                    let msg = match next_msg {
                        Some(msg) => msg?,
                        None => break,
                    };
                    tracing::trace!(target: SOL_SDK, "ws.next(): {:?}", &msg);

                    // Get text from the message
                    let text = match msg {
                        Message::Text(text) => text,
                        Message::Binary(_data) => continue, // Ignore
                        Message::Ping(data) => {
                            ws.send(Message::Pong(data)).await?;
                            continue
                        },
                        Message::Pong(_data) => continue,
                        Message::Close(_frame) => break,
                        Message::Frame(_frame) => continue,
                    };


                    let mut json: Map<String, Value> = serde_json::from_str(&text)?;

                    // Subscribe/Unsubscribe response, example:
                    // `{"jsonrpc":"2.0","result":5308752,"id":1}`
                    if let Some(id) = json.get("id") {
                        let id = id.as_u64().ok_or_else(|| {
                            PubsubClientError::SubscribeFailed { reason: "invalid `id` field".into(), message: text.clone() }
                        })?;

                        let err = json.get("error").map(|error_object| {
                            match serde_json::from_value::<RpcErrorObject>(error_object.clone()) {
                                Ok(rpc_error_object) => {
                                    format!("{} ({})",  rpc_error_object.message, rpc_error_object.code)
                                }
                                Err(err) => format!(
                                    "Failed to deserialize RPC error response: {} [{}]",
                                    serde_json::to_string(error_object).unwrap(),
                                    err
                                )
                            }
                        });

                        if let Some(response_sender) = other_requests.remove(&id) {
                            match err {
                                Some(reason) => {
                                    let _ = response_sender.send(Err(PubsubClientError::RequestFailed { reason, message: text.clone()}));
                                },
                                None => {
                                    let json_result = json.get("result").ok_or_else(|| {
                                        PubsubClientError::RequestFailed { reason: "missing `result` field".into(), message: text.clone() }
                                    })?;
                                    if response_sender.send(Ok(json_result.clone())).is_err() {
                                        break;
                                    }
                                }
                            }
                        } else if let Some(response_sender) = requests_unsubscribe.remove(&id) {
                            let _ = response_sender.send(()); // do not care if receiver is closed
                        } else if let Some((operation, response_sender)) = requests_subscribe.remove(&id) {
                            match err {
                                Some(reason) => {
                                    let _ = response_sender.send(Err(PubsubClientError::SubscribeFailed { reason, message: text.clone()}));
                                },
                                None => {
                                    // Subscribe Id
                                    let sid = json.get("result").and_then(Value::as_u64).ok_or_else(|| {
                                        PubsubClientError::SubscribeFailed { reason: "invalid `result` field".into(), message: text.clone() }
                                    })?;

                                    // Create notifications channel and unsubscribe function
                                    let (notifications_sender, notifications_receiver) = mpsc::unbounded_channel();
                                    let unsubscribe_sender = unsubscribe_sender.clone();
                                    let unsubscribe = Box::new(move || async move {
                                        let (response_sender, response_receiver) = oneshot::channel();
                                        // do nothing if ws already closed
                                        if unsubscribe_sender.send((operation, sid, response_sender)).is_ok() {
                                            let _ = response_receiver.await; // channel can be closed only if ws is closed
                                        }
                                    }.boxed());

                                    if response_sender.send(Ok((notifications_receiver, unsubscribe))).is_err() {
                                        break;
                                    }
                                    subscriptions.insert(sid, notifications_sender);
                                }
                            }
                        } else {
                            tracing::error!(target: SOL_SDK, "Unknown request id: {}", id);
                            break;
                        }
                        continue;
                    }

                    // Notification, example:
                    // `{"jsonrpc":"2.0","method":"logsNotification","params":{"result":{...},"subscription":3114862}}`
                    if let Some(Value::Object(params)) = json.get_mut("params") {
                        if let Some(sid) = params.get("subscription").and_then(Value::as_u64) {
                            let mut unsubscribe_required = false;

                            if let Some(notifications_sender) = subscriptions.get(&sid) {
                                if let Some(result) = params.remove("result") {
                                    if notifications_sender.send(result).is_err() {
                                        unsubscribe_required = true;
                                    }
                                }
                            } else {
                                unsubscribe_required = true;
                            }

                            if unsubscribe_required {
                                if let Some(Value::String(method)) = json.remove("method") {
                                    if let Some(operation) = method.strip_suffix("Notification") {
                                        let (response_sender, _response_receiver) = oneshot::channel();
                                        let _ = unsubscribe_sender.send((operation.to_string(), sid, response_sender));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
