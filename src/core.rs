use clap::ValueEnum;

use crate::cli::{Cli, Commands};
use crate::pathlike::{DEFAULT_DELIM, seems_pathlike};
use crate::print_completions;
use crate::shell::Shell;
use crate::state::{ManagedVariable, ManagedVariableSource, SetResult, State};

pub fn dispatch_command(
    state: &mut State,
    cmd: Commands,
    shell: Shell,
    delimitator: char,
) -> anyhow::Result<()> {
    match cmd {
        Commands::Set { name, value, force } => match state.set(&name, &value, force) {
            SetResult::Success => println!("Variable set successfully."),
            SetResult::EnvVarExists => {
                println!("Variable already exists. Use --force to overwrite.")
            }
            SetResult::Override => println!("Variable overridden."),
        },
        Commands::Get { name } => match state.vars.get(&name) {
            Some(ManagedVariable::Simple(val)) => println!("{}", val),
            Some(ManagedVariable::Pathlike(vec)) => {
                println!("{}", vec.join(&delimitator.to_string()))
            }
            None => eprintln!("Variable not found."),
        },
        Commands::List => {
            println!("Simple:");
            for (k, v) in &state.vars {
                if let ManagedVariable::Simple(val) = v {
                    println!("\t- {}={}", k, val);
                }
            }
            println!("Pathlike:");
            for (k, v) in &state.vars {
                if let ManagedVariable::Pathlike(vec) = v {
                    println!("\t- {}={}", k, vec.join(&delimitator.to_string()));
                }
            }

            println!("Overrides:");
            for (k, v) in &state.overrides {
                println!("\t- {}={}", k, v);
            }

            println!("Pathlike Additions:");
            for (k, v) in &state.pathlike_adds {
                println!("\t- {}={}", k, v.join(&delimitator.to_string()));
            }
        }
        Commands::Remove { name } => {
            match state.remove(&name) {
                Some(ManagedVariableSource::Vars(_)) => println!("Variable removed."),
                Some(ManagedVariableSource::PathlikeAdditions(_)) => println!("Pathlike addition removed."),
                Some(ManagedVariableSource::Overrides(_)) => println!("Override removed."),
                None => eprintln!("Variable not found."),
            }
        }
        Commands::AddPathLike { name, value, delimitator } => {
            state.add_pathlike(&name, &value)?;
            println!("Value added to path-like variable.");
        }
        Commands::RemovePathLike { name, value, delimitator } => {
            state.remove_pathlike(&name, &value)?;
            println!("Value removed from path-like variable.");
        }
        Commands::Completions { .. } => unreachable!("preposterous!"),
    }
    Ok(())
}

pub fn run_cli(cli: Cli) {
    let mut state = State::load().unwrap_or_default();
    let shell = cli
        .shell
        .unwrap_or(Shell::current().expect("Couldn't detect current shell, try to force a shell."));

    if let Some(err) = dispatch_command(
        &mut state,
        cli.command.unwrap(),
        shell,
        cli.delimitator.unwrap_or(DEFAULT_DELIM),
    )
    .err()
    {
        println!("Error: {err:?}");
    }

    if let Some(err) = state.save().err() {
        println!("Error saving state: {err:?}");
    }
    state.save().expect("Failed to save state");
}
