use clap::Parser;
use tasker_cli::{cli::Cli, execution::execute_application};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    execute_application(cli)?;

    Ok(())
}
