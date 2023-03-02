/// versioned keeps the version number of the state
pub trait Versioned {
    /// returns the version number of the state
    fn version(&self) -> u64;
    /// increments the version number of the state
    fn increment_version(&mut self);
}
