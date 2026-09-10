use crate::cli::{Cli, Commands};
use crate::print_completions;
use crate::shell::Shell;
use crate::state::State;

pub fn run_cli(cli: Cli) {
    let mut state = State::load().unwrap_or_default();
    let mut shell =
        cli.forced_shell.unwrap_or(Shell::current().expect(
            "Couldn't detect current shell and not forcing a shell. Try to force a shell.",
        ));
    match cli.command.unwrap() {
        Commands::Set { name, value, force } => {}
        Commands::Get { name } => {}
        Commands::Remove { name } => {}
        Commands::AddPathLike {
            name,
            value,
            delimitator,
        } => {}
        Commands::RemovePathLike {
            name,
            value,
            delimitator,
        } => {}
        Commands::Completions { .. } => {
            unreachable!("completions should be catched in main")
        }
    }
}
