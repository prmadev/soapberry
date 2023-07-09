//! This package encompasses the various messages that are transmitted throughout the system.

mod change;
mod information;

pub use change::*;
pub use information::*;
use time::OffsetDateTime;

/// Represents valid requests that can be made to the system.
pub enum Request {
    /// A request to modify data, akin to commands in the CQRS pattern.
    Change((OffsetDateTime, Change)),
    /// A request to retrieve information, akin to queries in the CQRS pattern.
    Information(Information),
}
