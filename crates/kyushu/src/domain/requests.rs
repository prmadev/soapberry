//! messages hold the differnet messages that are passed through the system.

mod change;
mod information;

pub use change::*;
pub use information::*;

//
// # type declaration
//

pub enum Request {
    Change(Change),
    Information(Information),
}
