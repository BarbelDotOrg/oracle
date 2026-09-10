use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    PowerShell,
    Fish,
}

impl Shell {
    #[cfg(unix)]
    pub fn current() -> Option<Shell> {
        if let Some(s) = std::env::var_os("SHELL") {
            // most of the times when $SHELL is defines its well defined
            // so we dont run the other cases
            return Self::shell_from_path(s.to_str()?);
        }

        // todo perhaps other detection

        None
    }

    #[cfg(windows)]
    pub fn current() -> Option<Shell> {
        Some(Shell::PowerShell)
    }

    fn shell_from_path(shell: &str) -> Option<Shell> {
        if shell.ends_with("fish") {
            return Some(Shell::Fish);
        }
        if shell.ends_with("zsh") {
            return Some(Shell::Zsh);
        }
        if shell.ends_with("bash") {
            return Some(Shell::Bash);
        }
        None
    }
}
