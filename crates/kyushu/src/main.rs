use std::time::SystemTime;

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
use whirlybird::journey::{JournalEventWrapper, JourneyEvent};

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
                let created_time = SystemTime::now();
                repo.save(RedMaple::new(
                    new_id.clone(),
                    created_time,
                    vec![JournalEventWrapper::new(
                        ID::from(uuid::Uuid::new_v4()),
                        created_time,
                        JourneyEvent::MapleCreated(mpl),
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
                        .map(JournalEventWrapper::time)
                        .unwrap_or(&SystemTime::UNIX_EPOCH);

                    let bt = b
                        .events()
                        .first()
                        .map(JournalEventWrapper::time)
                        .unwrap_or(&SystemTime::UNIX_EPOCH);

                    at.cmp(bt)
                });

                redmaples
                    .iter()
                    .map(|rm| {
                        EntryPrinter::new(true, true, "%y-%m-%d %H:%M".to_string()).projector(rm)
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
