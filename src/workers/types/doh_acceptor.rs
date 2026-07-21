use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::time::timeout;

use crate::config::Config;
use crate::exceptions::SCloudException;
use crate::utils;
use crate::workers::task::{InFlightTask, SCloudWorkerTask};
use crate::workers::{SCloudWorker, WorkerType, reply_registry};
use crate::{log_debug, log_error, log_info};

const MAX_DNS_MESSAGE_BYTES: usize = 65_535;
const REPLY_TIMEOUT_SECS: u64 = 10;
const DNS_MESSAGE_MIME: &str = "application/dns-message";

#[derive(Clone)]
struct DohHandlerCtx {
    worker: Arc<SCloudWorker>,
    tx: Vec<mpsc::Sender<InFlightTask>>,
    paths: Arc<HashSet<String>>,
    allowed_origins: Arc<HashSet<String>>,
}

pub async fn run_dns_doh_acceptor(
    worker: Arc<SCloudWorker>,
    tx: Vec<mpsc::Sender<InFlightTask>>,
) -> Result<(), SCloudException> {
    if tx.is_empty() {
        return Err(SCloudException::SCLOUD_WORKER_TX_NOT_SET);
    }

    let cfg = Config::from_file(Path::new("./config/config.json"))?;
    if !cfg.doh.enabled {
        log_info!("DoH disabled in config, acceptor idle");
        futures_util::future::pending::<()>().await;
        return Ok(());
    }

    let bind_addr: SocketAddr = cfg
        .doh
        .bind
        .parse()
        .map_err(|_| SCloudException::SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_ADDR)?;

    let listener = TcpListener::bind(bind_addr)
        .await
        .map_err(|_| SCloudException::SCLOUD_WORKER_LISTENER_BIND_FAILED)?;

    log_info!("DoH acceptor listening on http://{}", bind_addr);

    let ctx = DohHandlerCtx {
        worker: worker.clone(),
        tx,
        paths: Arc::new(cfg.doh.paths.into_iter().collect()),
        allowed_origins: Arc::new(cfg.doh.allowed_origins.into_iter().collect()),
    };

    loop {
        let (stream, peer) = match listener.accept().await {
            Ok(v) => v,
            Err(e) => {
                log_error!("doh accept failed: {}", e);
                continue;
            }
        };
        let io = TokioIo::new(stream);
        let ctx = ctx.clone();

        tokio::spawn(async move {
            let svc = service_fn(move |req| handle_request(ctx.clone(), peer, req));
            if let Err(e) = Builder::new(TokioExecutor::new())
                .serve_connection(io, svc)
                .await
            {
                log_debug!("doh connection from {} ended: {}", peer, e);
            }
        });
    }
}

async fn handle_request(
    ctx: DohHandlerCtx,
    peer: SocketAddr,
    req: Request<Incoming>,
) -> Result<Response<Full<Bytes>>, std::convert::Infallible> {
    let method = req.method().clone();
    let origin = req
        .headers()
        .get("origin")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    if method == Method::OPTIONS {
        return Ok(cors_preflight(&ctx, origin.as_deref()));
    }

    if !ctx.paths.contains(req.uri().path()) {
        return Ok(simple(StatusCode::NOT_FOUND, "unknown doh path"));
    }

    let wire = match method {
        Method::GET => match extract_get_dns(&req) {
            Ok(b) => b,
            Err((status, msg)) => return Ok(simple(status, msg)),
        },
        Method::POST => match extract_post_dns(req).await {
            Ok(b) => b,
            Err((status, msg)) => return Ok(simple(status, msg)),
        },
        _ => return Ok(simple(StatusCode::METHOD_NOT_ALLOWED, "method not allowed")),
    };

    if wire.is_empty() || wire.len() > MAX_DNS_MESSAGE_BYTES {
        return Ok(simple(
            StatusCode::BAD_REQUEST,
            "empty or oversize dns body",
        ));
    }

    let reply = match dispatch_and_wait(&ctx, peer, wire).await {
        Ok(b) => b,
        Err(status) => return Ok(simple(status, "dispatch failed")),
    };

    let mut resp = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", DNS_MESSAGE_MIME)
        .header("Cache-Control", "no-store")
        .body(Full::new(reply))
        .unwrap();
    apply_cors(&mut resp, &ctx, origin.as_deref());
    Ok(resp)
}

