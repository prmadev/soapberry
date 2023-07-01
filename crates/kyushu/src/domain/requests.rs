//! This package encompasses the various messages that are transmitted throughout the system.

mod change;
mod information;

pub use change::*;
pub use information::*;

/// Represents valid requests that can be made to the system.
pub enum Request {
    /// A request to modify data, akin to commands in the CQRS pattern.
    Change(Change),
    /// A request to retrieve information, akin to queries in the CQRS pattern.
    Information(Information),
}
