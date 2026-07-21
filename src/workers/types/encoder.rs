use crate::exceptions::SCloudException;
use crate::workers::SCloudWorker;
use crate::workers::task::InFlightTask;
use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn run_dns_encoder(
    _worker: Arc<SCloudWorker>,
    mut rx: Vec<mpsc::Receiver<InFlightTask>>,
    tx: Vec<mpsc::Sender<InFlightTask>>,
) -> Result<(), SCloudException> {
    loop {
        for rx_channel in rx.iter_mut() {
            while let Some(msg) = rx_channel.recv().await {
                let mut current = Some(msg);

                for tx_channel in tx.iter() {
                    match tx_channel.try_send(current.take().unwrap()) {
                        Ok(_) => break,
                        Err(mpsc::error::TrySendError::Full(returned)) => {
                            current = Some(returned);
                        }
                        Err(mpsc::error::TrySendError::Closed(_)) => {
                            return Ok(());
                        }
                    }
                }

                if let Some(unsent) = current
                    && tx[0].send(unsent).await.is_err()
                {
                    return Ok(());
                }
            }
        }
    }
}
