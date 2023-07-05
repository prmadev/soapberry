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

use std::path::PathBuf;

use kyushu::{
    self,
    cli::{Args, MaplePrinter},
    config::{Config, InputInfo},
    domain::requests::{Change, Request},
    persistence::{self, FileDB, FrostElfError},
};
use redmaple::{
    event_group::EventKind,
    id::{Unique, ValidID, ID},
    FrostElf, RedMaple,
};
use thiserror::Error;
use time::format_description;
use whirlybird::journey::{self, Body, Event, EventWrapper, Links, ValidMapleID};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // checking for config file
    let mut config_file_output: Option<Config> = Option::default();

    // finding the platform specific directory
    let config_dir = match std::env::consts::OS {
        "linux" | "openbsd" | "netbsd" | "freebsd" | "dragonfly" => {
            std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from)
        }
        "macos" => std::env::var_os("HOME")
            .map(|o| PathBuf::from(o).join("Library").join("Application Support")),
        "solaris" => std::env::var_os("HOME").map(|o| PathBuf::from(o).join(".config")),
        "windows" => std::env::var_os("USERPROFILE")
            .map(|o| PathBuf::from(o).join("AppData").join("Roaming")),
        _ => None,
    };

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
    let frost_elf = persistence::FileDB::try_from(
        configurations
            .file_store
            .ok_or(MainError::FileStoreCannotBeEmpty)?,
    )?;

    // matching requests to the appropiate functions
    match req {
        Request::Change(chng) => match chng {
            Change::CreateNewMaple(mpl) => create_maple(&frost_elf, mpl)?,
            Change::UpdateMapleBody(maple_id, new_body) => {
                update_maple(&frost_elf, &maple_id, new_body)?;
            }
            Change::AddLinkToMaple { from, to, why } => add_link(&frost_elf, &from, &to, why)?,
        },

        Request::Information(i) => match i {
            kyushu::domain::requests::Information::ListEntries => list_entries(&frost_elf)?,
        },
    };
    Ok(())
}

fn add_link(
    frost_elf: &persistence::FileDB,
    from: &ID,
    to: &ID,
    why: String,
) -> Result<(), color_eyre::Report> {
    let des = ValidMapleID::try_from(frost_elf.redmaple_similar_id(to)?)?;
    let time_now = time::OffsetDateTime::now_utc();
    let ev = EventWrapper::new(time_now.into(), time_now, Event::LinkAdded((des, why)));
    frost_elf.save(
        frost_elf
            .redmaple_similar_id(from)?
            .clone()
            .into_appended(ev),
    )?;
    Ok(())
}

/// Retrieves a collection of events from the given `persistence::FileDB` repository,
/// sorts them based on their timestamps, and prints formatted representations of each event
/// using a `MaplePrinter`.
///
/// # Arguments
///
/// * `frost_elf` - A reference to a `persistence::FileDB` object representing the repository
///                 from which the events are retrieved.
///
/// # Returns
///
/// This function returns a `Result<(), color_eyre::Report>` indicating success or failure.
/// - If successful, `Ok(())` is returned.
/// - If an error occurs during the retrieval, sorting, or printing process, an `Err` variant
///   containing a `color_eyre::Report` is returned.
///
fn list_entries(frost_elf: &FileDB) -> Result<(), color_eyre::Report> {
    let mut redmaples = frost_elf.all_events()?.values().collect::<Vec<_>>();

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
                Body::from(rm),
                *rm.time_created()
                    .ok_or("Could not find the time created".to_owned())?,
                &format_description::parse("[year]-[month]-[day]:[hour]-[minute]-[second]")
                    .unwrap_or_default(),
                Links::from(rm).0,
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
    frost_elf: &persistence::FileDB,
    mpl: journey::Maple,
) -> Result<(), color_eyre::Report> {
    let created_time = time::OffsetDateTime::now_utc();
    frost_elf.save(RedMaple::new(vec![EventWrapper::new(
        mpl.id().inner().clone(),
        created_time,
        Event::MapleCreated(mpl),
    )]))?;
    Ok(())
}

fn update_maple(
    frost_elf: &persistence::FileDB,
    maple_id: &ID,
    new_body: Body,
) -> Result<(), color_eyre::Report> {
    let rdmpl = match frost_elf.redmaple_matching_id(maple_id) {
        Ok(o) => o.clone(),
        Err(e) => match e {
            FrostElfError::FailedToFindTheEventWithThatID => {
                match frost_elf.redmaple_similar_id(maple_id) {
                    Ok(o) => o.clone(),
                    Err(er) => return Err(er)?,
                }
            }
            FrostElfError::FailedToSerialize(e) => return Err(e)?,
            FrostElfError::FailedToCreateNewFile(e) | FrostElfError::FailedToWriteIntoFile(e) => {
                return Err(e)?
            }
            FrostElfError::FailedToGetID(e) => return Err(e)?,
            FrostElfError::FailedToFindASingleMatchingItem(e) => {
                return Err(color_eyre::Report::msg(format!("{e:#?}")))?
            }
            FrostElfError::EventBuilderFailed(e) => return Err(e)?,
        },
    };
    let time_now = time::OffsetDateTime::now_utc();
    let event = EventWrapper::new(
        ID::from(time_now),
        time_now,
        Event::MapleBodyUpdated(ValidMapleID::try_from(&rdmpl)?, new_body),
    );

    let rdmpl = rdmpl.into_appended(event);
    frost_elf.save(rdmpl)?;
    Ok(())
}

#[derive(Debug, Error)]
enum MainError {
    #[error("file store must be given")]
    FileStoreCannotBeEmpty,
}
