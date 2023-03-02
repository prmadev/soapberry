//! `RedMaple` offers an opinionated yet extremely flexible data modeling system based on events for backend applications
//!
//! `RedMaple` is still in its infancy. And for now, at least, it is not fully formed.
//! There is a 100 % certainty that if I can, I will strip away some items in it.
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

// pub mod argument;
pub mod store;
pub mod tree;
pub mod view_mode;
pub use tree::*;
