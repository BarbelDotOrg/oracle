use crate::shell::Shell;
use clap::{Parser, Subcommand};

/// Oracle - a tool for managing environment variables
#[derive(Parser)]
#[command(name = "oracle", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Force a specific shell instead of auto-detecting it
    #[arg(short, long)]
    pub forced_shell: Option<Shell>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Set an environment variable
    Set {
        /// Name of the variable to set
        name: String,
        /// Value to assign to the variable
        value: String,
        /// Overwrite the variable even if it already exists
        #[arg(short, long, default_value = "false")]
        force: bool,
    },

    /// Get the value of an environment variable
    Get {
        /// Name of the variable to retrieve
        name: String,
    },

    /// List all env variables managed by the tool
    List {
        /// List for all shells and not just for the current (or forced) one
        #[arg(short, long, default_value = "false")]
        all_shells: bool,
    },

    /// Remove an environment variable
    Remove {
        /// Name of the variable to remove
        name: String,
    },

    /// Append a value to a path-like variable (e.g. PATH)
    AddPathLike {
        /// Name of the path-like variable
        name: String,
        /// Value to append
        value: String,
        /// Character used to separate entries
        #[arg(short, long, default_value = ":")]
        delimitator: char,
    },

    /// Remove a value from a path-like variable (e.g. PATH)
    RemovePathLike {
        /// Name of the path-like variable
        name: String,
        /// Value to remove
        value: String,
        /// Character used to separate entries
        #[arg(short, long, default_value = ":")]
        delimitator: char,
    },

    /// Generate shell completion scripts
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}
