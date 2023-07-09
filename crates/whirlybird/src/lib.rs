//! Whirlybird - A Fun and Dynamic Library for RedMaple-based Workflows
//!
//! Welcome to the world of Whirlybird, a library of generic type implementations designed to complement the powerful [`RedMaple`](https://crates.io/crates/redmaple) library. Whirlybird offers a delightful array of workflows tailored for common content types like todos, blogs, and more!
//!
//! But hold on tight, because Whirlybird is still in its infancy. It's a vibrant and evolving project that's constantly growing and transforming. As the creator, I have an insatiable urge to experiment and refine, which means there's a 100% certainty that I'll be trimming and reshaping this library along the way.
//!
//! So, for now, I kindly ask you not to use Whirlybird in production just yet. But fear not! The version numbering will be your trusty guide, indicating when things have stabilized and Whirlybird is ready to soar.
//!
//! Join me on this exciting journey as we unleash the power of Whirlybird and create captivating workflows that will leave you spinning with joy!
//!
//! 🚀✨
//!

#![forbid(unsafe_code)]
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
    clippy::self_named_constructors,
    clippy::cloned_instead_of_copied,
    clippy::iter_cloned_collect,
    clippy::implicit_clone,
    clippy::map_clone
)]
#![allow(clippy::multiple_crate_versions)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "journey")]
pub mod journey;
