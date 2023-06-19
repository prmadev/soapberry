//! messages hold the differnet messages that are passed through the system.

mod change;
mod information;

pub use change::*;
pub use information::*;

//
// # type declaration
//

/// Valid [`Request`]s that can be made to the system
pub enum Request {
    /// A request for change in data.
    /// this is similar to commands in CQRS
    Change(Change),
    /// A request for information.
    /// this is similar to queries in CQRS
    Information(Information),
}
