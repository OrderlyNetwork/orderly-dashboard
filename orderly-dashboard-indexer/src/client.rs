use crate::config::CefiServerConfig;
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use hyper::header::{HeaderValue, CONTENT_TYPE};
use reqwest::Client as InnerClient;
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

pub(crate) type HttpClient = Arc<dyn Client + Send + Sync + 'static>;

const CLIENT_CONTEXT: &str = "client_context";

#[async_trait]
pub(crate) trait Client {}

pub(crate) struct RealClient {
    inner_client: InnerClient,
    server_addr: String,
    key_pair: Ed25519KeyPair,
}

impl RealClient {
    pub fn new(config: CefiServerConfig) -> Self {
        let decoded = base64::decode(&config.private_key).expect("Unable to decode private key");

        let key_pair =
            Ed25519KeyPair::from_pkcs8(&decoded).expect("Unable to create Ed25519 key pair");

        let peer_public_key_bytes = key_pair.public_key().as_ref();
        let encoded = base64::encode(peer_public_key_bytes);
        tracing::info!(
            target: crate::ORDERLY_DASHBOARD_INDEXER,
            "real client public key:{}",
            encoded
        );
        Self {
            inner_client: InnerClient::new(),
            server_addr: config.server_address,
            key_pair,
        }
    }
}

#[async_trait]
impl Client for RealClient {}

impl RealClient {
    async fn inner_post<Rq: Serialize, Rs: DeserializeOwned>(
        &self,
        request: &Rq,
        uri: url::Url,
    ) -> Result<Rs> {
        let body = serde_json::to_string(request)?;
        tracing::info!(
            target: CLIENT_CONTEXT,
            "start post to cefi data: {}, url: {}",
            body,
            uri.path(),
        );

        let timestamp = Utc::now().timestamp_millis();
        let message_string = format!("{}POST{}{}", timestamp, uri.path(), body);

        let sig = self.key_pair.sign(message_string.as_bytes());
        let encoded_sig = base64_url::encode(&sig);

        let raw_response = self
            .inner_client
            .post(uri.clone())
            .body(body.clone())
            .header(crate::settings::TIMESTAMP_HEADER, timestamp)
            .header(crate::settings::SIGNATURE_HEADER, encoded_sig)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .send()
            .await?
            .text()
            .await?;

        tracing::info!(
            target: CLIENT_CONTEXT,
            "Post to cefi data: {}, url: {}, response: {}",
            body,
            uri.path(),
            raw_response
        );

        Ok(serde_json::from_str(&raw_response)?)
    }
}

pub(crate) fn new_http_client(config: CefiServerConfig) -> HttpClient {
    Arc::new(RealClient::new(config))
}
