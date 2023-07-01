//! Module responsible for handling formal messages within the system.

use redmaple::id::ID;
use whirlybird::journey::entity::maple::Maple;
use whirlybird::journey::Body;

/// Represents various changes that can be made within the system.
#[derive(Debug, Clone)]
pub enum Change {
    /// Command to create a new maple entity.
    CreateNewMaple(Maple),
    /// Command to update the body of a maple entity.
    UpdateMapleBody(ID, Body),
}
