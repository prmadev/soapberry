//! [`Created`] is an special event which starts a new `RedMaple` and should be the first event of each
//! `RedMaple`.

use std::time::SystemTime;

use redmaple::id::ID;
use redmaple::view_mode::ViewMode;

/// Creates a new instance of Story
///
/// * `id`: is of type ID.
/// * `redmaple_id`: is of type ID.
#[derive(Clone, Debug)]
pub struct Created<V: ViewMode + Sized + Clone> {
    id: ID,
    created: SystemTime,
    redmaple_id: ID,
    view_mode: V,
}

impl<V: ViewMode + Sized + Clone> Created<V> {
    /// Creates a new [`Created`] event
    ///
    /// * `view_mode`: set the view mode for this `RedMaple` `ViewMode`
    /// * `redmaple_id`: set the id of the the parent redmaple
    #[must_use]
    pub fn new(view_mode: V, redmaple_id: ID) -> Self {
        Self {
            id: ID::new(),
            created: std::time::SystemTime::now(),
            redmaple_id,
            view_mode,
        }
    }

    /// returns the view mode of the parent redmaple
    pub const fn view_mode(&self) -> &V {
        &self.view_mode
    }

    /// returns the id of event
    #[must_use]
    pub const fn id(&self) -> &ID {
        &self.id
    }

    /// returns the id of the parent redmaple
    #[must_use]
    pub const fn redmaple_id(&self) -> &ID {
        &self.redmaple_id
    }

    // pub fn apply(
    //     &self,
    //     store: &dyn crate::store::StateStorage<>,
    // ) -> Result<(), crate::store::ApplyError> {
    //     store.apply(self)
    // }

    /// returns the creation time of event
    #[must_use]
    pub const fn created(&self) -> &SystemTime {
        &self.created
    }
}

#[cfg(test)]
mod tests {
    // use redmaple::view_mode::BlogMode;

    use redmaple::id::ID;

    use crate::argument::{
        maple_created::Created,
        views::{BlogMode, Views},
    };

    #[test]
    fn could_make_event() {
        let red_maple_id = ID::new();
        let new_event = Created::new(Views::Blog(BlogMode::Text), red_maple_id.clone());

        assert_eq!(new_event.redmaple_id(), &red_maple_id);
        assert_eq!(
            new_event.id().into_inner().to_string().len(),
            red_maple_id.into_inner().to_string().len()
        );
    }
}
