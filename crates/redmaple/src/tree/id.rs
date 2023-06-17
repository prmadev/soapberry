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
/// assert_eq!(id, ID::new(id).inner());
///
/// assert_eq!(4usize, ID::new(id).inner().get_version_num());
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
/// assert!(!ID::new(id).inner().is_nil());
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

#[derive(
    Default,
    Debug,
    PartialEq,
    Eq,
    Clone,
    PartialOrd,
    Ord,
    serde::Deserialize,
    serde::Serialize,
    Hash,
)]
pub struct ID(Uuid);

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.0)
    }
}

impl ID {
    /// creats a new instance of the [`ID`]
    #[must_use]
    pub const fn new(id: Uuid) -> Self {
        Self(id)
    }

    /// Returns the uuid of this [`ID`].
    #[must_use]
    pub const fn inner(&self) -> Uuid {
        self.0
    }
}

/// Any object that implements this type can turn into id
pub trait IDGiver {
    /// valid ID type for the specific item
    type Valid: Clone;
    /// Returns a refrence to the underlying [`ID`]
    fn id(&self) -> &Self::Valid;
    /// Consumes self into its underlying [`ID`]
    fn into_id(self) -> Self::Valid;
}

/// extractor for resulting id
#[allow(clippy::module_name_repetitions)] // I cannot thing of a better name
pub fn result_id<I: IDGiver, E>(x: Result<I, E>) -> Option<I::Valid> {
    Some(x.ok()?.into_id())
}

impl From<Uuid> for ID {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl From<ID> for Uuid {
    fn from(value: ID) -> Self {
        value.inner()
    }
}
