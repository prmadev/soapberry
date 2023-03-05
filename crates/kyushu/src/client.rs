//! `kyushu` is an opinionated way of journaling and contemplating about your life.
//!
//! this is a client library made to act as way to communicate with the daemon and be used by the
//! users
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

use kyushu::api::{health_check_service_client::HealthCheckServiceClient, Marco, MarcoPoloRequest};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HealthCheckServiceClient::connect("http://[::1]:8002").await?;
    let request = tonic::Request::new(MarcoPoloRequest {
        marco: Some(Marco {
            content: String::from("marco"),
        }),
    });
    let response = client.marco_polo(request).await?;
    println!("RESPONSE={response:?}");

    Ok(())
}
