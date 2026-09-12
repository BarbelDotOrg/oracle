pub mod bashlike;
pub mod cli;
pub mod core;
pub mod pathlike;
pub mod shell;
pub mod state;
pub mod ui;

use crate::core::run_cli;
use crate::ui::app::OracleApp;
use clap::{CommandFactory, Parser};
use clap_complete::{Generator, generate};
use cli::{Cli, Commands};
use cosmic::app::Settings;
use std::io;

fn print_completions<G: Generator>(generator: G, cmd: &mut clap::Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}

fn main() {
    let cli = Cli::parse();

    if let Some(Commands::Completions { shell }) = cli.command {
        let mut cmd = Cli::command();
        print_completions(shell, &mut cmd);
        return;
    }

    match cli.command {
        None => {
            // no command, its ui then
            cosmic::app::run::<OracleApp>(Settings::default(), ()).unwrap();
        }
        Some(_) => run_cli(cli),
    }
}
