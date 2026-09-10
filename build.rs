// build.rs
use clap::CommandFactory;
use clap_complete::generate_to;
use std::env;
use std::io::Error;

mod shell {
    include!("src/shell.rs");
}

mod cli {
    include!("src/cli.rs");
}

use cli::Cli;

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = Cli::command();
    for &shell in [
        clap_complete::Shell::Bash,
        clap_complete::Shell::Fish,
        clap_complete::Shell::PowerShell,
        clap_complete::Shell::Zsh,
    ]
    .iter()
    {
        generate_to(shell, &mut cmd, "oracle", &outdir)?;
    }

    Ok(())
}
