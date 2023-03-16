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
/// use uuid::Uuid;
///
/// let id = Uuid::new_v4();
///
/// assert_eq!(id, ID::new(id).into_inner());
///
/// assert_eq!(4usize, ID::new(id).into_inner().get_version_num());
/// ```
///
/// We should make sure to insure that the `ID::new()` never outputs a nil (all zero) id.
/// ```
/// use redmaple::tree::id::ID;
///
/// use uuid::Uuid;
///
/// let id = Uuid::new_v4();
///
/// assert!(!ID::new(id).into_inner().is_nil());
/// ```
///
/// Of course all ID's should be unique.
/// ```
/// use redmaple::tree::id::ID;
/// use uuid::Uuid;
///
/// let id1 = Uuid::new_v4();
/// let id2 = Uuid::new_v4();
///
///
/// assert_ne!(ID::new(id1), ID::new(id2));
/// ```

#[derive(Default, Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct ID(Uuid);

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.0)
    }
}

impl ID {
    /// creats a new instance of the ID
    #[must_use]
    pub const fn new(id: Uuid) -> Self {
        Self(id)
    }

    /// Returns the uuid of this [`ID`].
    #[must_use]
    pub const fn into_inner(&self) -> Uuid {
        self.0
    }
}
