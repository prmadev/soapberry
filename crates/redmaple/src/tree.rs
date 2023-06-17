//! redmaple is the central data-structure that is underlying the whole crate

use self::{
    event_group::EventGroup,
    id::{IDGiver, ID},
};
use std::fmt::Debug;

/// event module holds the types and functions that events could take and the operations that they
/// can do.
pub mod event_group;

/// id module holds the implementation of ID type
pub mod id;

/// versioned keeps the version of an stateful item
/// `RedMaple` is essentially a series of related events that form a state
///
/// * `id`: of type ID
/// * `events`: a list of entities that happened in time series
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RedMaple<T: EventGroup + Sized + Clone> {
    id: ValidRedMapleID,
    events: Vec<T>,
}

impl<T: EventGroup + Sized + Clone> RedMaple<T> {
    /// Creates a new instance of [`RedMaple`]
    ///
    /// * `view_mode`: sets the view mode of the `RedMaple`
    #[must_use]
    pub const fn new(id: ID, events: Vec<T>) -> Self {
        Self {
            id: ValidRedMapleID(id),
            events,
        }
    }

    // /// Gets the ID of the `RedMaple`
    // #[must_use]
    // pub const fn id(&self) -> &ValidRedMapleID {
    //     &self.id
    // }

    /// Gets an array of the events of the `RedMaple`
    #[must_use]
    pub const fn events(&self) -> &Vec<T> {
        &self.events
    }
}

/// A thin wrapper around [`ID`] that validates that the [`ID`] is coming from an [`Entry`]
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ValidRedMapleID(ID);

impl ValidRedMapleID {
    /// exposes the inner [`ID`] of the [`Entry`]
    #[must_use]
    pub const fn inner(&self) -> &ID {
        &self.0
    }
}

impl<T: EventGroup + Sized + Clone> IDGiver for RedMaple<T> {
    type Valid = ValidRedMapleID;

    fn id(&self) -> &Self::Valid {
        &self.id
    }

    fn into_id(self) -> Self::Valid {
        self.id
    }
}

// /// Error type when a dealing with a subscriber
// #[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
// pub enum SubscriberError {
//     /// when subscriber is in the list
//     #[error("Could not find the subscriber you are looking for: {0}")]
//     CouldNotFindSubscriber(ID),
// }

// /// [`SubscriberList`] is wrapper around  `Vec<ID>` which is there to ensure that the subscriber
// /// list follows some gurantees, like not having duplicates and being ordered.
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct SubscriberList(Vec<ID>);
// impl SubscriberList {
//     /// Creates [`SubscriberLists`] but first sorts the given [`ID`] list and and then checks for
//     /// duplicated subscribers, if found removes duplicates.
//     ///
//     /// # Parameters
//     ///
//     /// ## `D` or `deduplicator`
//     /// this function will be used inside a fold function after the list was sorted.
//     /// so for a case of a sorted list you can just use:
//     ///
//     ///```
//     /// use redmaple::id::ID;
//     /// |mut list:Vec<ID> , item: &ID| {
//     ///   if list.last() != Some(item) {
//     ///     list.push(item.to_owned());
//     ///   };
//     ///   list
//     /// };
//     ///```
//     /// note however that `list.last()` is the only item it will check. So if the list is not
//     /// sorted it will fail to detect duplicates.
//     /// In case you don't want a sorted list you can instead use this:
//     ///
//     ///```
//     /// use  redmaple::id::ID;
//     /// let a = |mut list: Vec<ID>, item: &ID| {
//     ///   if !list.contains(item)  {
//     ///     list.push(item.to_owned());
//     ///   };
//     ///   list
//     /// };
//     ///
//     ///```
//     /// this is less performant because all of the new list will be checked before a new item is
//     /// added. While for the sorted list you only need to check if the last item is equal to the
//     /// current one.
//     ///
//     /// you might also not want to deduplicate in which case you should just add items to the list,
//     /// regardless:
//     ///
//     ///```
//     /// use redmaple::id::ID;
//     /// |mut list: Vec<ID>, item: &ID| {
//     ///   list.push(item.to_owned());
//     ///   list
//     /// };
//     ///```
//     ///
//     ///
//     /// ## `S` or `sorter`
//     ///
//     /// This function will be used to sort the list before it gets deduplicated.
//     ///
//     /// An example would be:
//     ///
//     /// ```
//     /// use redmaple::id::ID;
//     /// |a: &ID , b: &ID| {
//     ///   Ord::cmp(a, b)
//     /// };
//     /// ```
//     ///
//     #[must_use]
//     pub fn new<S, D>(members: &[ID], sorter: S, deduplicator: D) -> Self
//     where
//         S: FnMut(&&ID, &&ID) -> Ordering,
//         D: FnMut(Vec<ID>, &ID) -> Vec<ID>,
//     {
//         Self(members.iter().sorted_by(sorter).fold(vec![], deduplicator))
//     }

