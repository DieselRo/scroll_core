// src/orchestra/mod.rs

pub mod bus;
pub mod message;

pub use bus::Bus;
pub use message::AgentMessage;

use crate::invocation::named_construct::NamedConstruct;

pub trait OrchestratedConstruct: NamedConstruct {
    fn attach_bus(&mut self, bus: Bus);
}
