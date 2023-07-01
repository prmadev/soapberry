//! Module responsible for handling queries, which are types used to request information from the system.

/// Represents various types of information that can be extracted from the system.
pub enum Information {
    /// Request to retrieve a list of entries, sorted from first to last.
    ListEntries,
}
