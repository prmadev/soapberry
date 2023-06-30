//! [`link`] contains logic for [`Link`]

use crate::journey::ValidMapleID;

/// [`Link`] is the holder of information between two valid objects
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Link {
    redmaple_id: ValidMapleID,
    explanation: Option<String>,
}
