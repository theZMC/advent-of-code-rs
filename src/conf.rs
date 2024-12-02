use std::{error::Error, fs, path::PathBuf};

use confique::Config;
use serde::{Deserialize, Serialize};

#[derive(Config, Debug, Serialize, Deserialize)]
pub struct Conf {
    #[config(env = "TOKEN")]
    pub token: String,
}

impl Conf {
    pub fn load_or_create() -> Result<Self, Box<dyn Error>> {
        let config_path = Self::get_path()?;
        let mut builder = Self::builder().env();
        if config_path.exists() {
            builder = builder.file(config_path);
        }
        let conf = match builder.load() {
            Ok(c) => c,
            Err(_) => {
                let token = Self::prompt_for_token()?;
                let new_conf = Self { token };
                new_conf.save_to_file()?;
                return Ok(new_conf);
            }
        };

        Ok(conf)
    }

    fn get_path() -> Result<PathBuf, Box<dyn Error>> {
        let mut config_dir = dirs::config_dir().ok_or("Could not find config directory")?;
        config_dir.push("advent-of-code");
        fs::create_dir_all(&config_dir)?;
        config_dir.push("config.yaml");
        Ok(config_dir)
    }

    fn prompt_for_token() -> Result<String, Box<dyn Error>> {
        println!("Please enter your Advent of Code session token:");
        let mut token = String::new();
        std::io::stdin().read_line(&mut token)?;
        Ok(token.trim().to_string())
    }

    fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        let yaml = serde_yaml::to_string(self)?;
        let path = Self::get_path()?;
        let path_str = path.to_string_lossy().to_string();
        match fs::write(path, yaml) {
            Ok(_) => {
                println!("Config saved to file: {:?}", path_str);
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
