use bytes::Bytes;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use tokio::sync::oneshot;
use uuid::Uuid;

pub const REPLY_TAG_DOH: &str = "doh";

static REGISTRY: Lazy<DashMap<Uuid, oneshot::Sender<Bytes>>> = Lazy::new(DashMap::new);

pub fn register(task_id: Uuid) -> oneshot::Receiver<Bytes> {
    let (tx, rx) = oneshot::channel();
    REGISTRY.insert(task_id, tx);
    rx
}

pub fn take(task_id: &Uuid) -> Option<oneshot::Sender<Bytes>> {
    REGISTRY.remove(task_id).map(|(_, v)| v)
}

pub fn drop_entry(task_id: &Uuid) {
    REGISTRY.remove(task_id);
}
