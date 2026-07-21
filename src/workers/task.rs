use crate::exceptions::SCloudException;
use crate::workers::WorkerType;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use uuid::Uuid;

#[allow(unused)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SCloudWorkerTask {
    pub task_id: Uuid,
    pub for_type: WorkerType,
    pub for_who: SocketAddr,
    pub payload: Bytes,
    pub attempts: u8,
    pub max_attempts: u8,
    pub created_at: SystemTime,
    pub deadline_timeout: Option<SystemTime>,
    pub priority: u8,                   // if supported by the broker
    pub reply_to: Option<String>,       // response endpoint
    pub correlation_id: Option<String>, // id request/response
}

pub struct InFlightTask {
    pub task: SCloudWorkerTask,
    pub _permit: OwnedSemaphorePermit,
}

impl InFlightTask {
    pub async fn new(
        payload: &[u8],
        peer: SocketAddr,
        worker_type: WorkerType,
        sem: Arc<Semaphore>,
    ) -> Result<Self, SCloudException> {
        let permit = sem
            .acquire_owned()
            .await
            .map_err(|_| SCloudException::SCLOUD_WORKER_SEM_CLOSED)?;

        Ok(Self {
            task: SCloudWorkerTask {
                task_id: Uuid::new_v4(),
                for_type: worker_type,
                for_who: peer,
                payload: Bytes::copy_from_slice(payload),
                attempts: 0,
                max_attempts: 3,
                created_at: SystemTime::now(),
                deadline_timeout: None,
                priority: 0,
                reply_to: None,
                correlation_id: None,
            },
            _permit: permit,
        })
    }
}
