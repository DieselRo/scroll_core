use std::collections::VecDeque;
use std::time::Duration;

use crate::core::cost_manager::InvocationCost;
use crate::invocation::types::Invocation;
use crate::sessions::database;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::mpsc::{self, error::TrySendError};
use tokio::task::JoinHandle;
use tracing::warn;

/// Event containing the invocation and its assessed cost.
#[derive(Clone)]
pub struct LedgerEvent {
    pub invocation: Invocation,
    pub cost: InvocationCost,
}

#[async_trait::async_trait]
pub trait LedgerWriter: Send + Sync + 'static {
    async fn write(&self, event: LedgerEvent) -> Result<(), sea_orm::DbErr>;
}

/// Default writer that persists events via SeaORM.
pub struct SeaOrmLedgerWriter;

#[async_trait::async_trait]
impl LedgerWriter for SeaOrmLedgerWriter {
    async fn write(&self, event: LedgerEvent) -> Result<(), sea_orm::DbErr> {
        crate::invocation::ledger::log_invocation_db(&event.invocation, &event.cost).await
    }
}

/// Cloneable handle for sending ledger events.
#[derive(Clone)]
pub struct LedgerHandle {
    sender: mpsc::Sender<LedgerEvent>,
}

impl LedgerHandle {
    pub fn try_log(&self, event: LedgerEvent) -> Result<(), TrySendError<LedgerEvent>> {
        self.sender.try_send(event)
    }
}

/// Service owning a dedicated runtime and worker task.
pub struct LedgerService {
    rt: Runtime,
    worker: JoinHandle<()>,
}

impl LedgerService {
    /// Shut down the worker, waiting up to `timeout` for pending writes.
    pub fn shutdown(self, timeout: Duration) {
        self.rt.block_on(async {
            let _ = self.worker.await;
        });
        self.rt.shutdown_timeout(timeout);
    }
}

/// Starts the ledger service with the given channel and staging capacities.
pub fn start(chan_capacity: usize, staging_capacity: usize) -> (LedgerHandle, LedgerService) {
    let rt = Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("ledger rt");
    let (tx, mut rx) = mpsc::channel::<LedgerEvent>(chan_capacity);
    let writer = SeaOrmLedgerWriter;

    let worker = rt.spawn(async move {
        let mut staging: VecDeque<LedgerEvent> = VecDeque::new();
        while let Some(event) = rx.recv().await {
            if !database::is_initialized() {
                if staging.len() >= staging_capacity {
                    staging.pop_front();
                }
                staging.push_back(event);
                continue;
            }

            // Flush any staged events first
            while let Some(ev) = staging.pop_front() {
                if let Err(e) = writer.write(ev).await {
                    warn!("ledger write failed: {e}");
                }
            }

            if let Err(e) = writer.write(event).await {
                warn!("ledger write failed: {e}");
            }
        }

        // Final flush on shutdown
        if database::is_initialized() {
            while let Some(ev) = staging.pop_front() {
                if let Err(e) = writer.write(ev).await {
                    warn!("ledger write failed: {e}");
                }
            }
        }
    });

    let handle = LedgerHandle { sender: tx };
    let service = LedgerService { rt, worker };
    (handle, service)
}
