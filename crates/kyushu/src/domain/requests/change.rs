//! commands hold the formal message that goes through the system

use whirlybird::journey::entity::maple::Maple;

//
// # type declaration
//

/// changes that can be made
#[derive(Debug, Clone)]
pub enum Change {
    /// creating a new remaple
    CreateNewMaple(Maple),
}
