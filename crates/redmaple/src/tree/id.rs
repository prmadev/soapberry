use std::fmt::Display;

/// The Implementation of the ID that the crate uses
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
pub struct ID(u64);

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.0)
    }
}

impl ID {
    /// creats a new instance of the [`ID`]
    #[must_use]
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the uuid of this [`ID`].
    #[must_use]
    pub const fn inner(&self) -> u64 {
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

impl From<u64> for ID {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<ID> for u64 {
    fn from(value: ID) -> Self {
        value.inner()
    }
}
