//! `api` is a module wrapper to import `tonic-build` generated GRPC SDKs to be used by both client
//! and server binaries.

// the reason for this is that this module will import generated code which does not need to adhere
// to the standards of user-written code
#![allow(
    clippy::perf,
    clippy::style,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery,
    clippy::complexity,
    clippy::restriction,
    missing_docs
)]
tonic::include_proto!("health.v1");
tonic::include_proto!("journey.v1");

/// a set of helper function to deal with apis
pub mod helper {
    /// Converts a [`crate::api::Journey`]'s id  from `Option<String>` into `Result<String, E>`
    ///
    /// # Errors
    ///
    /// This function will return an error if if the id field is `Option::None`.
    #[allow(clippy::needless_pass_by_value)]
    pub fn journey_id_extractor<E>(
        x: crate::api::Journey,
        when_id_not_found: E,
    ) -> Result<String, E> {
        Ok(x.id.ok_or(when_id_not_found)?.id)
    }
}
