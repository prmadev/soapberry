//! Kyushu is a local-first, plain-text,
//! event-driven journaling tool for the modern age
//
//             ▄▓████▄  ▄██▄
//         ▓▓ ████████████████▄
//        ▓███████▓     █▓████████▄
//       ▓█████████▄  ▜███████ ██████
//         ▓████████▄    ▓██████▓ ▓███▄
//            ██▀       ███ ██████████
//             ▀▀█       █   ▄▄▄▄    ██
//       ▓▓██▓    █      █  ▓████▓ ██ ██▓
//      ▓███████   █    █   ▓███████████▀
//    ▓████████▄▄▄ █▄█████▄▄▄████████▀
//     ▓████████ ▀▀▀▀███▀▀████▀
//      ▓████         ▀███
//                   ▀███▀ ▄▀
//                    ▓▓██▀
//                    ▓▓██
//                    ▓███
//                    ▓███
//                    ▓███
//                   ▓████▄
//                 ▄▄██████▄
//  ▄▄▄▄▄▄▄▄▄▄▄ ▄▄████████████▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄

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
#![cfg_attr(docsrs, feature(doc_cfg))]

use std::{ffi::OsString, fmt::Debug, path::PathBuf};

use clap::Parser;
use kyushu::{
    self,
    cli::{Args, MaplePrinter},
    config::{Config, InputInfo},
    domain::requests::{Change, Request},
    persistence::{self, AminTheSinger, FrostElfError, ParastooTheKeeper},
};
use redmaple::{
    event_group::EventKind,
    id::{Unique, ValidID, ID},
    BeeElf, CartographerElf, GardnerElf, RedMaple, SeekingElf,
};
use thiserror::Error;
use time::{format_description, OffsetDateTime};
use whirlybird::journey::{self, Body, Event, EventWrapper, Link, Links, ValidMapleID};

fn main() -> color_eyre::Result<()> {
    // color_eyre::install()?;

    // checking for config file
    let mut config_file_output: Option<Config> = Option::default();

    // finding the platform specific directory
    let config_dir = config_dir_from_os(
        std::env::consts::OS,
        std::env::var_os("XDG_CONFIG_HOME"),
        std::env::var_os("HOME"),
        std::env::var_os("USERPROFILE"),
    );

    if let Some(config) = config_dir {
        let config_file_path = config.join("kyushu").join("config.json");
        if config_file_path.exists() {
            config_file_output = Some(Config::try_from(config_file_path)?);
        };
    };

    // getting arguments
    let cli_arguments = Args::try_parse_from(std::env::args_os())?;

    // getting configurations from cli_arguments
    let configurations = Config::from(InputInfo::new(
        Some(cli_arguments.clone()),
        config_file_output,
    ));

    // creating persistence
    let selda = persistence::SeldaTheListener::try_from(
        configurations
            .file_store
            .ok_or(MainError::FileStoreCannotBeEmpty)?,
    )?;

    let time_now = time::OffsetDateTime::now_utc();

    // matching requests to the appropriate functions
    match cli_arguments.to_request(time_now)? {
        Request::Change((time_of_change, the_change)) => match the_change {
            Change::CreateNewMaple(the_new_maple) => {
                plant_maple(&AminTheSinger::from(selda), the_new_maple, time_of_change)?;
            }

            Change::UpdateMapleBody(maple_id, the_new_body) => {
                let parastoo = ParastooTheKeeper::try_from(selda)?;
                water_maple(
                    &parastoo,
                    &parastoo,
                    &maple_id,
                    the_new_body,
                    time_of_change,
                    ID::from(time_of_change),
                )?;
            }
            Change::AddLinkToMaple { from, to, why } => {
                let parastoo = ParastooTheKeeper::try_from(selda)?;
                linkup(
                    &parastoo,
                    &parastoo,
                    &from,
                    time_of_change,
                    &to,
                    why,
                    ID::from(time_of_change),
                )?;
            }
            Change::Dislink { link_id } => {
                let parastoo = ParastooTheKeeper::try_from(selda)?;
                dislink(
                    &parastoo,
                    &parastoo,
                    &link_id,
                    ID::from(time_of_change),
                    time_of_change,
                )?;
            }
        },

        Request::Information(i) => match i {
            kyushu::domain::requests::Information::ListEntries => {
                show_forest(&ParastooTheKeeper::try_from(selda)?)?;
            }
        },
    };
    Ok(())
}

