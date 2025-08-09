use std::collections::VecDeque;
use std::time::Duration;

use async_trait::async_trait;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{self, error::TrySendError, Receiver, Sender};
use tokio::task::JoinHandle;
use tracing::warn;

use crate::core::cost_manager::InvocationCost;
use crate::invocation::ledger;
use crate::invocation::types::Invocation;
use crate::sessions::database;

/// Event containing the data needed for a ledger write.
pub struct LedgerEvent {
    pub invocation: Invocation,
    pub cost: InvocationCost,
}

#[async_trait]
pub trait LedgerWriter: Send + Sync + 'static {
    async fn write(&self, event: LedgerEvent);
}

pub struct SeaOrmLedgerWriter;

#[async_trait]
impl LedgerWriter for SeaOrmLedgerWriter {
    async fn write(&self, event: LedgerEvent) {
        if let Err(e) = ledger::log_invocation_db(&event.invocation, &event.cost).await {
            warn!("ledger write error: {e}");
        }
    }
}

/// Cloneable handle exposed to call sites for non-blocking logging.
#[derive(Clone)]
pub struct LedgerHandle {
    sender: Sender<LedgerEvent>,
}

impl LedgerHandle {
    pub fn try_log(&self, event: LedgerEvent) -> Result<(), TrySendError<LedgerEvent>> {
        self.sender.try_send(event)
    }
}

pub struct LedgerService {
    runtime: Runtime,
    worker: JoinHandle<()>,
}

impl LedgerService {
    pub fn shutdown(self, timeout: Duration) {
        self.runtime.block_on(async {
            let _ = tokio::time::timeout(timeout, self.worker).await;
        });
        self.runtime.shutdown_timeout(timeout);
    }
}

pub fn start(chan_capacity: usize, staging_capacity: usize) -> (LedgerHandle, LedgerService) {
    let (tx, rx) = mpsc::channel(chan_capacity);
    let runtime = Runtime::new().expect("failed to create runtime");
    let worker = runtime.spawn(worker(rx, staging_capacity, SeaOrmLedgerWriter));
    (
        LedgerHandle { sender: tx },
        LedgerService { runtime, worker },
    )
}

async fn worker<W>(mut rx: Receiver<LedgerEvent>, staging_capacity: usize, writer: W)
where
    W: LedgerWriter,
{
    let mut staging = VecDeque::new();
    while let Some(event) = rx.recv().await {
        if !database::is_initialized() {
            if staging.len() >= staging_capacity {
                staging.pop_front();
            }
            staging.push_back(event);
            continue;
        }

        flush_staging(&mut staging, &writer).await;
        writer.write(event).await;
    }

    if database::is_initialized() {
        flush_staging(&mut staging, &writer).await;
    }
}

async fn flush_staging<W>(staging: &mut VecDeque<LedgerEvent>, writer: &W)
where
    W: LedgerWriter,
{
    while let Some(ev) = staging.pop_front() {
        writer.write(ev).await;
    }
}
