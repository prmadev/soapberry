use kyushu::{self, config::Config};

fn main() {
    let cli_arguments = kyushu::cli::Args::from(std::env::args_os());
    let _configurations = Config::from(cli_arguments);
}