fn dislink(
    gardner_elf: &impl GardnerElf<Item = EventWrapper, EventError = FrostElfError>,
    cartographer_elf: &impl CartographerElf<Item = EventWrapper, EventError = FrostElfError>,
    link_id: &ID,
    new_event_id: ID,
    time_of_the_new_event: time::OffsetDateTime,
) -> Result<(), color_eyre::Report> {
    // creating links of all maples
    let binding = cartographer_elf.all_redmaples_as_map()?;
    let suspect_maples = binding.values().map(|x| (x, Links::from(*x).0));

    // try to find the exact match on the item
    let the_exact = suspect_maples.clone().find_map(|(x, l)| {
        l.into_iter()
            .find(|li| li.id().inner() == link_id)
            .map(|li| Some((li, *x)))?
    });

    let (link_in_question, harboring_redmaple) = if let Some(l) = the_exact {
        Ok(l)
    } else {
        let matches: Vec<(Link, &RedMaple<EventWrapper>)> = suspect_maples
            .flat_map(|(the_redmaple_in_question, suspect_links)| {
                {
                    suspect_links
                        .into_iter()
                        .filter(|one_link| {
                            one_link
                                .id()
                                .inner()
                                .inner()
                                .to_string()
                                .contains(&link_id.inner().to_string())
                        })
                        .map(|li| (li, *the_redmaple_in_question))
                        .collect::<Vec<(Link, &RedMaple<EventWrapper>)>>()
                }
            })
            .collect();

        if matches.len() > 1 {
            // too many matches
            Err::<(Link, &RedMaple<EventWrapper>), MainError>(MainError::TooManyLinksMatched)
        } else {
            matches
                .first()
                // not enough matches
                .ok_or(MainError::LinkCouldNotBeFound)
                .cloned()
        }
    }?;

    let event_at_the_bay = EventWrapper::new(
        new_event_id,
        time_of_the_new_event,
        Event::Dislinked(link_in_question.into_id()),
    );

    gardner_elf.tend(harboring_redmaple.clone().into_appended(event_at_the_bay))?;
    Ok(())
}

