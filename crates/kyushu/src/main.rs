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
    config::{Config, InputInfo},
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
use whirlybird::journey::{Body, Event, EventWrapper};

#[allow(clippy::cast_sign_loss)] // timestamp is given in i64, but it can only be positive
fn main() -> color_eyre::Result<()> {
    // setting up loggers
    color_eyre::install()?;

    // checking for config file
    let mut config_file_output: Option<Config> = Option::default();

    let config_dir = dirs::config_dir();

    if let Some(config) = config_dir {
        let config_file_path = config.join("kyushu").join("config.json");
        if config_file_path.exists() {
            config_file_output = Some(Config::try_from(config_file_path)?);
        };
    };

    // getting arguments
    let cli_arguments = Args::try_from(std::env::args_os())?;

    // getting configurations from cli_arguments
    let configurations = Config::from(InputInfo::new(
        Some(cli_arguments.clone()),
        config_file_output,
    ));

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
            Change::CreateNewMaple(mpl) => create_maple(&repo, mpl)?,
            Change::UpdateMapleBody(maple_id, new_body) => update_maple(&repo, maple_id, new_body)?,
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

fn create_maple(
    repo: &persistence::FileDB,
    mpl: whirlybird::journey::Maple,
) -> Result<(), color_eyre::Report> {
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
    Ok(())
}

fn update_maple(
    repo: &persistence::FileDB,
    maple_id: ID,
    new_body: Body,
) -> Result<(), color_eyre::Report> {
    let rdmpl = repo.redmaple_matching_id(&maple_id)?.clone();
    let time_now = time::OffsetDateTime::now_utc();
    let event = EventWrapper::new(
        ID::from(time_now),
        time_now,
        Event::MapleBodyUpdated(rdmpl.id().to_owned(), new_body),
    );

    let rdmpl = rdmpl.into_appended(event);
    repo.save(rdmpl)?;
    Ok(())
}

#[derive(Debug, Error)]
enum MainError {
    #[error("file store must be given")]
    FileStoreCannotBeEmpty,
}
