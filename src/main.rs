// DNS record types and the worker/state enums use the protocol's standard
// uppercase names (A, AAAA, CNAME, TCP, ...); renaming them would hide the
// spec, so the acronym lint is allowed crate-wide.
#![allow(clippy::upper_case_acronyms)]
// The DNS protocol library and worker API are intentionally built ahead of the
// runtime that wires them together (pre-1.0); silence dead-code noise.
#![allow(dead_code)]

use crate::config::Config;
use crate::exceptions::SCloudException;
use crate::workers::manager::StartGate;
use crate::workers::{SCloudWorker, WorkerType};
use std::path::Path;
use std::sync::Arc;

mod config;
mod dns;
mod exceptions;
mod ui;
mod utils;
mod workers;

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() -> Result<(), SCloudException> {
    let config = Config::from_file(Path::new("./config/config.json"))?;
    utils::logging::init(config.logging.clone())?;

    if !config.logging.dyn_ui {
        println!(
            r#"
        ██████╗ ███████╗ ██████╗██╗      ██████╗ ██╗   ██╗██████╗
        ╚════██╗██╔════╝██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗  scloud-dns (v0.3.0)
         █████╔╝███████╗██║     ██║     ██║   ██║██║   ██║██║  ██║      org: https://github.com/2SCloud/
        ██╔═══╝ ╚════██║██║     ██║     ██║   ██║██║   ██║██║  ██║      rep: https://github.com/2SCloud/scloud-dns
        ███████╗███████║╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝      own: @onihilist
        ╚══════╝╚══════╝ ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝
        "#
        );
    } else {
        let _ = ratatui::run(|terminal| ui::App::default().run(terminal));
    }

    //#[cfg(target_os = "windows")]
    //{
    //    use tokio::net::UdpSocket;
    //    use std::sync::Arc;
    //    use workers::types::listener::SHARED_UDP_SOCKET;
    //    let udp = UdpSocket::bind("0.0.0.0:5353")
    //        .await
    //        .map_err(|_| SCloudException::SCLOUD_WORKER_LISTENER_BIND_FAILED)?;
    //    SHARED_UDP_SOCKET.set(Arc::new(udp)).ok();
    //}

    let gate = Arc::new(StartGate::new(1));

    let worker_specs: [(WorkerType, u16); 13] = [
        (WorkerType::LISTENER, config.workers.listener),
        (WorkerType::TCP_ACCEPTOR, config.workers.tcp_acceptor),
        (WorkerType::DOH_ACCEPTOR, config.workers.doh_acceptor),
        (WorkerType::DECODER, config.workers.decoder),
        (
            WorkerType::QUERY_DISPATCHER,
            config.workers.query_dispatcher,
        ),
        (WorkerType::CACHE_LOOKUP, config.workers.cache_lookup),
        (WorkerType::ZONE_MANAGER, config.workers.zone_manager),
        (WorkerType::RESOLVER, config.workers.resolver),
        (WorkerType::CACHE_WRITER, config.workers.cache_writer),
        (WorkerType::ENCODER, config.workers.encoder),
        (WorkerType::SENDER, config.workers.sender),
        (WorkerType::CACHE_JANITOR, config.workers.cache_janitor),
        (WorkerType::METRICS, config.workers.metrics),
    ];

    let total = worker_specs.iter().map(|(_, n)| *n as usize).sum();
    let mut workers: Vec<Arc<SCloudWorker>> = Vec::with_capacity(total);

    for (worker_type, count) in worker_specs {
        for _ in 0..count {
            workers.push(Arc::new(SCloudWorker::new(worker_type)?));
        }
    }

    workers::manager::channels_generation::generate_channels(workers.clone()).await?;
    workers.sort_by_key(|w| w.get_worker_id());

    let mut handles: Vec<tokio::task::JoinHandle<()>> = Vec::new();
    for w in workers {
        handles.push(workers::spawn_worker(w, gate.clone()));
    }

    futures_util::future::pending::<()>().await;
    Ok(())
}
