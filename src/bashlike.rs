use crate::shell::Shell;
use crate::state::{ManagedVariable, State};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

fn get_oracle_file_path(shell: Shell) -> PathBuf {
    dirs::config_dir()
        .unwrap()
        .join("oracle")
        .join(match shell {
            Shell::Bash => "oracle.sh",
            Shell::Zsh => "oracle.zsh",
            Shell::Fish => "oracle.fish",
            _ => unreachable!("mumma!"),
        })
}

fn get_shell_profile_path(shell: Shell) -> PathBuf {
    match shell {
        Shell::Bash => dirs::home_dir().unwrap().join(".bashrc"),
        Shell::Zsh => dirs::home_dir().unwrap().join(".zshrc"),
        Shell::Fish => dirs::config_dir().unwrap().join("fish").join("config.fish"),
        _ => unreachable!("it's too christpilled..."),
    }
}

pub fn ensure_oracle_file_is_sourced(shell: Shell) -> std::io::Result<()> {
    let oracle_path = get_oracle_file_path(shell);
    let profile_path = get_shell_profile_path(shell);

    let source_line = format!("source \"{}\"", oracle_path.to_str().unwrap());

    // ensure oracle_path exists and is a file
    if let Some(parent) = oracle_path.parent() {
        fs::create_dir_all(parent)?;
    }
    if !oracle_path.exists() {
        fs::write(&oracle_path, "")?;
    }

    let contents = fs::read_to_string(&profile_path).unwrap_or_default();

    if contents.lines().any(|line| line.trim() == source_line) {
        return Ok(());
    }

    let mut profile_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&profile_path)?;

    writeln!(profile_file, "\n# added by oracle\n{source_line}")?;

    Ok(())
}

impl State {
    pub fn save_to_oracle_sh(&self, shell: Shell) -> std::io::Result<()> {
        let mut contents = String::new();
        contents.push_str(
            "# Managed by Oracle, 5000 years of generationnal dharma loss if edited manually!\n\
        # Well nobody cares as Oracle will regenerate it... mumma....\n\n",
        );

        let fmt_export = |k: &str, v: &str| match shell {
            Shell::Fish => format!("set -gx {k} \"{v}\"\n"),
            _ => format!("export {k}=\"{v}\"\n"),
        };

        contents.push_str("# -- Simple Variables --\n");
        for (k, v) in &self.vars {
            if let ManagedVariable::Simple(val) = v {
                contents.push_str(&fmt_export(k, val));
            }
        }
        contents.push('\n');

        contents.push_str("# -- Overrides --\n");
        for (k, v) in &self.overrides {
            contents.push_str(&fmt_export(k, v));
        }
        contents.push('\n');

        contents.push_str("# -- Pathlike Variables --\n");
        for (k, v) in &self.vars {
            if let ManagedVariable::Pathlike(vec) = v {
                contents.push_str(&fmt_export(k, &vec.join(":")));
            }
        }
        contents.push('\n');

        contents.push_str("# -- Pathlike Additions --\n");
        for (k, v) in &self.pathlike_adds {
            let joined = format!("{k}:{}", v.join(":"));
            // fish has no bare $VAR interpolation inside "..." the way bash/zsh do,
            // it needs $VAR without quotes to expand, so build this one differently:
            match shell {
                Shell::Fish => contents.push_str(&format!("set -gx {k} \"${k}:{}\"\n", v.join(":"))),
                _ => contents.push_str(&format!("export {k}={k}:{}\n", v.join(":"))),
            }
            let _ = joined; // just here to show the shape; drop if unused
        }
        contents.push('\n');

        fs::write(get_oracle_file_path(shell), contents)?;
        Ok(())
    }
}