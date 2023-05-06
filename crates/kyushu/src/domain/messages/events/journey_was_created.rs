// //! indicates that a journey was created at some point in time

// use std::{fmt::Display, time::SystemTime};

// use redmaple::id::{IDGiver, ID};
// use whirlybird::journey::{title::Title, Journey};

// use super::ValidEventID;

// /// Event recording that a journey was created
// pub struct JourneyWasCreated {
//     event_id: ValidEventID,
//     time_created: SystemTime,
//     title: Option<Title>,
//     journey_id: ID,
// }

// impl JourneyWasCreated {
//     /// reutrns a valid EventID
//     #[must_use]
//     pub const fn event_id(&self) -> &ValidEventID {
//         &self.event_id
//     }

//     /// returns time that the event happened
//     #[must_use]
//     pub const fn time_created(&self) -> SystemTime {
//         self.time_created
//     }

//     /// returns the title of journey
//     #[must_use]
//     pub const fn title(&self) -> &Option<Title> {
//         &self.title
//     }

//     /// returns ID of journey
//     #[must_use]
//     pub const fn journey_id(&self) -> &ID {
//         &self.journey_id
//     }
// }

// impl IDGiver for JourneyWasCreated {
//     type Valid = ValidEventID;

//     #[must_use]
//     fn id(&self) -> &Self::Valid {
//         &self.event_id
//     }

//     #[must_use]
//     fn into_id(self) -> Self::Valid {
//         self.event_id
//     }
// }

// pub trait JourneyWasCreatedRepository {
//     /// error that contains indication of problem with adding or retrieving items from repo
//     type Error: Display + std::error::Error;

//     /// returns a specific [`JourneyWasCreated`] from repo
//     fn query_id(&self, id: &ID) -> Result<JourneyWasCreated, Self::Error>;
//     /// adds a [`JoruneyWasCreated`] to the repo
//     fn add(&self, item: &JourneyWasCreated) -> Result<(), Self::Error>;
// }

// impl From<JourneyWasCreated> for Journey {
//     fn from(value: JourneyWasCreated) -> Self {
//         Journey::new(
//             value.journey_id.clone(),
//             value.time_created().clone(),
//             value.title().clone(),
//         )
//     }
// }
