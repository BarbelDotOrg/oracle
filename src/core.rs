use clap::ValueEnum;

use crate::cli::{Cli, Commands};
use crate::print_completions;
use crate::shell::Shell;
use crate::state::State;

pub fn dispatch_command(state: &mut State, cmd: Commands, shell: Shell) -> anyhow::Result<()> {
    match cmd {
        Commands::Set { name, value, force } => todo!(),
        Commands::Get { name } => todo!(),
        Commands::List { all_shells } => {
            // todo perhaps try not to use a vec
            let shells: Vec<Shell> = if all_shells {
                Shell::value_variants().to_vec()
            } else {
                vec![shell]
            };

            for shell in shells {
                println!("Shell: {shell:?}");
                println!("Unmanaged:");
                println!("todo...");
                println!("Managed:");
                for (k, v) in state.vars.get(&shell).unwrap() {
                    println!("{k}={v}")
                }
            }
        }
        Commands::Remove { name } => todo!(),
        Commands::AddPathLike {
            name,
            value,
            delimitator,
        } => todo!(),
        Commands::RemovePathLike {
            name,
            value,
            delimitator,
        } => todo!(),
        Commands::Completions { shell } => todo!(),
    }
    Ok(())
}

pub fn run_cli(cli: Cli) {
    let mut state = State::load().unwrap_or_default();
    let shell = cli
        .forced_shell
        .unwrap_or(Shell::current().expect("Couldn't detect current shell, try to force a shell."));

    if let Some(err) = dispatch_command(&mut state, cli.command.unwrap(), shell).err() {
        println!("Error: {err:?}");
    }

    if let Some(err) = state.save().err() {
        println!("Error saving state: {err:?}");
    }
}
