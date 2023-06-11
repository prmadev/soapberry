use kyushu::{self, config::Config, domain::requests, persistence};

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let cli_arguments = kyushu::cli::Args::try_from(std::env::args_os())?;
    let _configurations = Config::from(cli_arguments.clone());
    let req: requests::Request = cli_arguments.try_into()?;
    let per = persistence::FileRepo::try_from(Some(_configurations.file_store.unwrap()).unwrap());

    match req {
        requests::Request::Change(chng) => match chng {
            requests::Change::CreateNewEntry(entr) => {}
        },
        requests::Request::Information(_) => todo!(),
    }

    Ok(())
}
