//! `kyushu` is an opinionated way of journaling and contemplating about your life.
//!
//! For now `kyushu` is still in its infancy. I suggest that you do not use it for now.
//! But it should be awesome when it reaches a  stable core.
//!
//! `kyushu` is proudly standing on the shoulders of [`RedMaple`](https://crates.io/crates/redmaple) library.

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

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
