use anyhow::{anyhow, Result};
use rspotify::{prelude::*, AuthCodeSpotify};
use serde::{Deserialize, Serialize};
use serde_yaml;

use std::{
  fs,
  io::Write,
  path::{Path, PathBuf},
};

const APP_CONFIG_DIR: &str = "spotify-cli";
const FILE_NAME: &str = "client.yml";

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfig {
  device_id: Option<String>,
}

impl DeviceConfig {
  pub fn new() -> DeviceConfig {
    DeviceConfig { device_id: None }
  }

  pub fn get_device_id(&self) -> Option<&String> {
    self.device_id.as_ref()
  }

  pub async fn load(&mut self, spotify: &AuthCodeSpotify) -> Result<()> {
    let path = self.get_or_build_paths()?;

    // If there is a file load from file
    if path.exists() {
      let config_string = fs::read_to_string(&path)?;
      let config_yml: DeviceConfig = serde_yaml::from_str(&config_string)?;

      self.device_id = config_yml.device_id;

      Ok(())
    } else {
      // Get from spotify api
      // @TODO more than one device is not handled
      let devices = spotify.device().await;

      if let Ok(devs) = devices {
        let first_device = &devs[0];
        let device_id = first_device.id.clone();

        self.device_id = device_id;

        let config_yml = DeviceConfig {
          device_id: self.device_id.clone(),
        };

        let content_yml = serde_yaml::to_string(&config_yml)?;

        let mut new_config = fs::File::create(&path)?;
        write!(new_config, "{}", content_yml)?;

        self.device_id = config_yml.device_id;

        Ok(())
      } else {
        Err(anyhow!("Failed to get device id"))
      }
    }
  }

  pub fn get_or_build_paths(&self) -> Result<PathBuf> {
    match home::home_dir() {
      Some(home) => {
        let path = Path::new(&home);
        let app_config_dir = path.join(APP_CONFIG_DIR);

        if !app_config_dir.exists() {
          fs::create_dir(&app_config_dir)?;
        }

        let config_file_name = &app_config_dir.join(FILE_NAME);

        Ok(config_file_name.to_path_buf())
      }
      None => Err(anyhow!("No $HOME directory found for client config")),
    }
  }
}
