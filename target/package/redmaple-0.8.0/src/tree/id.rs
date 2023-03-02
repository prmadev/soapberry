use std::fmt::{Debug, Display};

use uuid::Uuid;

/// The Implementation of the ID that the crate uses
///
/// # Example
///
/// For now, the implementation uses UUID v4 as the inner type.
///
/// ```rust
/// use redmaple::tree::id::ID;
///
/// assert_eq!(4usize, ID::new().into_inner().get_version_num());
/// ```
///
/// We should make sure to insure that the `ID::new()` never outputs a nil (all zero) id.
/// ```
/// use redmaple::tree::id::ID;
///
/// assert!(!ID::new().into_inner().is_nil());
/// ```
///
/// Of course all ID's should be unique.
/// ```
/// use redmaple::tree::id::ID;
///
/// assert_ne!(ID::new(), ID::new());
/// ```

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct ID(Uuid);

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.0)
    }
}

impl ID {
    /// creats a new instance of the ID
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Returns the uuid of this [`ID`].
    #[must_use]
    pub const fn into_inner(&self) -> Uuid {
        self.0
    }
}
