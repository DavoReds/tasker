use clap::Parser;
use tasker_cli::{cli::Cli, execution::execute_application};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    execute_application(cli)?;

    Ok(())
}