//     /// a very simple sorted which compares the first ID with the second ID
//     #[allow(clippy::trivially_copy_pass_by_ref)]
//     #[must_use]
//     pub fn simple_sorter(a: &&ID, b: &&ID) -> Ordering {
//         Ord::cmp(a, b)
//     }

//     /// a very simple deduplicator that sees if the last id in the list is the same as the one
//     /// that is given, if so, it will add the item to the list and return the list.
//     ///
//     /// You should note that this function does not "seem" to be purely functional as it takes a
//     /// mutable list. However, you should consider that the variable is now owned by the function.
//     /// as such it is not a shared mutable reference between different functions.
//     /// This will not comprimise the state of the application, While at the sametime improves
//     /// performance considerably by not cloning every single element time and times again.
//     ///
//     /// another approach could be to copy around the list on each item. which is more "pure". but
//     /// misses the point of "pureness" for a worse performance.
//     #[must_use]
//     pub fn simple_deduplicator(mut list: Vec<ID>, item: &ID) -> Vec<ID> {
//         if list.last() != Some(item) {
//             list.push(item.clone());
//         }
//         list
//     }

//     /// Creates a reference to see the inner vector.
//     #[must_use]
//     pub const fn inner(&self) -> &Vec<ID> {
//         &self.0
//     }

//     /// Returns the inner vector and consumes itself in the process.
//     #[must_use]
//     #[allow(clippy::missing_const_for_fn)] // currently a destructor method cannot be const
//     pub fn into_inner(self) -> Vec<ID> {
//         self.0
//     }
// }
// #[cfg(test)]
// mod tests {
//     use uuid::Uuid;

//     use super::*;

//     #[test]
//     fn make_empty_subscribers_list() {
//         let empty_list: Vec<ID> = vec![];
//         let empty_subscribers_list = SubscriberList::new(
//             &empty_list,
//             SubscriberList::simple_sorter,
//             SubscriberList::simple_deduplicator,
//         );

//         assert_eq!(empty_subscribers_list.into_inner(), vec![]);
//     }

//     #[test]
//     fn make_a_sorted_list() {
//         let (item1, item2, item3, item4) = (
//             ID::new(Uuid::new_v4()),
//             ID::new(Uuid::new_v4()),
//             ID::new(Uuid::new_v4()),
//             ID::new(Uuid::new_v4()),
//         );
//         let mut sorted_list = vec![item1.clone(), item2.clone(), item3.clone(), item4.clone()];
//         sorted_list.sort();

//         let full_list: Vec<ID> = vec![item1, item2, item3, item4];
//         let new_subscribers_list = SubscriberList::new(
//             &full_list,
//             SubscriberList::simple_sorter,
//             SubscriberList::simple_deduplicator,
//         );
//         assert_eq!(new_subscribers_list.into_inner(), sorted_list);
//     }

//     #[test]
//     fn make_a_sorted_deduplicated_list() {
//         let (item1, item2, item3, item4) = (
//             ID::new(Uuid::new_v4()),
//             ID::new(Uuid::new_v4()),
//             ID::new(Uuid::new_v4()),
//             ID::new(Uuid::new_v4()),
//         );
//         let mut sorted_list = vec![
//             item1.clone(),
//             item2.clone(),
//             item3.clone(),
//             item4.clone(),
//             item2.clone(),
//         ];
//         sorted_list.sort();
//         sorted_list.dedup();

//         let full_list: Vec<ID> = vec![item1, item2, item3, item4];
//         let new_subscribers_list = SubscriberList::new(
//             &full_list,
//             SubscriberList::simple_sorter,
//             SubscriberList::simple_deduplicator,
//         );
//         assert_eq!(new_subscribers_list.into_inner(), sorted_list);
//     }
// }

///
pub trait RedMaplePrinter {
    ///
    type EventType: EventGroup + Clone;

    ///
    fn printer(&self, data: &RedMaple<Self::EventType>) -> String;
}
