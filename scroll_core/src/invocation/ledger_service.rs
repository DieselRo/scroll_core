use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::core::cost_manager::InvocationCost;
use crate::invocation::ledger::log_invocation_db;
use crate::invocation::types::Invocation;
use crate::sessions::database;

/// Event containing an invocation and its cost.
#[derive(Clone)]
pub struct LedgerEvent {
    pub invocation: Invocation,
    pub cost: InvocationCost,
}

#[async_trait]
pub trait LedgerWriter: Send + Sync + 'static {
    async fn write(&self, event: &LedgerEvent) -> Result<(), sea_orm::DbErr>;
}

pub struct SeaOrmLedgerWriter;

#[async_trait]
impl LedgerWriter for SeaOrmLedgerWriter {
    async fn write(&self, event: &LedgerEvent) -> Result<(), sea_orm::DbErr> {
        log_invocation_db(&event.invocation, &event.cost).await
    }
}

#[derive(Clone)]
pub struct LedgerHandle {
    tx: mpsc::Sender<LedgerEvent>,
}

impl LedgerHandle {
    pub fn try_log(
        &self,
        event: LedgerEvent,
    ) -> Result<(), mpsc::error::TrySendError<LedgerEvent>> {
        self.tx.try_send(event)
    }
}

pub struct LedgerService {
    handle: Option<tokio::task::JoinHandle<()>>,
    runtime: Option<tokio::runtime::Runtime>,
}

impl LedgerService {
    pub fn shutdown(mut self, timeout: Duration) {
        if let (Some(rt), Some(handle)) = (self.runtime.take(), self.handle.take()) {
            let _ = rt.block_on(async { let _ = tokio::time::timeout(timeout, handle).await; });
        }
    }
}

/// Starts the ledger service with given channel and staging capacities.
pub fn start(
    chan_capacity: usize,
    staging_capacity: usize,
) -> (LedgerHandle, LedgerService) {
    start_with_writer(chan_capacity, staging_capacity, Arc::new(SeaOrmLedgerWriter))
}

fn start_with_writer(
    chan_capacity: usize,
    staging_capacity: usize,
    writer: Arc<dyn LedgerWriter>,
) -> (LedgerHandle, LedgerService) {
    let (tx, mut rx) = mpsc::channel::<LedgerEvent>(chan_capacity);
    let rt = tokio::runtime::Runtime::new().expect("runtime");

    let handle = rt.spawn(async move {
        let mut staging: VecDeque<LedgerEvent> = VecDeque::with_capacity(staging_capacity);
        loop {
            if !database::is_initialized() {
                if staging_capacity > 0 {
                    match rx.recv().await {
                        Some(ev) => {
                            if staging.len() == staging_capacity {
                                staging.pop_front();
                            }
                            staging.push_back(ev);
                        }
                        None => {
                            if database::is_initialized() {
                                while let Some(ev) = staging.pop_front() {
                                    if let Err(e) = writer.write(&ev).await {
                                        tracing::warn!("ledger write failed: {}", e);
                                    }
                                }
                            }
                            break;
                        }
                    }
                } else {
                    if rx.is_closed() {
                        break;
                    }
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
                continue;
            }

            while let Some(ev) = staging.pop_front() {
                if let Err(e) = writer.write(&ev).await {
                    tracing::warn!("ledger write failed: {}", e);
                }
            }

            match rx.recv().await {
                Some(ev) => {
                    if let Err(e) = writer.write(&ev).await {
                        tracing::warn!("ledger write failed: {}", e);
                    }
                }
                None => {
                    while let Some(ev) = staging.pop_front() {
                        if let Err(e) = writer.write(&ev).await {
                            tracing::warn!("ledger write failed: {}", e);
                        }
                    }
                    break;
                }
            }
        }
    });

    (
        LedgerHandle { tx },
        LedgerService {
            handle: Some(handle),
            runtime: Some(rt),
        },
    )
}
