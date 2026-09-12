use crate::shell::Shell;
use anyhow::anyhow;
use clap::ValueEnum;
use cosmic::iced::core::svg::Data::Path;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::PathBuf;

#[cfg(not(windows))]
const DEFAULT_DELIM: char = ':';

#[cfg(windows)]
const DEFAULT_DELIM: char = ';';

#[derive(Serialize, Deserialize)]
pub struct State {
    pub vars: HashMap<Shell, HashMap<String, String>>,
}

impl State {
    pub fn save(&self) -> anyhow::Result<()> {
        let cfg_dir = dirs::config_dir()
            .ok_or(anyhow!("Couldnt find config dir (???)"))?
            .join("oracle");

        create_dir_all(&cfg_dir)?;

        let cfg_path = cfg_dir.join("state.ron");
        let mut file = File::create(cfg_path)?;
        let serialized = ron::ser::to_string_pretty(self, PrettyConfig::default())?;
        file.write(serialized.as_bytes())?;
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

    pub fn add(&mut self, shell: Shell, key: &str, value: &str) -> anyhow::Result<()> {
        // todo validate key
        self.vars
            .entry(shell)
            .or_default()
            .insert(key.to_string(), value.to_string());
        Ok(())
    }

    pub fn add_pathlike(
        &mut self,
        shell: Shell,
        key: &str,
        value: &str,
        delimitator: char,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn pathlike_parts(&self, shell: Shell, key: &str) -> Option<Vec<String>> {
        match self.vars.get(&shell).unwrap().get(key) {
            Some(val) => Some(
                val.split(DEFAULT_DELIM)
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>(),
            ),
            None => None,
        }
    }

    pub fn remove(&mut self, shell: Shell, key: &str) -> bool {
        let mut contains = false;
        if let Some(vars) = self.vars.get_mut(&shell) {
            contains = vars.contains_key(key);
            vars.remove(key);
        }
        contains
    }
}

impl Default for State {
    fn default() -> Self {
        let mut vars: HashMap<Shell, HashMap<String, String>> = HashMap::new();
        for shell in Shell::value_variants() {
            vars.insert(*shell, HashMap::new());
        }
        Self { vars }
    }
}
