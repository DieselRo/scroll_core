use std::collections::VecDeque;
use std::time::Duration;

use async_trait::async_trait;
use tokio::sync::mpsc::{self, error::TrySendError, Receiver, Sender};
use tracing::warn;

use crate::core::cost_manager::InvocationCost;
use crate::invocation::ledger;
use crate::invocation::types::Invocation;
use crate::sessions::database;

/// Event wrapping an invocation and its assessed cost.
#[derive(Clone)]
pub struct LedgerEvent {
    pub invocation: Invocation,
    pub cost: InvocationCost,
}

#[async_trait]
pub trait LedgerWriter: Send + Sync + 'static {
    async fn write(&self, event: &LedgerEvent) -> Result<(), sea_orm::DbErr>;
}

/// Default writer backed by SeaORM.
pub struct SeaOrmLedgerWriter;

#[async_trait]
impl LedgerWriter for SeaOrmLedgerWriter {
    async fn write(&self, event: &LedgerEvent) -> Result<(), sea_orm::DbErr> {
        ledger::log_invocation_db(&event.invocation, &event.cost).await
    }
}

/// Cloneable handle for sending ledger events.
#[derive(Clone)]
pub struct LedgerHandle {
    sender: Sender<LedgerEvent>,
}

impl LedgerHandle {
    pub fn try_log(&self, event: LedgerEvent) -> Result<(), TrySendError<LedgerEvent>> {
        self.sender.try_send(event)
    }
}

/// Background service running the worker task.
pub struct LedgerService {
    join_handle: Option<std::thread::JoinHandle<()>>,
}

impl LedgerService {
    /// Attempt to shut down the worker, waiting up to `timeout`.
    /// Currently this simply joins the worker thread.
    pub fn shutdown(mut self, _timeout: Duration) {
        if let Some(handle) = self.join_handle.take() {
            let _ = handle.join();
        }
    }
}

async fn worker_loop<W: LedgerWriter>(
    mut rx: Receiver<LedgerEvent>,
    staging_capacity: usize,
    writer: W,
) {
    let mut staging: VecDeque<LedgerEvent> = VecDeque::new();
    while let Some(event) = rx.recv().await {
        if !database::is_initialized() {
            if staging_capacity > 0 {
                staging.push_back(event);
                if staging.len() > staging_capacity {
                    staging.pop_front();
                }
            }
            continue;
        }

        while let Some(staged) = staging.pop_front() {
            if let Err(e) = writer.write(&staged).await {
                warn!("ledger write failed: {e}");
            }
        }

        if let Err(e) = writer.write(&event).await {
            warn!("ledger write failed: {e}");
        }
    }

    if database::is_initialized() {
        while let Some(staged) = staging.pop_front() {
            if let Err(e) = writer.write(&staged).await {
                warn!("ledger write failed: {e}");
            }
        }
    }
}

/// Starts the ledger service with a bounded channel and optional staging buffer.
pub fn start(
    chan_capacity: usize,
    staging_capacity: usize,
) -> (LedgerHandle, LedgerService) {
    let (tx, rx) = mpsc::channel(chan_capacity);
    let handle = LedgerHandle { sender: tx.clone() };

    let join_handle = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("ledger runtime");
        rt.block_on(worker_loop(rx, staging_capacity, SeaOrmLedgerWriter));
    });

    (
        handle,
        LedgerService {
            join_handle: Some(join_handle),
        },
    )
}

