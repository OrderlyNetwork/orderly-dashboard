use crate::api;
use crate::config::CommonConfigs;
use crate::formats_external::{RecoveryBlockRequest, RecoverySolEventRequest};
use anyhow::{Context, Result};
use hyper::{
    body::Buf,
    header,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use ring::signature::UnparsedPublicKey;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::{convert::Infallible, io::Read, net::SocketAddr};
use tracing::error;
use url::form_urlencoded;

const API_SERVER: &str = "api_server_handler";
static MISSING: &[u8] = b"Missing field";

enum Either<T1, T2> {
    Left(T1),
    Right(T2),
}
pub(crate) struct Service {
    peer_public_key: UnparsedPublicKey<Vec<u8>>,
}

impl Service {
    pub fn new(cefi_public_key: String) -> Self {
        let decoded = base64::decode(&cefi_public_key).expect("Unable to decode cefi_public_key");
        let peer_public_key = UnparsedPublicKey::new(&ring::signature::ED25519, decoded);

        Self { peer_public_key }
    }

    pub async fn handle_request(&self, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let uri_path = req.uri().path().to_string();
        let timer = std::time::Instant::now();
        let resp = match self.inner_handle_request(req).await {
            Ok(response) => {
                tracing::info!(target: API_SERVER, "query url {} success", uri_path);
                Ok(response)
            }
            Err(err) => {
                error!(
                    target: API_SERVER,
                    "query url {} fail with err:{}", uri_path, err
                );
                let body = Body::from(format!("Internal server error with: {}", err));
                let mut internal_server_error = Response::new(body);
                *internal_server_error.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                Ok(internal_server_error)
            }
        };
        let elapse = timer.elapsed().as_secs();
        if elapse > 3 {
            tracing::warn!(target: API_SERVER, "query url {} spent {} s which is slow", uri_path, elapse);
        }
        resp
    }

    async fn inner_handle_request(&self, req: Request<Body>) -> anyhow::Result<Response<Body>> {
        let response = match req.method() {
            &Method::GET => self.handle_get(req).await?,
            &Method::POST => self.handle_post(req).await?,
            _ => {
                let mut bad_request = Response::default();
                *bad_request.status_mut() = StatusCode::BAD_REQUEST;
                return Ok(bad_request);
            }
        };

        Ok(response)
    }

    async fn handle_get(&self, req: Request<Body>) -> anyhow::Result<Response<Body>> {
        tracing::info!(
            target: API_SERVER,
            "Got get from cefi query: {}, url: {}",
            req.uri().query().unwrap_or_default(),
            req.uri().path()
        );

        if let Some(sign) = req.headers().get(crate::settings::SIGNATURE_HEADER) {
            let timestamp_header = req
                .headers()
                .get(crate::settings::TIMESTAMP_HEADER)
                .context("mandatory x-timestamp header not found")?;
            let mut message_string =
                format!("{}GET{}", timestamp_header.to_str()?, req.uri().path());
            if let Some(query) = req.uri().query() {
                message_string.push('?');
                message_string.push_str(query);
            }

            let decoded_sig = base64_url::decode(sign.to_str()?).unwrap();
            self.peer_public_key
                .verify(message_string.as_bytes(), decoded_sig.as_ref())
                .map_err(|_| {
                    anyhow::anyhow!("Verifying is failed, message string: {}", message_string)
                })?;
        }

        let json = match req.uri().path() {
            "/pull_perp_trading_events" => match get_query_params(&req) {
                Either::Left(params) => {
                    serde_json::to_string(&api::pull_perp_trading_events(&params).await?)
                }
                Either::Right(response) => return Ok(response),
            },
            "/pull_sol_events" => match get_query_params(&req) {
                Either::Left(params) => {
                    serde_json::to_string(&api::pull_sol_events(&params).await?)
                }
                Either::Right(response) => return Ok(response),
            },
            "/pull_account_trading_events" => match get_query_params(&req) {
                Either::Left(params) => {
                    serde_json::to_string(&api::pull_perp_trading_events_by_account(&params).await?)
                }
                Either::Right(response) => return Ok(response),
            },
            "/pull_account_trading_events_v2" => match get_query_params(&req) {
                Either::Left(params) => serde_json::to_string(
                    &api::pull_perp_trading_events_by_account_v2(&params).await?,
                ),
                Either::Right(response) => return Ok(response),
            },
            "/pull_account_sol_events" => match get_query_params(&req) {
                Either::Left(params) => {
                    serde_json::to_string(&api::pull_sol_events_by_account(&params).await?)
                }
                Either::Right(response) => return Ok(response),
            },
            "/pull_transaction_gas_cost" => match get_query_params(&req) {
                Either::Left(params) => serde_json::to_string(
                    &api::calculate_gas::pull_gas_consumptions::pull_gas_consumptions(&params)
                        .await?,
                ),
                Either::Right(response) => return Ok(response),
            },
            "/get_symbols_data" => serde_json::to_string(&api::get_symbols_data().await?),
            "/get_network_info" => serde_json::to_string(&api::get_network_info().await?),
            "/pull_block_timestamp" => match get_query_params(&req) {
                Either::Left(params) => {
                    serde_json::to_string(&api::network_info::pull_block_time(&params).await?)
                }
                Either::Right(response) => return Ok(response),
            },
            "/status" => serde_json::to_string(&api::get_status().await?),
            _ => {
                let mut not_found = Response::default();
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                return Ok(not_found);
            }
        }?;
        tracing::info!(
            target: API_SERVER,
            "response for cefi get query: {}, url: {}, response: {}",
            req.uri().query().unwrap_or_default(),
            req.uri().path(),
            json,
        );

        let response = Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))?;
        Ok(response)
    }

    async fn handle_post(&self, req: Request<Body>) -> anyhow::Result<Response<Body>> {
        let headers = req.headers().clone();
        let uri = req.uri().clone();
        let body_as_string = get_post_data_as_string(req).await?;

        tracing::info!(
            target: API_SERVER,
            "Got post from cefi data: {}, url: {}",
            body_as_string,
            uri.path()
        );

        if let Some(sign) = headers.get(crate::settings::SIGNATURE_HEADER) {
            let timestamp_header = headers
                .get(crate::settings::TIMESTAMP_HEADER)
                .context("mandatory x-timestamp header not found")?;
            let message_string = format!(
                "{}POST{}{}",
                timestamp_header.to_str()?,
                uri.path(),
                body_as_string
            );

            let decoded_sig = base64_url::decode(sign.to_str()?).unwrap();
            self.peer_public_key
                .verify(message_string.as_bytes(), decoded_sig.as_ref())
                .map_err(|_| {
                    anyhow::anyhow!("Verifying is failed, message string: {}", message_string)
                })?;
        }

        let json = match uri.path() {
            "/recovery/block" => {
                let recovery_block_request: RecoveryBlockRequest =
                    serde_json::from_str(&body_as_string)?;

                let recovery_block_response =
                    api::recovery::recovery_block(recovery_block_request).await?;
                serde_json::to_string(&recovery_block_response)
            }
            "/recovery/deposit_sol" => {
                let recovery_block_request: RecoveryBlockRequest =
                    serde_json::from_str(&body_as_string)?;

                let recovery_block_response =
                    api::recovery::recover_sol_deposit_events(recovery_block_request).await?;
                serde_json::to_string(&recovery_block_response)
            }
            "/recovery/deposit_withdraw_sol_approve_rebalance" => {
                let recovery_block_request: RecoveryBlockRequest =
                    serde_json::from_str(&body_as_string)?;

                let recovery_block_response =
                    api::recovery::recover_sol_withdraw_approve_rebalance_events(
                        recovery_block_request,
                    )
                    .await?;
                serde_json::to_string(&recovery_block_response)
            }
            "/recover/sol_events" => {
                let recovery_sol_events_request: RecoverySolEventRequest =
                    serde_json::from_str(&body_as_string)?;

                let recovery_sol_events_response =
                    api::recovery::recovery_sol_events(recovery_sol_events_request).await?;
                serde_json::to_string(&recovery_sol_events_response)
            }
            _ => {
                let mut not_found = Response::default();
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                return Ok(not_found);
            }
        }?;

        let response = Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))?;
        Ok(response)
    }
}

fn get_query_params(req: &Request<Body>) -> Either<HashMap<String, String>, Response<Body>> {
    let query = if let Some(q) = req.uri().query() {
        q
    } else {
        return Either::Right(
            Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .body(MISSING.into())
                .unwrap(),
        );
    };
    let params = form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>();

    Either::Left(params)
}

async fn get_post_data_as_string(req: Request<Body>) -> Result<String> {
    let whole_body = hyper::body::aggregate(req).await?;

    let mut buf = String::new();

    let _ = whole_body.reader().read_to_string(&mut buf)?;

    Ok(buf)
}

pub(crate) async fn webserver(common_config: CommonConfigs) {
    let cefi_server = &common_config.indexer_server;
    let addr = SocketAddr::from_str(&cefi_server.indexer_address)
        .expect("unable to parse api server listen address");

    let service = Arc::new(Service::new(cefi_server.public_key.clone()));
    let make_svc = make_service_fn(move |_conn| {
        let service = service.clone();

        async {
            Ok::<_, anyhow::Error>(service_fn(move |req| {
                let service = service.clone();
                async move { service.handle_request(req).await }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        error!(target: API_SERVER, "{}", e);
    }
}
