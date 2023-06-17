use std::time::SystemTime;

use kyushu::{
    self,
    cli::Args,
    config::Config,
    domain::requests::{Change, Request},
    persistence,
};
use redmaple::{id::ID, EventRepo};
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

        Request::Information(_) => todo!(),
    };
    Ok(())
}

#[derive(Debug, Error)]
enum MainError {
    #[error("file store must be given")]
    FileStoreCannotBeEmpty,
}
