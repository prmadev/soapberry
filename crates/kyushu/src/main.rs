use std::time::SystemTime;

use chrono::{DateTime, Local};
use kyushu::{
    self,
    cli::Args,
    config::Config,
    domain::requests::{Change, Request},
    persistence,
};
use redmaple::{
    event_group::EventGroup,
    id::{IDGiver, ID},
    EventRepo, RedMaple, RedMaplePrinter,
};
use thiserror::Error;
use uuid::Uuid;
use whirlybird::journey::{Journal, JournalEvent};

fn main() -> color_eyre::Result<()> {
    // setting up loggers
    color_eyre::install()?;

    // getting arguments
    let cli_arguments = Args::try_from(std::env::args_os())?;

    // getting configurations from cli_arguments
    let configurations = Config::from(cli_arguments.clone());

    // forming a request
    let req: Request = cli_arguments.try_into()?;

    // creating persistence
    let per = persistence::FileRepo::try_from(
        configurations
            .file_store
            .ok_or(MainError::FileStoreCannotBeEmpty)?,
    )?;

    // matching requests to the appropiate functions
    match req {
        Request::Change(chng) => match chng {
            Change::CreateNewEntry(entr) => {
                per.save(redmaple::RedMaple::new(
                    ID::from(Uuid::new_v4()),
                    vec![JournalEvent::new(
                        ID::from(uuid::Uuid::new_v4()),
                        SystemTime::now(),
                        Journal::EntryCreated(entr),
                    )],
                ))?;
                // saving in the permaenent storage
            }
        },

        Request::Information(i) => match i {
            kyushu::domain::requests::Information::ListEntries => {
                let mut a = per.all_events()?.values().collect::<Vec<_>>();
                a.sort_by(|a, b| {
                    let at = a
                        .events()
                        .first()
                        .map(|x| x.time())
                        .unwrap_or(&SystemTime::UNIX_EPOCH);
                    let bt = b
                        .events()
                        .first()
                        .map(|x| x.time())
                        .unwrap_or(&SystemTime::UNIX_EPOCH);
                    at.cmp(bt)
                });
                a.iter()
                    .map(|rm| {
                        Printable::new(true, false, "%Y-%m-%d %H:%M:%S".to_string()).printer(rm)
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

struct Printable {
    show_time: bool,
    show_id: bool,
    time_format: String,
}

impl Printable {
    fn new(show_time: bool, show_id: bool, time_format: String) -> Self {
        Self {
            show_time,
            show_id,
            time_format,
        }
    }
}

impl RedMaplePrinter for Printable {
    type EventType = JournalEvent;

    fn printer(&self, data: &RedMaple<Self::EventType>) -> String {
        let id = match self.show_id {
            true => data.id().inner().inner().to_string(),
            false => "".to_string(),
        };
        let date = match self.show_time {
            true => data
                .events()
                .first()
                .map(|x| {
                    let a: DateTime<Local> = x.time().to_owned().into();
                    a.format(&self.time_format).to_string()
                })
                .unwrap_or(String::from("____-__-__ __:__:__")),
            false => "".to_string(),
        };

        let body = data
            .events()
            .first()
            .map(|x| match x.data() {
                Journal::EntryCreated(e) => e.body().clone().map(|x| x.inner().to_owned()),
                _ => None,
            })
            .flatten()
            .unwrap_or_else(|| "".to_string());

        format!("{date}: {body} -- {id} ")
    }
}
