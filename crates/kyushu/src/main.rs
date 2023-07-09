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
use time::{format_description, OffsetDateTime};
use whirlybird::journey::{self, Body, Event, EventWrapper, Link, Links, ValidMapleID};

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

    // creating persistence
    let frost_elf = persistence::FileDB::try_from(
        configurations
            .file_store
            .ok_or(MainError::FileStoreCannotBeEmpty)?,
    )?;

    let time_now = time::OffsetDateTime::now_utc();

    // matching requests to the appropiate functions
    match cli_arguments.to_request()? {
        Request::Change(the_change) => match the_change {
            Change::CreateNewMaple(the_new_maple) => {
                create_maple(&frost_elf, the_new_maple, time_now)?;
            }
            Change::UpdateMapleBody(maple_id, the_new_body) => {
                update_maple(
                    &frost_elf,
                    &maple_id,
                    the_new_body,
                    time_now,
                    ID::from(time_now),
                )?;
            }
            Change::AddLinkToMaple { from, to, why } => {
                add_link(&frost_elf, &from, time_now, &to, why, ID::from(time_now))?;
            }
            Change::Dislink { link_id } => {
                dislink(&frost_elf, &link_id, ID::from(time_now), time_now)?;
            }
        },

        Request::Information(i) => match i {
            kyushu::domain::requests::Information::ListEntries => list_entries(&frost_elf)?,
        },
    };
    Ok(())
}

fn dislink(
    frost_elf: &persistence::FileDB,
    link_id: &ID,
    new_event_id: ID,
    time_of_the_new_event: time::OffsetDateTime,
) -> Result<(), color_eyre::Report> {
    // creating links of all maples
    let suspect_maples = frost_elf
        .all_redmaples_as_map()?
        .values()
        .map(|x| (x, Links::from(x).0));

    // try to find the exact match on the item
    let the_exact = suspect_maples.clone().find_map(|(x, l)| {
        l.into_iter()
            .find(|li| li.id().inner() == link_id)
            .map(|li| Some((li, x)))?
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
                        .map(|li| (li, the_redmaple_in_question))
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

    frost_elf.save(harboring_redmaple.clone().into_appended(event_at_the_bay))?;
    Ok(())
}

fn add_link(
    frost_elf: &persistence::FileDB,
    from: &ID,
    time_of_the_new_event: OffsetDateTime,
    to: &ID,
    why: String,
    new_event_id: ID,
) -> Result<(), color_eyre::Report> {
    let des = ValidMapleID::try_from(frost_elf.redmaple_similar_id(to)?)?;
    let ev = EventWrapper::new(
        time_of_the_new_event.into(),
        time_of_the_new_event,
        Event::LinkAdded((des, why, new_event_id)),
    );
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
    let mut redmaples = frost_elf
        .all_redmaples_as_map()?
        .values()
        .collect::<Vec<_>>();

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
    time_of_the_new_event: OffsetDateTime,
) -> Result<(), color_eyre::Report> {
    frost_elf.save(RedMaple::new(vec![EventWrapper::new(
        mpl.id().inner().clone(),
        time_of_the_new_event,
        Event::MapleCreated(mpl),
    )]))?;
    Ok(())
}

fn update_maple(
    frost_elf: &persistence::FileDB,
    maple_id: &ID,
    new_body: Body,
    time_now: OffsetDateTime,
    new_event_id: ID,
) -> Result<(), color_eyre::Report> {
    let young_redmaple = match frost_elf.redmaple_matching_id(maple_id) {
        Ok(o) => o.clone(),
        Err(e) => match e {
            FrostElfError::FailedToFindTheEventWithThatID => {
                match frost_elf.redmaple_similar_id(maple_id) {
                    Ok(the_maple) => the_maple.clone(),
                    Err(err) => return Err(err)?,
                }
            }
            FrostElfError::FailedToSerialize(err) => return Err(err)?,
            FrostElfError::FailedToCreateNewFile(err)
            | FrostElfError::FailedToWriteIntoFile(err) => return Err(err)?,
            FrostElfError::FailedToGetID(err) => return Err(err)?,
            FrostElfError::FailedToFindASingleMatchingItem(err) => {
                return Err(color_eyre::Report::msg(format!("{err:#?}")))?
            }
            FrostElfError::EventBuilderFailed(err) => return Err(err)?,
        },
    };
    let new_event = EventWrapper::new(
        new_event_id,
        time_now,
        Event::MapleBodyUpdated(ValidMapleID::try_from(&young_redmaple)?, new_body),
    );

    let more_sophisticated_redmaple = young_redmaple.into_appended(new_event);
    frost_elf.save(more_sophisticated_redmaple)?;
    Ok(())
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
