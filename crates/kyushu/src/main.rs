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
    cli::{Args, MaplePrinter},
    config::{Config, InputInfo},
    domain::requests::{Change, Request},
    persistence,
};
use redmaple::{
    event_group::EventGroup,
    id::{IDGiver, ID},
    EventRepo, RedMaple,
};
use thiserror::Error;
use time::format_description;
use whirlybird::journey::{Body, Event, EventWrapper, ValidMapleID};

fn main() -> color_eyre::Result<()> {
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
            Change::UpdateMapleBody(maple_id, new_body) => {
                update_maple(&repo, &maple_id, new_body)?;
            }
        },

        Request::Information(i) => match i {
            kyushu::domain::requests::Information::ListEntries => list_entries(&repo)?,
        },
    };
    Ok(())
}

fn list_entries(repo: &persistence::FileDB) -> Result<(), color_eyre::Report> {
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
        .into_iter()
        .map(|rm| -> Result<MaplePrinter, String> {
            MaplePrinter::new_with_local_offset(
                ValidMapleID::try_from(rm)
                    .map_err(|er| format!("could nto create id for map{er}"))?,
                Body::from(rm.clone()),
                *rm.time_created()
                    .ok_or("Could not find the time created".to_owned())?,
                &format_description::parse("[year]-[month]-[day]:[hour]-[minute]-[second]")
                    .unwrap_or_default(),
            )
            .map_err(|er| format!("could not create printer: {er}"))
        })
        .for_each(|each| {
            match each {
                Ok(o) => println!("{o}"),
                Err(e) => eprintln!("{e}"),
            };
        });
    Ok(())
}

fn create_maple(
    repo: &persistence::FileDB,
    mpl: whirlybird::journey::Maple,
) -> Result<(), color_eyre::Report> {
    let created_time = time::OffsetDateTime::now_utc();
    repo.save(RedMaple::new(vec![EventWrapper::new(
        mpl.id().inner().clone(),
        created_time,
        Event::MapleCreated(mpl),
    )]))?;
    Ok(())
}

fn update_maple(
    repo: &persistence::FileDB,
    maple_id: &ID,
    new_body: Body,
) -> Result<(), color_eyre::Report> {
    let rdmpl = match repo.redmaple_matching_id(maple_id) {
        Ok(o) => o.clone(),
        Err(e) => match e {
            persistence::EventRepoError::CouldNotFindTheEventWithThatID => {
                match repo.redmaple_similar_id(maple_id) {
                    Ok(o) => o.clone(),
                    Err(er) => return Err(er)?,
                }
            }
            persistence::EventRepoError::CouldNotSerialize(e) => return Err(e)?,
            persistence::EventRepoError::CouldNotCreateNewFile(e) => return Err(e)?,
            persistence::EventRepoError::CouldNotWriteIntoFile(e) => return Err(e)?,
            persistence::EventRepoError::IDGettingFailed(e) => return Err(e)?,
            persistence::EventRepoError::MultipleItemsFound(e) => {
                return Err(color_eyre::Report::msg(format!("{:#?}", e)))?
            }
        },
    };
    let time_now = time::OffsetDateTime::now_utc();
    let event = EventWrapper::new(
        ID::from(time_now),
        time_now,
        Event::MapleBodyUpdated(ValidMapleID::try_from(&rdmpl)?, new_body),
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
