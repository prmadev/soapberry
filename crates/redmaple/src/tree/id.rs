use std::fmt::{Display, LowerHex};

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
pub struct ID(i128);

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl LowerHex for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl ID {
    /// creats a new instance of the [`ID`]
    #[must_use]
    pub const fn new(id: i128) -> Self {
        Self(id)
    }

    /// Returns the uuid of this [`ID`].
    #[must_use]
    pub const fn inner(&self) -> i128 {
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
// #[allow(clippy::module_name_repetitions)] // I cannot think of a better name
// impl From <Result_id<I:
// pub fn result_id<I: IDGiver, E>(x: Result<I, E>) -> Option<I::Valid> {
//     Some(x.ok()?.into_id())
// }

impl From<i128> for ID {
    fn from(value: i128) -> Self {
        Self::new(value)
    }
}

impl From<ID> for i128 {
    fn from(value: ID) -> Self {
        value.inner()
    }
}

impl From<time::OffsetDateTime> for ID {
    fn from(value: time::OffsetDateTime) -> Self {
        Self::new(value.unix_timestamp_nanos())
    }
}

impl TryFrom<ID> for time::OffsetDateTime {
    type Error = time::Error;
    fn try_from(value: ID) -> Result<Self, Self::Error> {
        Ok(Self::from_unix_timestamp_nanos(value.inner())?)
    }
}
