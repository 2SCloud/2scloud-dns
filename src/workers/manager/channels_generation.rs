use crate::exceptions::SCloudException;
use crate::workers::{SCloudWorker, WorkerType};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;

pub(crate) async fn generate_channels(
    workers: Vec<Arc<SCloudWorker>>,
) -> Result<(), SCloudException> {
    let mut wl: HashMap<&str, Vec<Arc<SCloudWorker>>> = HashMap::new();
    for w in workers {
        let key = match &w.get_worker_type() {
            WorkerType::LISTENER => "listener",
            WorkerType::DECODER => "decoder",
            WorkerType::QUERY_DISPATCHER => "query-dispatcher",
            WorkerType::CACHE_LOOKUP => "cache-lookup",
            WorkerType::ZONE_MANAGER => "zone-manager",
            WorkerType::RESOLVER => "resolver",
            WorkerType::CACHE_WRITER => "cache-writer",
            WorkerType::ENCODER => "encoder",
            WorkerType::SENDER => "sender",
            WorkerType::CACHE_JANITOR => "cache-janitor",
            WorkerType::METRICS => "metrics",
            WorkerType::TCP_ACCEPTOR => "tcp-acceptor",
            WorkerType::DOH_ACCEPTOR => "doh-acceptor",
            WorkerType::NONE => "none",
        };
        wl.entry(key).or_default().push(Arc::clone(&w));
    }

    let default_worker = vec![Arc::new(SCloudWorker::new(WorkerType::NONE)?)];
    let listener = wl.get("listener").cloned();
    let tcp_acceptor = wl.get("tcp-acceptor").unwrap_or(&default_worker);
    let doh_acceptor = wl.get("doh-acceptor").cloned();
    let decoder = wl.get("decoder").unwrap_or(&default_worker);
    let query_dispatcher = wl.get("query-dispatcher").unwrap_or(&default_worker);
    let cache_lookup = wl.get("cache-lookup").unwrap_or(&default_worker);
    let cache_writers = wl.get("cache-writer").unwrap_or(&default_worker);
    let zone_manager = wl.get("zone-manager").unwrap_or(&default_worker);
    let resolvers = wl.get("resolver").unwrap_or(&default_worker);
    let encoders = wl.get("encoder").unwrap_or(&default_worker);
    let senders = wl.get("sender").unwrap_or(&default_worker);

    // Helper: wire N producers -> M consumers
    // Each producer gets M senders, each consumer gets N receivers
    async fn wire(
        producers: &[Arc<SCloudWorker>],
        consumers: &[Arc<SCloudWorker>],
        capacity: usize,
    ) {
        for p in producers {
            let mut txs = Vec::new();
            for c in consumers {
                let (tx, rx) = mpsc::channel(capacity);
                c.push_dns_rx(rx).await;
                txs.push(tx);
            }
            p.push_dns_tx_many(txs).await;
        }
    }

    wire(tcp_acceptor, decoder, 1024).await;
    if let Some(udp) = listener.as_deref() {
        wire(udp, decoder, 1024).await;
    }
    if let Some(doh) = doh_acceptor.as_deref() {
        wire(doh, decoder, 1024).await;
    }
    wire(decoder, cache_lookup, 1024).await;
    wire(cache_lookup, cache_writers, 1024).await; // tx[0] = miss path
    wire(cache_lookup, query_dispatcher, 1024).await; // tx[1] = hit path
    wire(query_dispatcher, zone_manager, 1024).await;
    wire(query_dispatcher, resolvers, 1024).await;
    wire(zone_manager, cache_writers, 1024).await;
    wire(resolvers, cache_writers, 1024).await;
    wire(cache_writers, encoders, 1024).await;
    wire(encoders, senders, 1024).await;

    Ok(())
}
