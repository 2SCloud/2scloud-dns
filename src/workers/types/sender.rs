use crate::exceptions::SCloudException;
use crate::log_debug;
use crate::workers::SCloudWorker;
use crate::workers::reply_registry;
use crate::workers::task::InFlightTask;
use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn run_dns_sender(
    worker: Arc<SCloudWorker>,
    mut rx: Vec<mpsc::Receiver<InFlightTask>>,
) -> Result<(), SCloudException> {
    let _ = worker;
    loop {
        for rx_channel in rx.iter_mut() {
            while let Some(msg) = rx_channel.recv().await {
                let tag = msg.task.reply_to.as_deref().unwrap_or("");
                match tag {
                    reply_registry::REPLY_TAG_DOH => {
                        if let Some(sender) = reply_registry::take(&msg.task.task_id) {
                            let _ = sender.send(msg.task.payload.clone());
                        } else {
                            log_debug!(
                                "sender: no registered reply channel for task {}",
                                msg.task.task_id
                            );
                        }
                    }
                    _ => {
                        // TODO: UDP / TCP reply path — not implemented yet.
                    }
                }
            }
        }
    }
}
