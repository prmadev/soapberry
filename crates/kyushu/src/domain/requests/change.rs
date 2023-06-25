//! commands hold the formal message that goes through the system

use redmaple::id::ID;
use whirlybird::journey::{entity::maple::Maple, Body};

//
// # type declaration
//

/// changes that can be made
#[derive(Debug, Clone)]
pub enum Change {
    /// creating a new remaple
    CreateNewMaple(Maple),
    /// updates the body of a maple
    UpdateMapleBody(ID, Body),
}
