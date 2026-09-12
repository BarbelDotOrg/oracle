use crate::bashlike::ensure_oracle_file_is_sourced;
use crate::shell::Shell;
use anyhow::anyhow;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{File, create_dir_all};
use std::io::Write;

pub enum SetResult {
    Success,
    EnvVarExists,
    Override,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ManagedVariable {
    Simple(String),
    Pathlike(Vec<String>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ManagedVariableSource {
    Vars(ManagedVariable),
    PathlikeAdditions(Vec<String>),
    Overrides(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    #[serde(skip, default = "current_env_vars")]
    env_vars: HashMap<String, String>,
    pub vars: HashMap<String, ManagedVariable>,
    pub pathlike_adds: HashMap<String, Vec<String>>,
    pub overrides: HashMap<String, String>,
}

impl State {
    pub fn save(&self) -> anyhow::Result<()> {
        let cfg_dir = dirs::config_dir()
            .ok_or(anyhow!("Couldnt find config dir (what the sigma ???)"))?
            .join("oracle");

        create_dir_all(&cfg_dir)?;

        let cfg_path = cfg_dir.join("state.ron");
        let mut file = File::create(cfg_path)?;
        let serialized = ron::ser::to_string_pretty(self, PrettyConfig::default())?;
        file.write_all(serialized.as_bytes())?;

        for shell in [Shell::Zsh, Shell::Bash, Shell::Fish] {
            ensure_oracle_file_is_sourced(shell)?;
            self.save_to_oracle_sh(shell)?;
        }

        Ok(())
    }

    pub fn load() -> anyhow::Result<State> {
        let cfg_path = dirs::config_dir()
            .ok_or(anyhow!("todo err message"))?
            .join("oracle")
            .join("state.ron");
        let file = File::open(cfg_path)?;
        Ok(ron::de::from_reader(file)?)
    }

    pub fn set(&mut self, key: &str, value: &str, force: bool) -> SetResult {
        if self.env_vars.contains_key(key) {
            self.overrides.insert(key.to_string(), value.to_string());
            return SetResult::Override;
        }
        if !force && self.vars.contains_key(key) {
            SetResult::EnvVarExists
        } else {
            self.vars
                .insert(key.to_string(), ManagedVariable::Simple(value.to_string()));
            SetResult::Success
        }
    }

    pub fn add_pathlike(&mut self, key: &str, value: &str) -> anyhow::Result<()> {
        if self.env_vars.contains_key(key) {
            if let Some(vec) = self.pathlike_adds.get_mut(key) {
                if !vec.contains(&value.to_string()) {
                    vec.push(value.to_string());
                }
            } else {
                self.pathlike_adds
                    .insert(key.to_string(), vec![value.to_string()]);
            }
        } else {
            if let Some(existing_value) = self.vars.get_mut(key) {
                match existing_value {
                    ManagedVariable::Simple(s) => {
                        *existing_value =
                            ManagedVariable::Pathlike(vec![s.clone(), value.to_string()]);
                    }
                    ManagedVariable::Pathlike(vec) => {
                        if !vec.contains(&value.to_string()) {
                            vec.push(value.to_string());
                        }
                    }
                }
            } else {
                self.vars.insert(
                    key.to_string(),
                    ManagedVariable::Pathlike(vec![value.to_string()]),
                );
            }
        }

        Ok(())
    }

    pub fn remove_pathlike(&mut self, key: &str, value: &str) -> anyhow::Result<()> {
        match self.vars.get_mut(key) {
            Some(ManagedVariable::Pathlike(vec)) => {
                vec.retain(|v| v != value);
                if vec.is_empty() {
                    self.vars.remove(key);
                }
            }
            Some(ManagedVariable::Simple(val)) => {
                if val == value {
                    self.vars.remove(key);
                }
            }
            None => {}
        }
        Ok(())
    }

    pub fn remove(&mut self, key: &str) -> Option<ManagedVariableSource> {
        if let Some(var) = self.vars.remove(key) {
            return Some(ManagedVariableSource::Vars(var));
        }
        if let Some(adds) = self.pathlike_adds.remove(key) {
            return Some(ManagedVariableSource::PathlikeAdditions(adds));
        }
        if let Some(val) = self.overrides.remove(key) {
            return Some(ManagedVariableSource::Overrides(val));
        }
        None
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            env_vars: env::vars().collect(),
            vars: HashMap::new(),
            pathlike_adds: HashMap::new(),
            overrides: HashMap::new(),
        }
    }
}

fn current_env_vars() -> HashMap<String, String> {
    env::vars().collect()
}