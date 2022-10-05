use crate::oauth;
use anyhow::{anyhow, Result};

use rspotify::{
  prelude::*, AuthCodeSpotify, Config, Credentials, DEFAULT_CACHE_PATH,
};
use std::path::PathBuf;

pub async fn get_spotify() -> AuthCodeSpotify {
  let creds = Credentials::from_env().unwrap();

  let oauth = oauth::get_oauth();

  let cache_path = get_cache_dir();

  let mut spotify = AuthCodeSpotify::with_config(
    creds,
    oauth,
    Config {
      token_refreshing: true,
      token_cached: true,
      cache_path,
      ..Config::default()
    },
  );

  let url = spotify.get_authorize_url(false).unwrap();

  spotify.prompt_for_token(&url).await.unwrap();

  spotify
}

// @TODO must be improved, store device id in yaml or json
// @TODO if there is more than one device, ask to choose
pub async fn get_device(spotify: &AuthCodeSpotify) -> Result<String> {
  let devices = spotify.device().await;

  if let Ok(devs) = devices {
    let first_device = &devs[0];

    Ok(first_device.id.as_ref().unwrap().clone())
  } else {
    Err(anyhow!("Failed to get device id"))
  }
}

pub fn get_cache_dir() -> PathBuf {
  let mut cache_path = PathBuf::new();

  match home::home_dir() {
    Some(path) => cache_path.push(path),
    None => println!("Impossible to get your home dir!"),
  }

  cache_path.push(DEFAULT_CACHE_PATH);

  cache_path
}
