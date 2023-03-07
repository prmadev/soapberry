//! [`telemetry`] is the subscriber component used for instrumentalizing the main server

use tracing::{
    subscriber::{set_global_default, SetGlobalDefaultError},
    Subscriber,
};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

/// this will initialize the subscriber created
///
/// # Errors
///
/// it may return errors from upstream library
pub fn init_subscriber(
    subscriber: impl Subscriber + Send + Sync,
) -> Result<(), SetGlobalDefaultError> {
    set_global_default(subscriber)
}

/// this function creates a subscriber item
pub fn get_subscriber<M>(name: String, env_filter: String, sink: M) -> impl Subscriber + Send + Sync
where
    M: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_env("KYUSHU_LOG").unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}
