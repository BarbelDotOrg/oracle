pub mod bashlike;
pub mod cli;
pub mod core;
pub mod fish;
pub mod shell;
pub mod state;

use crate::core::run_cli;
use clap::{CommandFactory, Parser};
use clap_complete::{Generator, generate};
use cli::{Cli, Commands};
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
            todo!("launch gui")
        }
        Some(_) => run_cli(cli),
    }
}