fn linkup(
    gardner_elf: &impl GardnerElf<Item = EventWrapper, EventError = FrostElfError>,
    seeking_elf: &impl SeekingElf<Item = EventWrapper, EventError = FrostElfError>,
    from: &ID,
    time_of_the_new_event: OffsetDateTime,
    to: &ID,
    why: String,
    new_event_id: ID,
) -> Result<(), color_eyre::Report> {
    let dest = match seeking_elf.redmaple_matching_id(to) {
        Ok(o) => Ok(ValidMapleID::try_from(o)),
        Err(err) => match err {
            FrostElfError::FailedToFindTheEventWithThatID => {
                Ok(ValidMapleID::try_from(seeking_elf.redmaple_similar_id(to)?))
            }
            FrostElfError::FailedToSerialize(err) => Err(FrostElfError::FailedToSerialize(err)),
            FrostElfError::FailedToCreateNewFile(err) => {
                Err(FrostElfError::FailedToCreateNewFile(err))
            }
            FrostElfError::FailedToWriteIntoFile(err) => {
                Err(FrostElfError::FailedToWriteIntoFile(err))
            }
            FrostElfError::FailedToGetID(err) => Err(FrostElfError::FailedToGetID(err)),
            FrostElfError::FailedToFindASingleMatchingItem(err) => {
                Err(FrostElfError::FailedToFindASingleMatchingItem(err))
            }
            FrostElfError::FileExists(err) => Err(FrostElfError::FileExists(err)),
            FrostElfError::FileReadFailed(err) => Err(FrostElfError::FileReadFailed(err)),
            FrostElfError::GivenPathDoesNotExist(err) => {
                Err(FrostElfError::GivenPathDoesNotExist(err))
            }
            FrostElfError::CouldNotReadTheDirectory(err) => {
                Err(FrostElfError::CouldNotReadTheDirectory(err))
            }
            FrostElfError::FileDoesNotExists(err) => Err(FrostElfError::FileDoesNotExists(err)),
        },
    }??;

    let origin_maple = match seeking_elf.redmaple_matching_id(from) {
        Ok(o) => Ok(o),
        Err(e) => match e {
            FrostElfError::FailedToFindTheEventWithThatID => {
                Ok(seeking_elf.redmaple_similar_id(from)?)
            }
            FrostElfError::FailedToSerialize(err) => Err(FrostElfError::FailedToSerialize(err)),
            FrostElfError::FailedToCreateNewFile(err) => {
                Err(FrostElfError::FailedToCreateNewFile(err))
            }
            FrostElfError::FailedToWriteIntoFile(err) => {
                Err(FrostElfError::FailedToWriteIntoFile(err))
            }
            FrostElfError::FailedToGetID(err) => Err(FrostElfError::FailedToGetID(err)),
            FrostElfError::FailedToFindASingleMatchingItem(err) => {
                Err(FrostElfError::FailedToFindASingleMatchingItem(err))
            }
            FrostElfError::FileExists(err) => Err(FrostElfError::FileExists(err)),
            FrostElfError::FileReadFailed(err) => Err(FrostElfError::FileReadFailed(err)),
            FrostElfError::GivenPathDoesNotExist(err) => {
                Err(FrostElfError::GivenPathDoesNotExist(err))
            }
            FrostElfError::CouldNotReadTheDirectory(err) => {
                Err(FrostElfError::CouldNotReadTheDirectory(err))
            }
            FrostElfError::FileDoesNotExists(err) => Err(FrostElfError::FileDoesNotExists(err)),
        },
    }?;

    let ev = EventWrapper::new(
        time_of_the_new_event.into(),
        time_of_the_new_event,
        Event::LinkAdded((dest, why, new_event_id)),
    );

    gardner_elf.tend(origin_maple.clone().into_appended(ev))?;

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
fn show_forest(
    cartographer_elf: &impl CartographerElf<Item = EventWrapper, EventError = FrostElfError>,
) -> Result<(), color_eyre::Report> {
    let binding = cartographer_elf.all_redmaples_as_map()?;
    let redmaples = &mut binding.values().collect::<Vec<_>>();

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
        .iter_mut()
        .map(|rm| -> Result<MaplePrinter, String> {
            MaplePrinter::new_with_local_offset(
                ValidMapleID::try_from(**rm)
                    .map_err(|er| format!("could not create id for map{er}"))?,
                Body::from(**rm),
                *rm.time_created()
                    .ok_or("Could not find the time created".to_owned())?,
                &format_description::parse("[year]-[month]-[day]:[hour]-[minute]-[second]")
                    .unwrap_or_default(),
                Links::from(**rm).0,
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

fn plant_maple(
    bee_elf: &impl BeeElf<Item = EventWrapper, EventError = FrostElfError>,
    the_new_maple: journey::Maple,
    time_of_the_new_event: OffsetDateTime,
) -> Result<(), color_eyre::Report> {
    bee_elf.plant(RedMaple::new(vec![EventWrapper::new(
        the_new_maple.id().inner().clone(),
        time_of_the_new_event,
        Event::MapleCreated(the_new_maple),
    )]))?;
    Ok(())
}

fn water_maple(
    gardner_elf: &impl GardnerElf<Item = EventWrapper, EventError = FrostElfError>,
    seeking_elf: &impl SeekingElf<Item = EventWrapper, EventError = FrostElfError>,
    maple_id: &ID,
    new_body: Body,
    time_now: OffsetDateTime,
    new_event_id: ID,
) -> Result<(), color_eyre::Report> {
    let young_redmaple = match seeking_elf.redmaple_matching_id(maple_id) {
        Ok(o) => o.clone(),
        Err(e) => match e {
            FrostElfError::FailedToFindTheEventWithThatID => {
                match seeking_elf.redmaple_similar_id(maple_id) {
                    Ok(the_maple) => the_maple.clone(),
                    Err(err) => return Err(err)?,
                }
            }
            FrostElfError::FailedToSerialize(err) => return Err(err)?,
            FrostElfError::FailedToCreateNewFile(err)
            | FrostElfError::FailedToWriteIntoFile(err) => return Err(err)?,
            FrostElfError::FailedToGetID(err) => return Err(err)?,
            FrostElfError::FailedToFindASingleMatchingItem(err) => return Err(to_ce_error(err))?,
            FrostElfError::FileExists(err)
            | FrostElfError::FileDoesNotExists(err)
            | FrostElfError::GivenPathDoesNotExist(err) => Err(to_ce_error(err))?,
            FrostElfError::FileReadFailed(err) | FrostElfError::CouldNotReadTheDirectory(err) => {
                return Err(err)?
            }
        },
    };

    let new_event = EventWrapper::new(
        new_event_id,
        time_now,
        Event::MapleBodyUpdated(ValidMapleID::try_from(&young_redmaple)?, new_body),
    );

    let more_sophisticated_redmaple = young_redmaple.into_appended(new_event);
    gardner_elf.tend(more_sophisticated_redmaple)?;
    Ok(())
}

fn to_ce_error(e: impl Debug) -> color_eyre::Report {
    color_eyre::Report::msg(format!("{e:#?}"))
}

#[derive(Debug, Error)]
enum MainError {
    #[error("file store must be given")]
    FileStoreCannotBeEmpty,

    #[error("Could not find the link")]
    LinkCouldNotBeFound,

    #[error("Matched too many links")]
    TooManyLinksMatched,
}

fn config_dir_from_os(
    os_name: &str,
    xdg_config_home: Option<OsString>,
    home: Option<OsString>,
    userprofile: Option<OsString>,
) -> Option<PathBuf> {
    match os_name {
        "linux" | "openbsd" | "netbsd" | "freebsd" | "dragonfly" => {
            xdg_config_home.map(PathBuf::from)
        }
        "macos" => home.map(|o| PathBuf::from(o).join("Library").join("Application Support")),
        "solaris" => home.map(|o| PathBuf::from(o).join(".config")),
        "windows" => userprofile.map(|o| PathBuf::from(o).join("AppData").join("Roaming")),
        _ => None,
    }
}
