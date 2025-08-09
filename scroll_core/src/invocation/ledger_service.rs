//! Async, non-panicking ledger pipeline with bounded channel and staging buffer.

use std::collections::VecDeque;
use std::time::Duration;

use tokio::sync::mpsc::{self, error::TrySendError, Receiver, Sender};
use tokio::task::JoinHandle;
use tokio::runtime::Runtime;

use crate::core::cost_manager::InvocationCost;
use crate::invocation::types::Invocation;
use crate::sessions::database;

/// Event type carried through the ledger pipeline.
#[derive(Clone, Debug)]
pub struct LedgerEvent {
    pub invocation: Invocation,
    pub cost: InvocationCost,
}

/// Write abstraction for the background worker.
#[async_trait::async_trait]
pub trait LedgerWriter: Send + Sync + 'static {
    async fn write(&self, ev: &LedgerEvent) -> Result<(), sea_orm::DbErr>;
}

/// SeaORM-backed writer using the existing entity/insert logic.
pub struct SeaOrmLedgerWriter;

#[async_trait::async_trait]
impl LedgerWriter for SeaOrmLedgerWriter {
    async fn write(&self, ev: &LedgerEvent) -> Result<(), sea_orm::DbErr> {
        // Safety: always gated by database::is_initialized() in the worker
        crate::invocation::ledger::log_invocation_db(&ev.invocation, &ev.cost).await
    }
}

/// Cloneable handle for hot paths to attempt non-blocking logging.
#[derive(Clone)]
pub struct LedgerHandle {
    tx: Sender<LedgerEvent>,
}

impl LedgerHandle {
    pub fn try_log(&self, event: LedgerEvent) -> Result<(), TrySendError<LedgerEvent>> {
        self.tx.try_send(event)
    }
}

/// Service guard to manage the background worker lifetime.
pub struct LedgerService {
    join: JoinHandle<()>,
    _rt: Runtime,
}

impl LedgerService {
    /// Attempt a graceful shutdown by waiting for the worker to stop after
    /// the handle is dropped (channel closed).
    pub async fn shutdown(self, timeout: Duration) {
        use tokio::time::{sleep, Instant};
        let start = Instant::now();
        loop {
            if self.join.is_finished() {
                let _ = self.join.await;
                break;
            }
            if start.elapsed() >= timeout {
                // Give up; task will be aborted on drop below.
                // Abort to avoid hanging shutdown.
                self.join.abort();
                break;
            }
            sleep(Duration::from_millis(10)).await;
        }
    }
}

/// Start the ledger service with a bounded channel and staging buffer.
///
/// - `chan_capacity`: max in-flight events before `try_log` returns Full
/// - `staging_capacity`: max buffered while DB is not ready (drop-oldest on overflow)
pub fn start(chan_capacity: usize, staging_capacity: usize) -> (LedgerHandle, LedgerService) {
    let (tx, rx) = mpsc::channel::<LedgerEvent>(chan_capacity);
    let handle = LedgerHandle { tx };

    // Dedicated single-thread runtime for the worker
    let rt = Runtime::new().expect("failed to create ledger runtime");
    let join = rt.spawn(worker::<SeaOrmLedgerWriter>(
        rx,
        staging_capacity,
        SeaOrmLedgerWriter,
    ));
    (handle, LedgerService { join, _rt: rt })
}

async fn worker<W: LedgerWriter>(mut rx: Receiver<LedgerEvent>, staging_capacity: usize, writer: W) {
    let mut staging: VecDeque<LedgerEvent> = VecDeque::with_capacity(staging_capacity);

    // Helper to drain staging once DB is ready
    async fn flush_staging<W: LedgerWriter>(
        staging: &mut VecDeque<LedgerEvent>,
        writer: &W,
    ) {
        if !database::is_initialized() {
            return;
        }
        while let Some(ev) = staging.pop_front() {
            if let Err(e) = writer.write(&ev).await {
                tracing::warn!(target: "ledger", "ledger write failed (flush): {e}");
                // Drop on error to avoid infinite retry loop; metrics can track later.
            }
        }
    }

    while let Some(ev) = rx.recv().await {
        if database::is_initialized() {
            // First, flush any buffered events
            flush_staging(&mut staging, &writer).await;
            // Then write the current event
            if let Err(e) = writer.write(&ev).await {
                tracing::warn!(target: "ledger", "ledger write failed: {e}");
                // Best-effort: drop on error.
            }
        } else {
            // DB not ready: stage with drop-oldest policy
            if staging.len() >= staging_capacity && staging_capacity > 0 {
                staging.pop_front();
            }
            if staging_capacity > 0 {
                staging.push_back(ev);
            } else {
                // No staging: drop immediately
            }
        }
    }

    // Channel closed: try one last flush
    flush_staging(&mut staging, &writer).await;
}

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

