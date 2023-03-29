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