fn extract_get_dns(req: &Request<Incoming>) -> Result<Bytes, (StatusCode, &'static str)> {
    let q = req.uri().query().unwrap_or("");
    let mut dns_param: Option<&str> = None;
    for pair in q.split('&') {
        if let Some(v) = pair.strip_prefix("dns=") {
            dns_param = Some(v);
            break;
        }
    }
    let dns_b64 = dns_param.ok_or((StatusCode::BAD_REQUEST, "missing dns query parameter"))?;
    URL_SAFE_NO_PAD
        .decode(dns_b64)
        .map(Bytes::from)
        .map_err(|_| (StatusCode::BAD_REQUEST, "invalid base64url dns parameter"))
}

async fn extract_post_dns(req: Request<Incoming>) -> Result<Bytes, (StatusCode, &'static str)> {
    let ct_ok = req
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.starts_with(DNS_MESSAGE_MIME))
        .unwrap_or(false);
    if !ct_ok {
        return Err((
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "expected application/dns-message",
        ));
    }
    let body = req
        .into_body()
        .collect()
        .await
        .map_err(|_| (StatusCode::BAD_REQUEST, "failed reading body"))?
        .to_bytes();
    Ok(body)
}

async fn dispatch_and_wait(
    ctx: &DohHandlerCtx,
    peer: SocketAddr,
    wire: Bytes,
) -> Result<Bytes, StatusCode> {
    let permit = ctx
        .worker
        .in_flight_sem
        .clone()
        .try_acquire_owned()
        .map_err(|_| StatusCode::TOO_MANY_REQUESTS)?;

    let task_id = utils::uuid::generate_uuid();
    let task = SCloudWorkerTask {
        task_id,
        for_type: WorkerType::DOH_ACCEPTOR,
        for_who: peer,
        payload: wire,
        attempts: 0,
        max_attempts: 0,
        created_at: SystemTime::now(),
        deadline_timeout: None,
        priority: 0,
        reply_to: Some(reply_registry::REPLY_TAG_DOH.to_string()),
        correlation_id: None,
    };
    let in_flight = InFlightTask {
        task,
        _permit: permit,
    };

    let rx = reply_registry::register(task_id);

    if !forward_task(in_flight, &ctx.tx).await {
        reply_registry::drop_entry(&task_id);
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }

    match timeout(Duration::from_secs(REPLY_TIMEOUT_SECS), rx).await {
        Ok(Ok(bytes)) => Ok(bytes),
        _ => {
            reply_registry::drop_entry(&task_id);
            Err(StatusCode::GATEWAY_TIMEOUT)
        }
    }
}

async fn forward_task(task: InFlightTask, tx: &[mpsc::Sender<InFlightTask>]) -> bool {
    let mut current = Some(task);
    for tx_channel in tx.iter() {
        match tx_channel.try_send(current.take().unwrap()) {
            Ok(_) => return true,
            Err(mpsc::error::TrySendError::Full(returned)) => {
                current = Some(returned);
            }
            Err(mpsc::error::TrySendError::Closed(_)) => return false,
        }
    }
    // Every channel was full; fall back to a blocking send on the first one.
    let Some(unsent) = current else { return true };
    match tx.first() {
        Some(first) => first.send(unsent).await.is_ok(),
        None => false,
    }
}

fn simple(status: StatusCode, msg: &str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(Full::new(Bytes::copy_from_slice(msg.as_bytes())))
        .unwrap()
}

fn cors_preflight(ctx: &DohHandlerCtx, origin: Option<&str>) -> Response<Full<Bytes>> {
    let mut resp = Response::builder()
        .status(StatusCode::NO_CONTENT)
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type, Accept")
        .header("Access-Control-Max-Age", "600")
        .body(Full::new(Bytes::new()))
        .unwrap();
    apply_cors(&mut resp, ctx, origin);
    resp
}

fn apply_cors(resp: &mut Response<Full<Bytes>>, ctx: &DohHandlerCtx, origin: Option<&str>) {
    if ctx.allowed_origins.is_empty() {
        return;
    }
    if let Some(o) = origin
        && ctx.allowed_origins.contains(o)
        && let Ok(v) = o.parse()
    {
        resp.headers_mut().insert("Access-Control-Allow-Origin", v);
    }
}
