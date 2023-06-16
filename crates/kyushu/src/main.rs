use kyushu::{self, config::Config, domain::requests, persistence};
use redmaple::EventRepo;

fn main() -> color_eyre::Result<()> {
    // setting up loggers
    color_eyre::install()?;

    // getting arguments
    let cli_arguments = kyushu::cli::Args::try_from(std::env::args_os())?;

    // getting configurations from cli_arguments
    let configurations = Config::from(cli_arguments.clone());

    // forming a request
    let req: requests::Request = cli_arguments.try_into()?;

    // creating persistence
    let per = persistence::FileRepo::try_from(
        configurations
            .file_store
            .ok_or(MainError::FileStoreCannotBeEmpty)?,
    )?;

    // matching requests to the appropiate functions
    match req {
        requests::Request::Change(chng) => match chng {
            requests::Change::CreateNewEntry(entr) => {
                let m = whirlybird::journey::JournalEvent::new(
                    redmaple::id::ID::new(uuid::Uuid::new_v4()),
                    std::time::SystemTime::now(),
                    whirlybird::journey::Journal::EntryCreated(entr),
                );

                // saving in the permaenent storage
                per.append(m)?;
            }
        },

        requests::Request::Information(_) => todo!(),
    };
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum MainError {
    #[error("file store must be given")]
    FileStoreCannotBeEmpty,
}
