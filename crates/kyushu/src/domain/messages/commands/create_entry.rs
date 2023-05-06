//! representation of request for creating an entry

use whirlybird::journey::{body::Body, title::Title};

/// requests an entry to be created
#[derive(Debug, Clone)]
pub struct CreateEntry {
    entry_title: Option<Title>,
    body: Option<Body>,
}

impl CreateEntry {
    /// Creates a new [`Entry`]
    #[must_use]
    pub const fn new(entry_title: Option<Title>, body: Option<Body>) -> Self {
        Self { entry_title, body }
    }

    /// returns the [`entry_title`] required
    #[must_use]
    pub const fn entry_title(&self) -> &Option<Title> {
        &self.entry_title
    }

    /// returns the [`Body`] required
    #[must_use]
    pub const fn body(&self) -> &Option<Body> {
        &self.body
    }
}
