//! queries for a list of entries, using a set of filters

use std::time::{SystemTime, SystemTimeError};

/// request a list of [`Entry`]
#[derive(Debug, Clone)]
pub struct GetEntries {
    // RequestingUser: VerifiedUserID, // TODO
    sorted_by: SortingCriteria,
    filtered_by: Vec<Filter>,
    number: Option<u32>,
}

impl GetEntries {
    /// returns the sorting criteria
    #[must_use]
    pub const fn sorted_by(&self) -> &SortingCriteria {
        &self.sorted_by
    }

    /// filters that will reduce the enteries
    #[must_use]
    pub fn filtered_by(&self) -> &[Filter] {
        self.filtered_by.as_ref()
    }

    /// number of entries shown
    #[must_use]
    pub const fn number(&self) -> Option<u32> {
        self.number
    }
}

/// critera for sorting
#[derive(Debug, Clone)]
pub enum SortingCriteria {
    /// based on the time of its creation
    Created(Order),
    /// based on the last update of its creation
    Updated(Order),
}

/// filters that will be applied to the entries
#[derive(Debug, Clone)]
pub enum Filter {
    /// based on the time range of its creation
    ByTimeCreated(TimeRange),
    /// based on the time of its update
    ByTimeUpdated(TimeRange),
}

/// a range of time, where the first time is neccessarily before the second one
#[derive(Debug, Clone)]
pub struct TimeRange(SystemTime, SystemTime);

impl TimeRange {
    /// Either builds a [`TimeRange`] or returns error
    ///
    /// # Errors
    ///
    /// * [`SystemTimeError`] happens if the `first` in not before `second`.
    ///
    pub fn build(first: SystemTime, second: SystemTime) -> Result<Self, SystemTimeError> {
        match second.duration_since(first) {
            Ok(_) => Ok(Self(first, second)),
            Err(e) => Err(e),
        }
    }

    /// returns the a reference to the start of the [`TimeRange`]
    #[must_use]
    pub const fn start(&self) -> &SystemTime {
        &self.0
    }

    /// returns the a reference to the end of the [`TimeRange`]
    #[must_use]
    pub const fn end(&self) -> &SystemTime {
        &self.1
    }
}

/// ordering
#[derive(Debug, Clone)]
pub enum Order {
    /// least comes first
    Ascending,
    /// most comes first
    Descenting,
}
