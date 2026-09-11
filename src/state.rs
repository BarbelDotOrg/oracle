use crate::shell::Shell;
use anyhow::anyhow;
use cosmic::iced::core::svg::Data::Path;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    pub vars: HashMap<Shell, HashMap<String, String>>,
}

impl State {
    pub fn save(&self) -> anyhow::Result<()> {
        let cfg_path = dirs::config_dir()
            .ok_or(anyhow!("todo err message"))?
            .join("oracle")
            .join("state.ron");
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

    pub fn remove(&mut self, shell: Shell, key: &str) -> bool {
        let mut contains = false;
        if let Some(vars) = self.vars.get_mut(&shell) {
            contains = vars.contains_key(key);
            vars.remove(key);
        }
        contains
    }
}
