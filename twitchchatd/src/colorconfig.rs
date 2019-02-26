use twitchchat::types::Color;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ColorConfig {
    map: HashMap<String, Color>,
}

const COLOR_CONFIG_NAME: &str = "twitchchat_colors.json";

impl ColorConfig {
    pub fn get(&self, id: &str) -> Option<&Color> {
        self.map.get(id)
    }

    pub fn set<S, C>(&mut self, id: S, color: C) -> Result<(), String>
    where
        S: ToString,
        C: Into<Color>,
    {
        self.map.insert(id.to_string(), color.into());
        self.save()
    }

    pub fn remove(&mut self, id: &str) -> Result<(), String> {
        self.map.remove(id);
        self.save()
    }
}

impl ColorConfig {
    pub fn load() -> Result<Self, String> {
        let dirs = directories::ProjectDirs::from("com.github", "museun", "twitchchat")
            .expect("system to have a valid $HOME directory");

        std::fs::create_dir_all(dirs.config_dir()).map_err(|err| err.to_string())?;
        let dir = dirs.config_dir().join(COLOR_CONFIG_NAME);

        let json = std::fs::read_to_string(dir).map_err(|err| err.to_string())?;
        Ok(serde_json::from_str(&json)
            .map_err(|err| err.to_string())
            .unwrap_or_default())
    }

    pub fn save(&self) -> Result<(), String> {
        let dirs = directories::ProjectDirs::from("com.github", "museun", "twitchchat")
            .expect("system to have a valid $HOME directory");

        std::fs::create_dir_all(dirs.config_dir()).map_err(|err| err.to_string())?;
        let dir = dirs.config_dir().join(COLOR_CONFIG_NAME);

        let mut fi = std::fs::File::create(dir).map_err(|err| err.to_string())?;
        serde_json::to_writer_pretty(&mut fi, &self).map_err(|err| err.to_string())
    }
}

impl Drop for ColorConfig {
    fn drop(&mut self) {
        let _ = self.save();
    }
}
