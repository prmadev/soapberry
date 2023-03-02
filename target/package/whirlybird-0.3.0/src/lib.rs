//! `whirlybird` is a library of generic type implementations for [`RedMaple`](https://crates.io/crates/redmaple) library, that offers workflows for different events compatible with common contents like todos, blogs and etc.
//!
//! `whirlybird` is still in its infancy. And for now, at least, it is not fully formed.
//! There is a 100% certainty that if I can, I will strip away some items in it.
//! So please, do not use it for now. Version numbering will tell you if things got stabilised.
//!

#![deny(missing_docs)]
#![deny(clippy::expect_used)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::panic)]
#![warn(
    rust_2018_idioms,
    clippy::pedantic,
    clippy::cargo,
    clippy::clone_on_ref_ptr,
    clippy::default_numeric_fallback,
    clippy::string_to_string,
    clippy::unnecessary_self_imports,
    clippy::str_to_string,
    clippy::same_name_method,
    clippy::rc_buffer,
    clippy::panic_in_result_fn,
    clippy::multiple_inherent_impl,
    clippy::map_err_ignore,
    clippy::if_then_some_else_none,
    clippy::empty_structs_with_brackets,
    clippy::useless_let_if_seq,
    clippy::use_self,
    clippy::missing_const_for_fn,
    clippy::cognitive_complexity,
    clippy::self_named_constructors
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod argument;

#[allow(clippy::panic)]
#[cfg(test)]
mod tests {

    use argument::Argument;
    use redmaple::{
        store::{EventStorage, FindError, SaveError},
        {event_group::EventGroup, id::ID, RedMaple},
    };

    use crate::argument::views::Views;

    use super::*;

    struct ES {
        events: Vec<Argument>,
    }

    impl ES {
        fn new(events: Vec<Argument>) -> Self {
            Self { events }
        }
    }

    impl EventStorage<Argument, Views> for ES {
        fn id_exists(&self, id: &ID) -> bool {
            self.events.iter().any(|x| x.id() == id)
        }

        fn add_event(&mut self, event: Argument) -> Result<(), SaveError> {
            self.events.push(event);
            Ok(())
        }

        fn get_events(&self) -> Option<Vec<Argument>> {
            let e = &self.events;
            if e.is_empty() {
                return None;
            };
            Some(e.clone())
        }

        fn get_event(&self, id: &ID) -> Result<Argument, FindError> {
            match self.get_events() {
                Some(i) => i
                    .iter()
                    .find(|x| x.id() == id)
                    .ok_or(FindError::NotFound)
                    .map(std::clone::Clone::clone),

                None => Err(FindError::NotFound),
            }
        }

        fn get_redmaples(&self) -> Option<Vec<RedMaple<Argument, Views>>> {
            self.get_events().map(|ms| {
                ms.iter()
                    .filter_map(|x| match &x {
                        Argument::Created(f) => {
                            Some(RedMaple::new(f.view_mode().clone(), f.id().clone()))
                        }

                        _ => None,
                    })
                    .collect::<Vec<RedMaple<Argument, Views>>>()
            })
        }
    }

    #[test]
    fn it_works() {
        let mut es = ES::new(vec![]);

        let created_event = Argument::new_create_event();

        match es.add_event(created_event) {
            Ok(_) => (),
            Err(e) => panic!("could not add event: {e}"),
        };

        match es.get_events() {
            Some(e) => {
                if e.len().ne(&1) {
                    panic!("event list should be 1 but instead is {}", e.len())
                } else {
                }
            }
            None => panic!("list is empty"),
        };

        match es.get_redmaples() {
            Some(f) => println!("{:#?}", f),
            None => panic!("list is empty"),
        }
    }
}
