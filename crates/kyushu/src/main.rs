//! Kyushu is a local-first, plain-text, event-driven journaling tool for the modern age

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

use kyushu::{
    self,
    cli::{Args, EntryPrinter},
    config::Config,
    domain::requests::{Change, Request},
    persistence,
};
use redmaple::{
    event_group::EventGroup,
    id::{IDGiver, ID},
    EventRepo, RedMaple, RedMapleProjector,
};
use thiserror::Error;
use time::format_description;
use whirlybird::journey::{Event, EventWrapper};

#[allow(clippy::cast_sign_loss)] // timestamp is given in i64, but it can only be positive
fn main() -> color_eyre::Result<()> {
    // setting up loggers
    color_eyre::install()?;

    // Firing up generator ID

    // getting arguments
    let cli_arguments = Args::try_from(std::env::args_os())?;

    // getting configurations from cli_arguments
    let mut configurations = Config::from(cli_arguments.clone());
    let config_dir = dirs::config_dir();
    if let Some(config) = config_dir {
        configurations = Config::try_from(config.join("kyushu").join("config.json"))?;
    }

    // forming a request
    let req = cli_arguments.to_request()?;

    // creating persistence
    let repo = persistence::FileDB::try_from(
        configurations
            .file_store
            .ok_or(MainError::FileStoreCannotBeEmpty)?,
    )?;

    // matching requests to the appropiate functions
    match req {
        Request::Change(chng) => match chng {
            Change::CreateNewMaple(mpl) => {
                let new_id = mpl.id().inner();
                let created_time = time::OffsetDateTime::now_utc();
                repo.save(RedMaple::new(
                    new_id.clone(),
                    created_time,
                    vec![EventWrapper::new(
                        ID::from(created_time),
                        created_time,
                        Event::MapleCreated(mpl),
                    )],
                ))?;
                // saving in the permaenent storage
            }
        },

        Request::Information(i) => match i {
            kyushu::domain::requests::Information::ListEntries => {
                let mut redmaples = repo.all_events()?.values().collect::<Vec<_>>();

                redmaples.sort_by(|a, b| {
                    let at = a
                        .events()
                        .first()
                        .map_or(&time::OffsetDateTime::UNIX_EPOCH, EventWrapper::time);

                    let bt = b
                        .events()
                        .first()
                        .map_or(&time::OffsetDateTime::UNIX_EPOCH, EventWrapper::time);

                    at.cmp(bt)
                });

                redmaples
                    .iter()
                    .map(|rm| {
                        EntryPrinter::new(
                            true,
                            true,
                            format_description::parse(
                                "[year]-[month]-[day]:[hour]-[minute]-[second]",
                            )
                            .unwrap_or_default(),
                        )
                        .projector(rm)
                    })
                    .for_each(|each| println!("{each}"));
            }
        },
    };
    Ok(())
}

#[derive(Debug, Error)]
enum MainError {
    #[error("file store must be given")]
    FileStoreCannotBeEmpty,
}
