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

    /// Command to add a link to a maple to another maple.
    AddLinkToMaple {
        /// The origin maple.
        from: ID,
        /// The Maple we are pointing to.
        to: ID,
        /// the explanantion of linking
        why: String,
    },

    /// Link to remove
    Dislink {
        /// ID of the link to remove
        link_id: ID,
    },
}
